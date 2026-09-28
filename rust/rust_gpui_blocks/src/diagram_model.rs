//! Diagram model for prototype 1. Not a SysML metamodel.
use serde::{Deserialize, Serialize};

pub const GRID: f32 = 12.0;
pub const WIDTH: f32 = 192.0;
pub const COLORS: [u32; 6] = [0x315bd6, 0x128275, 0xc97b19, 0xa34bae, 0xd14b53, 0x62768c];

#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}
impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn snap(self) -> Self {
        Self::new(
            (self.x / GRID).round() * GRID,
            (self.y / GRID).round() * GRID,
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: usize,
    pub name: String,
    pub kind: String,
    pub pos: Pos,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}
impl Block {
    pub fn height(&self) -> f32 {
        72.0 + self.inputs.len().max(self.outputs.len()).saturating_sub(1) as f32 * 24.0
    }
    pub fn contains(&self, p: Pos, padding: f32) -> bool {
        p.x >= self.pos.x - padding
            && p.x <= self.pos.x + WIDTH + padding
            && p.y >= self.pos.y - padding
            && p.y <= self.pos.y + self.height() + padding
    }
    pub fn port(&self, output: bool, index: usize) -> Pos {
        Pos::new(
            self.pos.x + if output { WIDTH } else { 0.0 },
            self.pos.y + 60.0 + index as f32 * 24.0,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Port {
    pub block: usize,
    pub index: usize,
    pub output: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Wire {
    pub id: usize,
    pub from: Port,
    pub to: Port,
    pub color: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Diagram {
    pub blocks: Vec<Block>,
    pub wires: Vec<Wire>,
    pub next_id: usize,
}
impl Diagram {
    pub fn block(&self, id: usize) -> &Block {
        self.blocks
            .iter()
            .find(|b| b.id == id)
            .expect("live block id")
    }
    pub fn block_mut(&mut self, id: usize) -> &mut Block {
        self.blocks
            .iter_mut()
            .find(|b| b.id == id)
            .expect("live block id")
    }
    pub fn port_pos(&self, p: Port) -> Pos {
        self.block(p.block).port(p.output, p.index)
    }
    pub fn port_name(&self, p: Port) -> &str {
        let b = self.block(p.block);
        if p.output {
            &b.outputs[p.index]
        } else {
            &b.inputs[p.index]
        }
    }
    pub fn can_connect(&self, a: Port, b: Port) -> Result<(), &'static str> {
        if !a.output || b.output {
            return Err("Connect an output on the right to an input on the left.");
        }
        if a.block == b.block {
            return Err("Self connections are not supported in this PoC.");
        }
        if self.wires.iter().any(|w| w.to == b) {
            return Err("This input is occupied. Select its wire and delete it first.");
        }
        Ok(())
    }
    pub fn connect(&mut self, a: Port, b: Port) -> Result<(), &'static str> {
        self.can_connect(a, b)?;
        self.wires.push(Wire {
            id: self.next_id,
            from: a,
            to: b,
            color: COLORS[a.block % COLORS.len()],
        });
        self.next_id += 1;
        Ok(())
    }
    pub fn add(&mut self, pos: Pos) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.blocks.push(Block {
            id,
            name: format!("Block {id}"),
            kind: "Subsystem".into(),
            pos: pos.snap(),
            inputs: vec!["in".into()],
            outputs: vec!["out".into()],
        });
        id
    }
    pub fn delete_block(&mut self, id: usize) {
        self.blocks.retain(|b| b.id != id);
        self.wires
            .retain(|w| w.from.block != id && w.to.block != id);
    }
    pub fn demo() -> Self {
        let data = [
            (
                "Power supply",
                "Electrical",
                60.,
                120.,
                vec![],
                vec!["dc", "status"],
            ),
            (
                "Sensor system",
                "Acquisition",
                60.,
                396.,
                vec!["power"],
                vec!["temperature", "humidity"],
            ),
            (
                "Control system",
                "Controller",
                432.,
                264.,
                vec!["power", "temperature", "humidity"],
                vec!["heat", "cool", "air"],
            ),
            (
                "Heater system",
                "Actuator",
                816.,
                72.,
                vec!["command"],
                vec!["status"],
            ),
            (
                "Cooler system",
                "Actuator",
                816.,
                312.,
                vec!["command"],
                vec!["status"],
            ),
            (
                "Air transfer",
                "Actuator",
                816.,
                552.,
                vec!["command"],
                vec!["flow"],
            ),
        ];
        let blocks = data
            .into_iter()
            .enumerate()
            .map(|(id, (name, kind, x, y, ins, outs))| Block {
                id,
                name: name.into(),
                kind: kind.into(),
                pos: Pos::new(x, y),
                inputs: ins.into_iter().map(str::to_owned).collect(),
                outputs: outs.into_iter().map(str::to_owned).collect(),
            })
            .collect();
        let mut d = Self {
            blocks,
            wires: vec![],
            next_id: 6,
        };
        for (a, ai, b, bi) in [
            (0, 0, 1, 0),
            (0, 0, 2, 0),
            (1, 0, 2, 1),
            (1, 1, 2, 2),
            (2, 0, 3, 0),
            (2, 1, 4, 0),
            (2, 2, 5, 0),
        ] {
            d.connect(
                Port {
                    block: a,
                    index: ai,
                    output: true,
                },
                Port {
                    block: b,
                    index: bi,
                    output: false,
                },
            )
            .unwrap();
        }
        d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_targets_follow_connection_rules() {
        let mut d = Diagram::demo();
        let from = Port {
            block: 0,
            index: 0,
            output: true,
        };
        let occupied = Port {
            block: 3,
            output: false,
            ..from
        };
        assert!(d.can_connect(from, occupied).is_err());
        let target = Port {
            block: d.add(Pos::new(1200., 120.)),
            output: false,
            ..from
        };
        assert!(d.can_connect(from, target).is_ok());
        d.connect(from, target).unwrap();
        assert!(d.can_connect(from, target).is_err());
        assert!(
            d.can_connect(
                from,
                Port {
                    output: false,
                    ..from
                }
            )
            .is_err()
        );
        assert!(d.can_connect(from, from).is_err());
    }
}
