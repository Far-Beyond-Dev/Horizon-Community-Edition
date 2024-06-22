mod actor;
mod celestial_bodies {
    pub mod celestial_body;
    pub mod planet;
}

use crate::actor::Actor;
use crate::actor::ActorType;
use crate::celestial_bodies::celestial_body::CelestialBody;
use crate::celestial_bodies::planet::Planet;

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

    let actors: Vec<ActorType> = vec![
        ActorType::Planet(earth),
        ActorType::CelestialBody(sun),
    ];

    // Serialization
    let serialized = serde_json::to_vec(&actors).unwrap();
    println!("Serialized actors: {:?}", serialized);

    // Deserialization
    let deserialized: Vec<ActorType> = serde_json::from_slice(&serialized).unwrap();
    println!("Deserialized actors: {:?}", deserialized);
}
