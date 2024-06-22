
# Explanation of the actors subsystem

The actors subsystem is designed to provide a standardised structure for server-side objects in game space, making interacting with actors depenable and simple. This document explains how to create a new type of actor with several custom fields, and how to create a sub-actor of your new actor type.

## Folder Structure

First, let's look at the folder structure:

```
src/
├── subsystems/
│   └── actors/
│       ├── celestial_bodies/
│       │   ├── celestial_body.rs
│       │   └── planet.rs
│       ├── new_actor_types/
│       │   ├── new_actor.rs
│       │   └── sub_new_actor.rs
│       ├── actor.rs
│       └── main.rs
```

## Step-by-Step Guide

1. **Define the `Actor` Trait** (This is done for you in Horizon Server):

   The `Actor` trait is the base trait that all actors must implement. It defines common behavior for all actors.

   ```rust
   // src/subsystems/actors/actor.rs
   use serde::{Serialize, Deserialize};

   pub trait Actor {
       fn position(&self) -> (f64, f64, f64);
       fn name(&self) -> &str;
   }

   #[derive(Serialize, Deserialize)]
   pub enum ActorType {
       CelestialBody(super::celestial_bodies::celestial_body::CelestialBody),
       Planet(super::celestial_bodies::planet::Planet),
       NewActorType(super::new_actor_types::new_actor::NewActor),
       SubNewActorType(super::new_actor_types::sub_new_actor::SubNewActor),
   }
   ```

2. **Create a New Actor Type (Child of `Actor`)**:

   Define a new struct for the new actor type and implement the `Actor` trait for it.

   ```rust
   // src/subsystems/actors/new_actor_types/new_actor.rs
   use serde::{Serialize, Deserialize};
   use crate::subsystems::actors::actor::Actor;

   #[derive(Serialize, Deserialize)]
   pub struct NewActor {
       pub name: String,
       pub position: (f64, f64, f64),
   }

   impl Actor for NewActor {
       fn position(&self) -> (f64, f64, f64) {
           self.position
       }

       fn name(&self) -> &str {
           &self.name
       }
   }
   ```

3. **Create a Sub-Actor Type (Child of `NewActor`)**:

   Define a new struct for the sub-actor type which extends `NewActor` and implements the `Actor` trait.

   ```rust
   // src/subsystems/actors/new_actor_types/sub_new_actor.rs
   use serde::{Serialize, Deserialize};
   use crate::subsystems::actors::new_actor_types::new_actor::NewActor;
   use crate::subsystems::actors::actor::Actor;

   #[derive(Serialize, Deserialize)]
   pub struct SubNewActor {
       pub new_actor: NewActor,
       pub additional_field: String,
   }

   impl Actor for SubNewActor {
       fn position(&self) -> (f64, f64, f64) {
           self.new_actor.position
       }

       fn name(&self) -> &str {
           &self.new_actor.name
       }
   }
   ```

4. **Update `main.rs` to Include New Actor Types**:

   Import the new actor types into your `main.rs`.

   ```rust
   // src/subsystems/actors/main.rs
   mod actor;
   mod celestial_bodies {
       pub mod celestial_body;
       pub mod planet;
   }
   mod new_actor_types {
       pub mod new_actor;
       pub mod sub_new_actor;
   }

   use crate::subsystems::actors::actor::Actor;
   use crate::subsystems::actors::actor::ActorType;
   use crate::subsystems::actors::celestial_bodies::celestial_body::CelestialBody;
   use crate::subsystems::actors::celestial_bodies::planet::Planet;
   use crate::subsystems::actors::new_actor_types::new_actor::NewActor;
   use crate::subsystems::actors::new_actor_types::sub_new_actor::SubNewActor;

   fn main() {
       let earth = Planet {
           celestial_body: CelestialBody {
               name: String::from("Earth"),
               position: (0.0, 0.0, 0.0),
               mass: 5.972e24,
           },
           has_life: true,
       };

       let sun = CelestialBody {
           name: String::from("Sun"),
           position: (0.0, 0.0, 0.0),
           mass: 1.989e30,
       };

       let new_actor = NewActor {
           name: String::from("NewActor1"),
           position: (1.0, 2.0, 3.0),
       };

       let sub_new_actor = SubNewActor {
           new_actor: new_actor,
           additional_field: String::from("AdditionalField"),
       };

       let actors: Vec<ActorType> = vec![
           ActorType::Planet(earth),
           ActorType::CelestialBody(sun),
           ActorType::NewActorType(new_actor),
           ActorType::SubNewActorType(sub_new_actor),
       ];

       // Serialization
       let serialized = serde_json::to_vec(&actors).unwrap();
       println!("Serialized actors: {:?}", serialized);

       // Deserialization
       let deserialized: Vec<ActorType> = serde_json::from_slice(&serialized).unwrap();
       println!("Deserialized actors: {:?}", deserialized);
   }
   ```

## Proper Rust Terms and Corresponding C++ Concepts

- **Trait** (`Actor`): Equivalent to an interface or an abstract base class in C++. Defines a set of methods that types must implement.
- **Struct** (`CelestialBody`, `Planet`, `NewActor`, `SubNewActor`): Similar to classes in C++. They are used to define types with fields and methods.
- **Enum** (`ActorType`): Used to define a type that can be one of several variants, similar to a union in C++ but type-safe.
- **Module**: A way to organize code into separate namespaces/files, akin to namespaces and header files in C++.
