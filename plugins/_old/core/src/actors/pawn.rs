use super::actor::Actor;
use super::controller::Controller;

pub trait Pawn: Actor {
    fn get_controller(&self) -> Option<&dyn Controller>;
    fn set_controller(&mut self, controller: Option<Box<dyn Controller>>);
}

// Example implementation
struct GenericActor {
    base: BaseActor,
    transform: Transform,
}

impl GenericActor {
    pub fn new(position: (f32, f32, f32)) -> Self {
        Self {
            base: BaseActor::new(),
            transform: Transform::new(position, (0.0, 0.0, 0.0), (1.0, 1.0, 1.0)),
        }
    }
}

impl Actor for GenericActor {
    fn get_transform(&self) -> Transform {
        self.transform
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn begin_play(&mut self) {
        println!("Actor began play at location: {:?}", self.get_actor_location());
    }

    fn tick(&mut self, delta_time: f32) {
        // Basic update logic
    }

    fn end_play(&mut self) {
        println!("Actor ended play");
    }
}