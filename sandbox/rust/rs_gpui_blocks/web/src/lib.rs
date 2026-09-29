// Share the desktop logic without pulling GPUI into the Wasm build.
#[allow(dead_code)]
#[path = "../../src/diagram_model.rs"]
mod diagram_model;
#[allow(dead_code)]
#[path = "../../src/diagram_router.rs"]
mod diagram_router;
#[path = "../../src/diagram_selection.rs"]
mod diagram_selection;
#[path = "../../src/diagram_validate.rs"]
mod diagram_validate;

use diagram_model::{Diagram, Port, Pos, WIDTH};
use diagram_router::{junctions, route_all, wire_distance};
use diagram_selection::{GroupDrag, Marquee, clicked_blocks};
use diagram_validate::validate_diagram;
use std::collections::BTreeSet;
use wasm_bindgen::prelude::*;

fn load(saved: &str) -> Result<Diagram, &'static str> {
    if saved.is_empty() {
        return Ok(Diagram::demo());
    }
    let diagram: Diagram = serde_json::from_str(saved).map_err(|_| "Invalid saved diagram JSON")?;
    validate_diagram(&diagram)?;
    Ok(diagram)
}

#[wasm_bindgen]
pub struct BrowserDiagram {
    diagram: Diagram,
    selected: BTreeSet<usize>,
    selected_port: Option<Port>,
    selected_wire: Option<usize>,
    marquee: Option<Marquee>,
    drag: Option<GroupDrag>,
}

#[wasm_bindgen]
impl BrowserDiagram {
    #[wasm_bindgen(constructor)]
    pub fn new(saved: &str) -> Result<BrowserDiagram, JsValue> {
        Ok(Self {
            diagram: load(saved).map_err(JsValue::from_str)?,
            selected: BTreeSet::new(),
            selected_port: None,
            selected_wire: None,
            marquee: None,
            drag: None,
        })
    }

    pub fn view(&self) -> String {
        let routes = route_all(&self.diagram);
        serde_json::json!({
            "diagram": self.diagram,
            "junctions": junctions(&self.diagram, &routes),
            "routes": routes,
            "selected": self.selected,
            "selected_port": self.selected_port,
            "selected_wire": self.selected_wire,
        })
        .to_string()
    }

    pub fn save(&self) -> String {
        serde_json::to_string(&self.diagram).expect("diagram is serializable")
    }

    pub fn select_block(&mut self, id: usize, shift: bool, x: f32, y: f32) {
        if !x.is_finite() || !y.is_finite() || !self.diagram.blocks.iter().any(|b| b.id == id) {
            return;
        }
        self.selected = clicked_blocks(&self.selected, id, shift);
        self.selected_port = None;
        self.selected_wire = None;
        self.drag = self
            .selected
            .contains(&id)
            .then(|| GroupDrag::new(Pos::new(x, y), &self.selected, &self.diagram));
    }

    pub fn drag_to(&mut self, x: f32, y: f32) -> bool {
        if !x.is_finite() || !y.is_finite() {
            return false;
        }
        let Some(drag) = &self.drag else { return false };
        let positions = drag.positions(Pos::new(x, y));
        let mut changed = false;
        for (id, pos) in positions {
            let block = self.diagram.block_mut(id);
            if block.pos != pos {
                block.pos = pos;
                changed = true;
            }
        }
        changed
    }

    pub fn end_drag(&mut self) {
        self.drag = None;
    }

    pub fn start_marquee(&mut self, x: f32, y: f32, shift: bool) {
        if !x.is_finite() || !y.is_finite() {
            return;
        }
        let base = if shift {
            self.selected.clone()
        } else {
            BTreeSet::new()
        };
        self.selected = base.clone();
        self.selected_port = None;
        self.selected_wire = None;
        self.marquee = Some(Marquee {
            start: Pos::new(x, y),
            end: Pos::new(x, y),
            base,
        });
    }

    pub fn marquee_to(&mut self, x: f32, y: f32) {
        if !x.is_finite() || !y.is_finite() {
            return;
        }
        if let Some(area) = &mut self.marquee {
            area.end = Pos::new(x, y);
            self.selected = area.selected(&self.diagram);
        }
    }

    pub fn end_marquee(&mut self) {
        self.marquee = None;
    }

    pub fn clear_selection(&mut self) {
        self.selected.clear();
        self.selected_port = None;
        self.selected_wire = None;
        self.marquee = None;
        self.drag = None;
    }

    pub fn select_port(&mut self, block: usize, index: usize, output: bool) {
        if let Some(port) = self.port(block, index, output) {
            self.selected.clear();
            self.selected_wire = None;
            self.selected_port = Some(port);
        }
    }

