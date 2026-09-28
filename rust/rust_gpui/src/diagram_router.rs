//! Grid A*: block clearance is mandatory; bends and wire congestion cost extra.
//! Prototype 1 deliberately reports unroutable edges instead of drawing through blocks.
use crate::diagram_model::{Diagram, GRID, Pos, WIDTH};
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Clone)]
pub struct Route {
    pub wire: usize,
    pub color: u32,
    pub points: Vec<Pos>,
}

#[derive(Clone, Debug)]
pub struct Junction {
    pub position: Pos,
    pub color: u32,
}

/// Only a shared path from the same output is a branch. Geometric crossings,
/// including crossings after two branches have separated, are not junctions.
pub fn junctions(d: &Diagram, routes: &[Route]) -> Vec<Junction> {
    let mut dots: Vec<Junction> = Vec::new();
    for (i, a) in routes.iter().enumerate() {
        let Some(source) = d.wires.iter().find(|w| w.id == a.wire).map(|w| w.from) else {
            continue;
        };
        for b in &routes[i + 1..] {
            if !d.wires.iter().any(|w| w.id == b.wire && w.from == source) {
                continue;
            }
            if let Some(position) = first_split(&a.points, &b.points)
                && !dots
                    .iter()
                    .any(|dot| dot.position == position && dot.color == a.color)
            {
                dots.push(Junction {
                    position,
                    color: a.color,
                });
            }
        }
    }
    dots
}

// Walk segment ends together: one route may bend in the middle of the other's
// simplified straight segment. Matching vertices alone would miss that split.
fn first_split(a: &[Pos], b: &[Pos]) -> Option<Pos> {
    let mut shared = *a.first()?;
    if b.first()? != &shared {
        return None;
    }
    let (mut i, mut j) = (1, 1);
    loop {
        while a.get(i) == Some(&shared) {
            i += 1;
        }
        while b.get(j) == Some(&shared) {
            j += 1;
        }
        let (next_a, next_b) = (*a.get(i)?, *b.get(j)?);
        let length = |p: Pos| (p.x - shared.x).abs() + (p.y - shared.y).abs();
        let (len_a, len_b) = (length(next_a), length(next_b));
        let dir_a = Pos::new((next_a.x - shared.x) / len_a, (next_a.y - shared.y) / len_a);
        let dir_b = Pos::new((next_b.x - shared.x) / len_b, (next_b.y - shared.y) / len_b);
        if dir_a != dir_b {
            return (shared != a[0]).then_some(shared);
        }
        let distance = len_a.min(len_b);
        shared = Pos::new(shared.x + dir_a.x * distance, shared.y + dir_a.y * distance);
    }
}

