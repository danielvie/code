use std::collections::HashMap;

// An entity is only an ID. Its data lives in separate component collections.
type Entity = u32;

#[derive(Debug, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Default)]
struct World {
    positions: HashMap<Entity, Position>,
    velocities: HashMap<Entity, Velocity>,
}

// A system operates on entities that have the components it needs.
fn movement_system(world: &mut World, seconds: f32) {
    for (entity, velocity) in &world.velocities {
        if let Some(position) = world.positions.get_mut(entity) {
            position.x += velocity.x * seconds;
            position.y += velocity.y * seconds;
        }
    }
}

fn main() {
    let mut world = World::default();
    let player: Entity = 0;
    let tree: Entity = 1;

    world.positions.insert(player, Position { x: 0.0, y: 0.0 });
    world.velocities.insert(player, Velocity { x: 2.0, y: 1.0 });
    world.positions.insert(tree, Position { x: 5.0, y: 5.0 });

    for step in 1..=3 {
        movement_system(&mut world, 1.0);
        println!("After {step} second(s):");
        println!("  player: {:?}", world.positions[&player]);
        println!("  tree:   {:?}", world.positions[&tree]);
    }
}