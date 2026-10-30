# ECS example in Rust

This project implements a small entity component system using only Rust's standard library. It is a learning example, with a player that moves and a tree that stays still.

## What is ECS?

ECS means **Entity Component System**. It's a way to organize a program by separating identity, data, and the logic that acts on that data.

It's common in games, so let's use our player and tree as examples.

### 1. Entity: an identity

An entity is just an ID.

```rust
let player: Entity = 0;
let tree: Entity = 1;
```

The ID itself doesn't contain a position or know how to move. It lets us associate different pieces of data with the same thing.

### 2. Component: data about an entity

A component describes one aspect of an entity.

For example:

```rust
Position { x: 0.0, y: 0.0 }
Velocity { x: 2.0, y: 1.0 }
```

We give entities the components they need:

| Entity | Position | Velocity |
|---|---|---|
| Player, ID `0` | `0, 0` | `2, 1` |
| Tree, ID `1` | `5, 5` | None |

The components contain data. They don't perform the movement themselves.

In this example, `Position` and `Velocity` are stored in separate maps, keyed by entity ID. The `World` struct holds those component collections.

### 3. System: logic that uses components

A system performs some operation on entities with the required components.

Our movement system follows this rule:

> For every entity that has both a position and a velocity, update its position.

Conceptually:

```text
new position = old position + velocity × elapsed time
```

After one second, the player moves to `2, 1`. The tree stays at `5, 5` because it has no velocity component.

The system doesn't need to know which entity is a player or a tree. **It cares about the components, not the kind of object.**

## Why organize code this way?

You build behavior by combining components rather than creating a different type for every combination.

A player, an arrow, and a falling rock could all use the same movement system. Give each a position and velocity, and that system can move all of them.

That's the core mental model:

```text
Entity    → Which thing?
Component → What data does it have?
System    → What happens to that data?
```

## Run the example

```sh
cargo run
cargo test
```

Read `src/main.rs` from top to bottom to follow the example. Each movement update represents one second.

Entity IDs are assigned by hand to keep the example small; a larger ECS would need to manage IDs and entity removal.

## Check your understanding

If we gave our tree a velocity component, what would the existing movement system do?
