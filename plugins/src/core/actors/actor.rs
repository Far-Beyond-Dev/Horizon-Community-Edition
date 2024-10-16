use super::component::{Component, ComponentContainer};
use super::object::Object;

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
    pub scale: (f32, f32, f32),
}

impl Transform {
    pub fn new(position: (f32, f32, f32), rotation: (f32, f32, f32), scale: (f32, f32, f32)) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }
}

pub trait Actor: Object {
    fn get_transform(&self) -> Transform;
    fn set_transform(&mut self, transform: Transform);

    fn get_actor_location(&self) -> (f32, f32, f32) {
        self.get_transform().position
    }

    fn set_actor_location(&mut self, new_location: (f32, f32, f32)) {
        let mut transform = self.get_transform();
        transform.position = new_location;
        self.set_transform(transform);
    }

    fn get_actor_rotation(&self) -> (f32, f32, f32) {
        self.get_transform().rotation
    }

    fn set_actor_rotation(&mut self, new_rotation: (f32, f32, f32)) {
        let mut transform = self.get_transform();
        transform.rotation = new_rotation;
        self.set_transform(transform);
    }

    fn get_actor_scale(&self) -> (f32, f32, f32) {
        self.get_transform().scale
    }

    fn set_actor_scale(&mut self, new_scale: (f32, f32, f32)) {
        let mut transform = self.get_transform();
        transform.scale = new_scale;
        self.set_transform(transform);
    }
}

pub struct BaseActor {
    base: BaseObject,
    transform: Transform,
    components: HashMap<TypeId, Box<dyn Component>>,
}

impl BaseActor {
    pub fn new(name: String, position: (f32, f32, f32)) -> Self {
        Self {
            base: BaseObject::new(name),
            transform: Transform::new(position, (0.0, 0.0, 0.0), (1.0, 1.0, 1.0)),
        }
    }
}

impl ComponentContainer for PlayerActor {
    fn add_component<T: Component>(&mut self, component: T) {
        self.base.add_component(component);
    }

    fn get_component<T: Component>(&self) -> Option<&T> {
        self.base.get_component::<T>()
    }

    fn get_component_mut<T: Component>(&mut self) -> Option<&mut T> {
        self.base.get_component_mut::<T>()
    }

    fn remove_component<T: Component>(&mut self) -> Option<Box<dyn Component>> {
        self.base.remove_component::<T>()
    }
}

impl Object for BaseActor {
    fn begin_play(&mut self) {
        println!("Actor '{}' began play at location: {:?}", self.get_name(), self.get_actor_location());
    }

    fn end_play(&mut self) {
        println!("Actor '{}' ended play", self.get_name());
    }

    fn tick(&mut self, delta_time: f32) {
        // Basic update logic for actors
    }

    fn get_name(&self) -> &str {
        self.base.get_name()
    }

    fn set_name(&mut self, name: String) {
        self.base.set_name(name);
    }

    fn add_tag(&mut self, tag: String) {
        self.base.add_tag(tag);
    }

    fn has_tag(&self, tag: &str) -> bool {
        self.base.has_tag(tag)
    }

    fn get_tags(&self) -> &[String] {
        self.base.get_tags()
    }
}

impl Actor for BaseActor {
    fn get_transform(&self) -> Transform {
        self.transform
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

//////////////////////////////
//  Example implementation  //
//////////////////////////////
//
//  struct GenericActor {
//      base: BaseActor,
//  }
//  
//  impl GenericActor {
//      pub fn new(name: String, position: (f32, f32, f32)) -> Self {
//          Self {
//              base: BaseActor::new(name, position),
//          }
//      }
//  }
//  
//  impl Object for GenericActor {
//      // Delegate to base
//      fn begin_play(&mut self) { self.base.begin_play(); }
//      fn end_play(&mut self) { self.base.end_play(); }
//      fn tick(&mut self, delta_time: f32) { self.base.tick(delta_time); }
//      fn get_name(&self) -> &str { self.base.get_name() }
//      fn set_name(&mut self, name: String) { self.base.set_name(name); }
//      fn add_tag(&mut self, tag: String) { self.base.add_tag(tag); }
//      fn has_tag(&self, tag: &str) -> bool { self.base.has_tag(tag) }
//      fn get_tags(&self) -> &[String] { self.base.get_tags() }
//  }
//  
//  impl Actor for GenericActor {
//      fn get_transform(&self) -> Transform { self.base.get_transform() }
//      fn set_transform(&mut self, transform: Transform) { self.base.set_transform(transform); }
//  }