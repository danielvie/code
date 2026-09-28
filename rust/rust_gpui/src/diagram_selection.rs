//! Selection geometry uses diagram coordinates, independent of pan and zoom.
use crate::diagram_model::{Diagram, Pos, WIDTH};
use std::collections::BTreeSet;

pub fn clicked_blocks(current: &BTreeSet<usize>, id: usize, shift: bool) -> BTreeSet<usize> {
    let mut result = current.clone();
    if shift {
        if !result.remove(&id) {
            result.insert(id);
        }
    } else if !result.contains(&id) {
        result = BTreeSet::from([id]);
    }
    result
}

pub struct Marquee {
    pub start: Pos,
    pub end: Pos,
    pub base: BTreeSet<usize>,
}
impl Marquee {
    pub fn bounds(&self) -> (Pos, Pos) {
        (
            Pos::new(self.start.x.min(self.end.x), self.start.y.min(self.end.y)),
            Pos::new(self.start.x.max(self.end.x), self.start.y.max(self.end.y)),
        )
    }
    pub fn selected(&self, diagram: &Diagram) -> BTreeSet<usize> {
        let (min, max) = self.bounds();
        let mut ids = self.base.clone();
        for block in &diagram.blocks {
            if block.pos.x >= min.x
                && block.pos.y >= min.y
                && block.pos.x + WIDTH <= max.x
                && block.pos.y + block.height() <= max.y
            {
                ids.insert(block.id);
            }
        }
        ids
    }
}

pub struct GroupDrag {
    start: Pos,
    originals: Vec<(usize, Pos)>,
}
impl GroupDrag {
    pub fn new(start: Pos, ids: &BTreeSet<usize>, diagram: &Diagram) -> Self {
        Self {
            start,
            originals: ids.iter().map(|&id| (id, diagram.block(id).pos)).collect(),
        }
    }
    pub fn positions(&self, pointer: Pos) -> Vec<(usize, Pos)> {
        if self.originals.is_empty() {
            return vec![];
        }
        let min_x = self
            .originals
            .iter()
            .map(|(_, p)| p.x)
            .fold(f32::INFINITY, f32::min);
        let max_x = self
            .originals
            .iter()
            .map(|(_, p)| p.x)
            .fold(f32::NEG_INFINITY, f32::max);
        let min_y = self
            .originals
            .iter()
            .map(|(_, p)| p.y)
            .fold(f32::INFINITY, f32::min);
        let max_y = self
            .originals
            .iter()
            .map(|(_, p)| p.y)
            .fold(f32::NEG_INFINITY, f32::max);
        // Snap and clamp a single delta, never each block separately. This preserves spacing.
        let delta = Pos::new(pointer.x - self.start.x, pointer.y - self.start.y).snap();
        let dx = delta.x.clamp(-min_x, 3600. - max_x);
        let dy = delta.y.clamp(-min_y, 3600. - max_y);
        self.originals
            .iter()
            .map(|&(id, p)| (id, Pos::new(p.x + dx, p.y + dy)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shift_toggles_and_plain_click_preserves_a_selected_group() {
        let ids = BTreeSet::from([0, 1]);
        assert_eq!(clicked_blocks(&ids, 2, true), BTreeSet::from([0, 1, 2]));
        assert_eq!(clicked_blocks(&ids, 0, true), BTreeSet::from([1]));
        assert_eq!(clicked_blocks(&ids, 0, false), ids);
        assert_eq!(clicked_blocks(&ids, 2, false), BTreeSet::from([2]));
        assert!(clicked_blocks(&BTreeSet::from([0]), 0, true).is_empty());
    }
    #[test]
    fn marquee_requires_full_containment_in_either_drag_direction() {
        let d = Diagram::demo();
        let a = Marquee {
            start: Pos::new(60., 120.),
            end: Pos::new(252., 216.),
            base: BTreeSet::new(),
        };
        assert_eq!(a.selected(&d), BTreeSet::from([0]));
        let reversed = Marquee {
            start: a.end,
            end: a.start,
            base: BTreeSet::new(),
        };
        assert_eq!(reversed.selected(&d), a.selected(&d));
        let partial = Marquee {
            start: a.start,
            end: Pos::new(251., 216.),
            base: BTreeSet::new(),
        };
        assert!(partial.selected(&d).is_empty());
    }
    #[test]
    fn shift_marquee_adds_to_base_and_shrinking_drops_preview_hits() {
        let d = Diagram::demo();
        let mut area = Marquee {
            start: Pos::new(0., 0.),
            end: Pos::new(300., 600.),
            base: BTreeSet::from([2]),
        };
        assert_eq!(area.selected(&d), BTreeSet::from([0, 1, 2]));
        area.end = Pos::new(12., 12.);
        assert_eq!(area.selected(&d), BTreeSet::from([2]));
    }
    #[test]
    fn group_uses_one_snapped_delta_and_does_not_accumulate_drift() {
        let d = Diagram::demo();
        let drag = GroupDrag::new(Pos::new(100., 100.), &BTreeSet::from([0, 1]), &d);
        let positions = drag.positions(Pos::new(125., 113.));
        assert_eq!(
            positions,
            vec![(0, Pos::new(84., 132.)), (1, Pos::new(84., 408.))]
        );
        assert_eq!(
            drag.positions(Pos::new(100., 100.)),
            vec![(0, d.block(0).pos), (1, d.block(1).pos)]
        );
    }
    #[test]
    fn world_limits_clamp_the_group_without_collapsing_spacing() {
        let d = Diagram::demo();
        let drag = GroupDrag::new(Pos::default(), &BTreeSet::from([0, 1]), &d);
        assert_eq!(
            drag.positions(Pos::new(-1000., -1000.)),
            vec![(0, Pos::new(0., 0.)), (1, Pos::new(0., 276.))]
        );
        assert_eq!(
            drag.positions(Pos::new(9000., 9000.)),
            vec![(0, Pos::new(3600., 3324.)), (1, Pos::new(3600., 3600.))]
        );
    }
}