pub fn route_all(d: &Diagram) -> Vec<Route> {
    let max_x = d
        .blocks
        .iter()
        .map(|b| b.pos.x + WIDTH + 96.)
        .fold(1200., f32::max);
    let max_y = d
        .blocks
        .iter()
        .map(|b| b.pos.y + b.height() + 96.)
        .fold(800., f32::max);
    // Allow routing above and to the left of blocks at the world origin.
    let origin = Pos::new(-96., -96.);
    let cols = ((max_x - origin.x) / GRID).ceil() as usize + 1;
    let rows = ((max_y - origin.y) / GRID).ceil() as usize + 1;
    let point = |i: usize| {
        Pos::new(
            origin.x + (i % cols) as f32 * GRID,
            origin.y + (i / cols) as f32 * GRID,
        )
    };
    let cell = |p: Pos| {
        ((p.y - origin.y) / GRID).round() as usize * cols
            + ((p.x - origin.x) / GRID).round() as usize
    };
    let mut blocked = vec![false; cols * rows];
    // All block positions and ports lie on the routing grid.
    for b in &d.blocks {
        let lo = cell(Pos::new(b.pos.x - GRID, b.pos.y - GRID));
        let hi = cell(Pos::new(
            b.pos.x + WIDTH + GRID,
            b.pos.y + b.height() + GRID,
        ));
        for y in lo / cols..=hi / cols {
            for x in lo % cols..=hi % cols {
                blocked[y * cols + x] = true;
            }
        }
    }
    let mut traffic = vec![[0u32; 2]; cols * rows];
    d.wires
        .iter()
        .map(|w| {
            let a = d.port_pos(w.from);
            let b = d.port_pos(w.to);
            let start = Pos::new(a.x + GRID * 2., a.y);
            let end = Pos::new(b.x - GRID * 2., b.y);
            // Check the short escape stubs against every other block too.
            let stub_clear = |p: Pos, q: Pos, owner: usize| {
                d.blocks.iter().filter(|b| b.id != owner).all(|b| {
                    !(p.x.min(q.x) <= b.pos.x + WIDTH + GRID
                        && p.x.max(q.x) >= b.pos.x - GRID
                        && p.y >= b.pos.y - GRID
                        && p.y <= b.pos.y + b.height() + GRID)
                })
            };
            let mut points = Vec::new();
            if stub_clear(a, start, w.from.block) && stub_clear(b, end, w.to.block) {
                let source = cell(start);
                let target = cell(end);
                if let Some(cells) = search(source, target, cols, rows, &blocked, &traffic) {
                    points.push(a);
                    points.extend(cells.iter().map(|&i| point(i)));
                    points.push(b);
                    for pair in cells.windows(2) {
                        let dir = usize::from(pair[0] / cols != pair[1] / cols);
                        traffic[pair[0]][dir] += 1;
                        traffic[pair[1]][dir] += 1;
                    }
                    // Remove collinear points without removing reversals.
                    let mut simplified: Vec<Pos> = Vec::new();
                    for p in points {
                        while simplified.len() >= 2 {
                            let u = simplified[simplified.len() - 2];
                            let v = simplified[simplified.len() - 1];
                            let straight = (u.x == v.x
                                && v.x == p.x
                                && (v.y - u.y) * (p.y - v.y) >= 0.)
                                || (u.y == v.y && v.y == p.y && (v.x - u.x) * (p.x - v.x) >= 0.);
                            if !straight {
                                break;
                            }
                            simplified.pop();
                        }
                        simplified.push(p);
                    }
                    points = simplified;
                }
            }
            Route {
                wire: w.id,
                color: w.color,
                points,
            }
        })
        .collect()
}

fn search(
    source: usize,
    target: usize,
    cols: usize,
    rows: usize,
    blocked: &[bool],
    traffic: &[[u32; 2]],
) -> Option<Vec<usize>> {
    if blocked[source] || blocked[target] {
        return None;
    }
    // State includes arrival axis so bends can be penalized correctly.
    let mut cost = vec![u32::MAX; cols * rows * 2];
    let mut parent = vec![usize::MAX; cost.len()];
    let heuristic = |i: usize| {
        ((i % cols).abs_diff(target % cols) + (i / cols).abs_diff(target / cols)) as u32 * 10
    };
    let mut heap = BinaryHeap::new();
    cost[source * 2] = 0;
    heap.push(Reverse((heuristic(source), 0u32, source * 2)));
    while let Some(Reverse((_, g, state))) = heap.pop() {
        if g != cost[state] {
            continue;
        }
        let i = state / 2;
        let dir = state % 2;
        if i == target {
            let mut path = vec![i];
            let mut cur = state;
            while parent[cur] != usize::MAX {
                cur = parent[cur];
                path.push(cur / 2);
            }
            path.reverse();
            return Some(path);
        }
        let x = i % cols;
        let y = i / cols;
        for (nx, ny, axis) in [
            (x.wrapping_sub(1), y, 0),
            (x + 1, y, 0),
            (x, y.wrapping_sub(1), 1),
            (x, y + 1, 1),
        ] {
            if nx >= cols || ny >= rows {
                continue;
            }
            let n = ny * cols + nx;
            if blocked[n] {
                continue;
            }
            let next = n * 2 + axis;
            let step = 10
                + if axis != dir { 24 } else { 0 }
                + traffic[n][axis] * 48
                + traffic[n][1 - axis] * 18;
            let ng = g + step;
            if ng < cost[next] {
                cost[next] = ng;
                parent[next] = state;
                heap.push(Reverse((ng + heuristic(n), ng, next)));
            }
        }
    }
    None
}

