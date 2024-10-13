// Purpose: This file defines the core components of the actor system.
//
// Contents:
// - Timer struct: Handles interval-based events for actors.
// - BaseActor struct: Provides common properties for all actors.
// - Actor trait: Defines the interface that all actors must implement.
//
// When to use:
// - Use Timer when you need to implement interval-based actions for an actor.
// - Use BaseActor as a foundation for creating specific actor types.
// - Implement the Actor trait for any entity in your game that needs to be
//   updated, rendered, or perform interval-based actions.
//
// Note: This system is designed to be flexible, allowing for both actors that
// use tick-based updates and those that don't, with minimal performance overhead.

pub trait Object {
    fn begin_play(&mut self);
    fn end_play(&mut self);
    fn tick(&mut self, delta_time: f32);
    
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: String);
    
    fn add_tag(&mut self, tag: String);
    fn has_tag(&self, tag: &str) -> bool;
    fn get_tags(&self) -> &[String];
}

pub struct BaseObject {
    name: String,
    tags: Vec<String>,
}

impl BaseObject {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tags: Vec::new(),
        }
    }
}

impl Object for BaseObject {
    fn begin_play(&mut self) {
        println!("Object '{}' began play", self.name);
    }

    fn end_play(&mut self) {
        println!("Object '{}' ended play", self.name);
    }

    fn tick(&mut self, _delta_time: f32) {
        // Base objects typically don't need updating
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag.to_string())
    }

    fn get_tags(&self) -> &[String] {
        &self.tags
    }
}