    pub fn delete_selected(&mut self) -> bool {
        if !self.selected.is_empty() {
            for id in std::mem::take(&mut self.selected) {
                self.diagram.delete_block(id);
            }
        } else if let Some(port) = self.selected_port.take() {
            let block = self.diagram.block_mut(port.block);
            let ports = if port.output {
                &mut block.outputs
            } else {
                &mut block.inputs
            };
            ports.remove(port.index);
            self.diagram
                .wires
                .retain(|wire| wire.from != port && wire.to != port);
            for wire in &mut self.diagram.wires {
                for endpoint in [&mut wire.from, &mut wire.to] {
                    if endpoint.block == port.block
                        && endpoint.output == port.output
                        && endpoint.index > port.index
                    {
                        endpoint.index -= 1;
                    }
                }
            }
            self.selected.insert(port.block);
        } else if let Some(id) = self.selected_wire.take() {
            self.diagram.wires.retain(|wire| wire.id != id);
        } else {
            return false;
        }
        self.drag = None;
        self.marquee = None;
        true
    }

    fn selected_block_id(&self) -> Option<usize> {
        if self.selected.len() == 1 {
            self.selected.first().copied()
        } else {
            self.selected_port.map(|port| port.block)
        }
    }

    pub fn add_port(&mut self, output: bool) -> bool {
        let Some(id) = self.selected_block_id() else {
            return false;
        };
        let block = self.diagram.block_mut(id);
        let ports = if output {
            &mut block.outputs
        } else {
            &mut block.inputs
        };
        if ports.len() >= 6 {
            return false;
        }
        ports.push(format!(
            "{}{}",
            if output { "out" } else { "in" },
            ports.len() + 1
        ));
        self.selected.clear();
        self.selected_port = Some(Port {
            block: id,
            index: ports.len() - 1,
            output,
        });
        true
    }

    pub fn rename_selected(&mut self, value: &str) -> bool {
        let value = value.trim();
        if value.is_empty() || value.chars().count() > 64 || value.chars().any(char::is_control) {
            return false;
        }
        if let Some(port) = self.selected_port {
            let block = self.diagram.block_mut(port.block);
            let ports = if port.output {
                &mut block.outputs
            } else {
                &mut block.inputs
            };
            ports[port.index] = value.to_owned();
        } else if let Some(id) = self.selected_block_id() {
            self.diagram.block_mut(id).name = value.to_owned();
        } else {
            return false;
        }
        true
    }

    pub fn select_wire(&mut self, x: f32, y: f32, tolerance: f32) -> bool {
        if !x.is_finite() || !y.is_finite() || !tolerance.is_finite() || tolerance <= 0. {
            return false;
        }
        let Some(route) = route_all(&self.diagram)
            .into_iter()
            .rev()
            .find(|r| wire_distance(Pos::new(x, y), &r.points) < tolerance)
        else {
            return false;
        };
        self.selected.clear();
        self.selected_port = None;
        self.selected_wire = Some(route.wire);
        true
    }

    fn port(&self, block: usize, index: usize, output: bool) -> Option<Port> {
        let b = self.diagram.blocks.iter().find(|b| b.id == block)?;
        (index
            < if output {
                b.outputs.len()
            } else {
                b.inputs.len()
            })
        .then_some(Port {
            block,
            index,
            output,
        })
    }

    pub fn can_connect(
        &self,
        source: usize,
        source_index: usize,
        target: usize,
        target_index: usize,
    ) -> bool {
        match (
            self.port(source, source_index, true),
            self.port(target, target_index, false),
        ) {
            (Some(a), Some(b)) => self.diagram.can_connect(a, b).is_ok(),
            _ => false,
        }
    }

    pub fn connect(
        &mut self,
        source: usize,
        source_index: usize,
        target: usize,
        target_index: usize,
    ) -> bool {
        if !self.can_connect(source, source_index, target, target_index) {
            return false;
        }
        self.diagram
            .connect(
                Port {
                    block: source,
                    index: source_index,
                    output: true,
                },
                Port {
                    block: target,
                    index: target_index,
                    output: false,
                },
            )
            .is_ok()
    }

    pub fn add_block(&mut self, x: f32, y: f32) -> bool {
        if !x.is_finite() || !y.is_finite() {
            return false;
        }
        let mut pos = Pos::new(x.clamp(0., 3300.), y.clamp(0., 3300.)).snap();
        for _ in 0..300 {
            let occupied = self.diagram.blocks.iter().any(|block| {
                pos.x < block.pos.x + WIDTH + 24.
                    && pos.x + WIDTH + 24. > block.pos.x
                    && pos.y < block.pos.y + block.height() + 24.
                    && pos.y + 96. > block.pos.y
            });
            if !occupied {
                break;
            }
            pos.y += 108.;
            if pos.y > 3300. {
                pos.y = 0.;
                pos.x = (pos.x + 240.).min(3600.);
            }
        }
        let id = self.diagram.add(pos);
        self.selected = BTreeSet::from([id]);
        self.selected_port = None;
        self.selected_wire = None;
        true
    }