pub fn wire_distance(p: Pos, points: &[Pos]) -> f32 {
    points
        .windows(2)
        .map(|s| {
            let x = p.x.clamp(s[0].x.min(s[1].x), s[0].x.max(s[1].x));
            let y = p.y.clamp(s[0].y.min(s[1].y), s[0].y.max(s[1].y));
            ((p.x - x).powi(2) + (p.y - y).powi(2)).sqrt()
        })
        .fold(f32::INFINITY, f32::min)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagram_model::{Port, Wire};

    fn route(wire: usize, points: &[(f32, f32)]) -> Route {
        Route {
            wire,
            color: 0x315bd6,
            points: points.iter().map(|&(x, y)| Pos::new(x, y)).collect(),
        }
    }

    fn diagram(sources: &[usize]) -> Diagram {
        Diagram {
            blocks: vec![],
            next_id: sources.len(),
            wires: sources
                .iter()
                .enumerate()
                .map(|(id, &index)| Wire {
                    id,
                    from: Port {
                        block: 0,
                        index,
                        output: true,
                    },
                    to: Port {
                        block: id + 1,
                        index: 0,
                        output: false,
                    },
                    color: 0x315bd6,
                })
                .collect(),
        }
    }

    #[test]
    fn marks_split_inside_simplified_segment() {
        let routes = [
            route(0, &[(0., 0.), (24., 0.), (24., 48.)]),
            route(1, &[(0., 0.), (72., 0.)]),
        ];
        let dots = junctions(&diagram(&[0, 0]), &routes);
        assert_eq!(dots.len(), 1);
        assert_eq!(dots[0].position, Pos::new(24., 0.));
        assert_eq!(dots[0].color, routes[0].color);
    }

    #[test]
    fn deduplicates_multiway_split() {
        let routes = [
            route(0, &[(0., 0.), (24., 0.), (24., 48.)]),
            route(1, &[(0., 0.), (72., 0.)]),
            route(2, &[(0., 0.), (24., 0.), (24., -48.)]),
        ];
        assert_eq!(junctions(&diagram(&[0, 0, 0]), &routes).len(), 1);
    }

    #[test]
    fn marks_later_split_of_a_shared_branch() {
        let routes = [
            route(0, &[(0., 0.), (24., 0.), (24., 48.)]),
            route(1, &[(0., 0.), (48., 0.), (48., 48.)]),
            route(2, &[(0., 0.), (72., 0.)]),
        ];
        let dots = junctions(&diagram(&[0, 0, 0]), &routes);
        assert_eq!(dots.len(), 2);
        assert!(dots.iter().any(|d| d.position == Pos::new(24., 0.)));
        assert!(dots.iter().any(|d| d.position == Pos::new(48., 0.)));
    }

    #[test]
    fn same_color_different_outputs_are_not_connected() {
        let routes = [
            route(0, &[(0., 0.), (24., 0.), (24., 48.)]),
            route(1, &[(0., 0.), (72., 0.)]),
        ];
        assert!(junctions(&diagram(&[0, 1]), &routes).is_empty());
    }

    #[test]
    fn crossing_after_split_does_not_add_a_dot() {
        let routes = [
            route(0, &[(0., 0.), (48., 0.), (48., 72.)]),
            route(1, &[(0., 0.), (24., 0.), (24., 24.), (72., 24.)]),
        ];
        let dots = junctions(&diagram(&[0, 0]), &routes);
        assert_eq!(dots.len(), 1);
        assert_eq!(dots[0].position, Pos::new(24., 0.));
    }

    #[test]
    fn single_identical_and_failed_routes_have_no_split() {
        let a = route(0, &[(0., 0.), (24., 0.), (24., 48.)]);
        assert!(junctions(&diagram(&[0]), std::slice::from_ref(&a)).is_empty());
        let mut b = a.clone();
        b.wire = 1;
        assert!(junctions(&diagram(&[0, 0]), &[a.clone(), b]).is_empty());
        assert!(junctions(&diagram(&[0, 0]), &[a, route(1, &[])]).is_empty());
    }
}
