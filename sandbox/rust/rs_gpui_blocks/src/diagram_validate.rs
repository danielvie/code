//! Validate persisted diagrams before routing or editing them.
use crate::diagram_model::{COLORS, Diagram, GRID};
use std::collections::HashSet;

pub fn validate_diagram(diagram: &Diagram) -> Result<(), &'static str> {
    let mut ids = HashSet::new();
    for b in &diagram.blocks {
        if !ids.insert(b.id) {
            return Err("duplicate id");
        }
        if b.inputs.len() > 6 || b.outputs.len() > 6 {
            return Err("too many ports");
        }
        for label in std::iter::once(&b.name).chain(&b.inputs).chain(&b.outputs) {
            if label.trim().is_empty()
                || label.chars().count() > 64
                || label.chars().any(char::is_control)
            {
                return Err("invalid label");
            }
        }
        for v in [b.pos.x, b.pos.y] {
            if !v.is_finite()
                || !(0.0..=3600.0).contains(&v)
                || (v / GRID - (v / GRID).round()).abs() > 0.001
            {
                return Err("invalid block position");
            }
        }
    }
    let mut occupied = HashSet::new();
    for w in &diagram.wires {
        if !ids.insert(w.id) {
            return Err("duplicate id");
        }
        let from = diagram
            .blocks
            .iter()
            .find(|b| b.id == w.from.block)
            .ok_or("missing source block")?;
        let to = diagram
            .blocks
            .iter()
            .find(|b| b.id == w.to.block)
            .ok_or("missing destination block")?;
        if !w.from.output
            || w.to.output
            || from.id == to.id
            || w.from.index >= from.outputs.len()
            || w.to.index >= to.inputs.len()
        {
            return Err("invalid connection endpoints");
        }
        if !occupied.insert((w.to.block, w.to.index)) {
            return Err("multiple connections to one input");
        }
        if w.color != COLORS[from.id % COLORS.len()] {
            return Err("invalid connection color");
        }
    }
    if ids.iter().any(|&id| id >= diagram.next_id) || diagram.next_id == usize::MAX {
        return Err("invalid next id");
    }
    Ok(())
}