    pub fn move_block(&mut self, id: usize, x: f32, y: f32) -> bool {
        if !x.is_finite() || !y.is_finite() {
            return false;
        }
        if let Some(block) = self.diagram.blocks.iter_mut().find(|b| b.id == id) {
            block.pos = Pos::new(x.clamp(0., 3600.), y.clamp(0., 3600.)).snap();
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browser_moves_connects_and_restores() {
        let mut app = BrowserDiagram::new("").unwrap();
        assert!(!app.can_connect(0, 0, 1, 0)); // Occupied input.
        assert!(app.add_block(1056., 696.));
        let block = app.diagram.blocks.last().unwrap().id;
        assert!(app.move_block(block, 1001., 200.));
        assert_eq!(app.diagram.block(block).pos, Pos::new(996., 204.));
        assert!(app.can_connect(0, 0, block, 0));
        assert!(app.connect(0, 0, block, 0));
        assert!(!app.connect(0, 0, block, 0));
        let restored = BrowserDiagram::new(&app.save()).unwrap();
        assert_eq!(restored.diagram.wires.len(), app.diagram.wires.len());
        assert_eq!(
            restored.diagram.block(block).pos,
            app.diagram.block(block).pos
        );
    }

    #[test]
    fn selection_marquee_group_drag_and_wire_hit() {
        let mut app = BrowserDiagram::new("").unwrap();
        app.select_block(0, false, 100., 150.);
        app.select_block(1, true, 100., 420.);
        assert_eq!(app.selected, BTreeSet::from([0, 1]));
        assert!(app.drag_to(112., 432.));
        app.end_drag();
        assert_eq!(app.diagram.block(0).pos, Pos::new(72., 132.));
        assert_eq!(app.diagram.block(1).pos, Pos::new(72., 408.));
        app.start_marquee(50., 100., false);
        app.marquee_to(265., 510.);
        assert_eq!(app.selected, BTreeSet::from([0, 1]));
        app.end_marquee();
        app.select_block(0, true, 100., 150.);
        assert_eq!(app.selected, BTreeSet::from([1]));
        app.start_marquee(300., 100., true);
        app.marquee_to(650., 400.);
        assert_eq!(app.selected, BTreeSet::from([1, 2]));
        app.end_marquee();
        let route = route_all(&app.diagram)
            .into_iter()
            .find(|r| r.points.len() > 1)
            .unwrap();
        let p = route.points[0];
        assert!(app.select_wire(p.x + 12., p.y, 7.));
        assert!(app.selected.is_empty());
        assert!(app.selected_wire.is_some());
    }

    #[test]
    fn deleting_selected_wire_frees_its_input_and_persists() {
        let mut app = BrowserDiagram::new("").unwrap();
        assert!(!app.delete_selected());
        let wire = &app.diagram.wires[0];
        let (id, from, to) = (wire.id, wire.from, wire.to);
        app.selected_wire = Some(id);
        assert!(app.delete_selected());
        assert!(app.selected_wire.is_none());
        assert!(!app.diagram.wires.iter().any(|wire| wire.id == id));
        assert_eq!(app.diagram.wires.len(), 6);
        assert!(app.diagram.can_connect(from, to).is_ok());
        let restored = BrowserDiagram::new(&app.save()).unwrap();
        assert_eq!(restored.diagram.wires.len(), 6);
        assert!(!app.delete_selected());
    }

    #[test]
    fn edits_ports_and_labels_and_cleans_up_connections() {
        let mut app = BrowserDiagram::new("").unwrap();
        app.select_block(2, false, 500., 300.);
        assert!(app.rename_selected("Controller"));
        assert!(!app.rename_selected("   "));
        assert!(app.add_port(false));
        assert!(app.rename_selected("feedback"));
        let block = app.diagram.block(2);
        assert_eq!(block.name, "Controller");
        assert_eq!(block.inputs.last().unwrap(), "feedback");
        app.select_port(2, 1, true);
        assert!(app.delete_selected());
        assert_eq!(app.diagram.block(2).outputs.len(), 2);
        assert!(!app.diagram.wires.iter().any(|w| w.to.block == 4));
        assert!(
            app.diagram
                .wires
                .iter()
                .any(|w| w.from.block == 2 && w.from.index == 1 && w.to.block == 5)
        );
        assert!(validate_diagram(&app.diagram).is_ok());
        let restored = BrowserDiagram::new(&app.save()).unwrap();
        assert_eq!(restored.diagram.block(2).inputs.last().unwrap(), "feedback");
        app.select_block(1, false, 100., 420.);
        assert!(app.delete_selected());
        assert!(app.diagram.blocks.iter().all(|b| b.id != 1));
        assert!(
            app.diagram
                .wires
                .iter()
                .all(|w| w.from.block != 1 && w.to.block != 1)
        );
    }

    #[test]
    fn new_blocks_and_groups_use_full_workspace() {
        let mut app = BrowserDiagram::new("").unwrap();
        assert!(app.add_block(2300., 2100.));
        let id = app.diagram.blocks.last().unwrap().id;
        assert_eq!(app.diagram.block(id).pos, Pos::new(2304., 2100.));
        app.select_block(id, false, 2340., 2130.);
        assert!(app.drag_to(2600., 2380.));
        assert!(app.diagram.block(id).pos.x > 2300.);
        assert!(BrowserDiagram::new(&app.save()).is_ok());
    }

    #[test]
    fn rejects_corrupt_saved_input() {
        let mut diagram = Diagram::demo();
        diagram.wires[0].to.index = 99;
        assert!(load(&serde_json::to_string(&diagram).unwrap()).is_err());
        assert!(load("not json").is_err());
    }
}
