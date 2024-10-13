// Purpose: Defines the Player actor, which represents the user-controlled
// character in the game.
//
// Contents:
// - Player struct: Represents a player in the game.
// - Implementation of the Actor trait for Player.
//
// When to use:
// - Use this as a template for creating the main player character.
// - Extend or modify this struct to add player-specific functionality.
// - This example demonstrates how to implement an actor that uses tick-based updates.


///////////// Imports /////////////
//  Import a few things we need  //
//  to create this type          //
///////////////////////////////////

use crate::actor::{Actor, Transform};
use crate::character::Character;
use crate::pawn::Pawn;
use crate::base::BaseActor;

struct PlayerCharacter {
    base: BaseActor,
    transform: Transform,
    health: f32,
}

impl PlayerCharacter {
    pub fn new(position: (f32, f32, f32)) -> Self {
        Self {
            base: BaseActor::new(),
            transform: Transform::new(position, (0.0, 0.0, 0.0), (1.0, 1.0, 1.0)),
            health: 100.0,
        }
    }
}

impl Actor for PlayerCharacter {
    fn get_transform(&self) -> Transform {
        self.transform
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn begin_play(&mut self) {
        println!("PlayerCharacter began play at location: {:?}", self.get_actor_location());
    }

    fn tick(&mut self, delta_time: f32) {
        // PlayerCharacter update logic
    }

    fn end_play(&mut self) {
        println!("PlayerCharacter ended play");
    }
}

// Implement Pawn and Character traits as needed