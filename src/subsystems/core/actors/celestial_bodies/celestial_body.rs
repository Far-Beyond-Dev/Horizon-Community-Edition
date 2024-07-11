use serde::{Serialize, Deserialize};
use crate::actor::Actor;

#[derive(Serialize, Deserialize)]
pub struct CelestialBody {
    pub name: String,
    pub position: (f64, f64, f64),
    pub mass: f64,
}

impl Actor for CelestialBody {
    fn position(&self) -> (f64, f64, f64) {
        self.position
    }

    fn name(&self) -> &str {
        &self.name
    }
}
