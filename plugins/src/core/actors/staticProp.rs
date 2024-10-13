// Purpose: Defines the StaticProp actor, which represents non-interactive
// elements in the game world.
//
// Contents:
// - StaticProp struct: Represents a static, non-interactive object in the game.
// - Implementation of the Actor trait for StaticProp.
//
// When to use:
// - Use this as a template for creating environmental objects, decorations,
//   or any non-interactive elements in your game.
// - This example demonstrates how to implement an actor that doesn't use
//   tick-based updates, showing how the system can be optimized for
//   simpler actor types.

use super::base::{Actor, BaseActor};

pub struct StaticProp {
    base: BaseActor,
}

impl StaticProp {
    pub fn new(position: (f32, f32)) -> Self {
        Self {
            base: BaseActor::new(position, 1.0),
        }
    }
}

impl Actor for StaticProp {
    fn init(&mut self) {
        println!("Static prop initialized at position: {:?}", self.base.position);
    }

    fn update(&mut self) {
        // Minimal or no update logic
    }

    fn render(&self) {
        // Render logic
    }
}