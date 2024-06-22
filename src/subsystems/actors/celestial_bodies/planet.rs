use serde::{Serialize, Deserialize};
use crate::celestial_bodies::celestial_body::CelestialBody;
use crate::actor::Actor;

#[derive(Serialize, Deserialize)]
pub struct Planet {
    pub celestial_body: CelestialBody,
    pub has_life: bool,
}

impl Actor for Planet {
    fn position(&self) -> (f64, f64, f64) {
        self.celestial_body.position
    }

    fn name(&self) -> &str {
        &self.celestial_body.name
    }
}