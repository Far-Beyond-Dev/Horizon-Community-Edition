use serde::{Serialize, Deserialize};
use crate::celestial_bodies::celestial_body::CelestialBody;
use crate::celestial_bodies::planet::Planet;

pub trait Actor {
    fn position(&self) -> (f64, f64, f64);
    fn name(&self) -> &str;
}

#[derive(Serialize, Deserialize)]
pub enum ActorType {
    CelestialBody(CelestialBody),
    Planet(Planet),
}
