use std::any::{Any, TypeId};
use std::collections::HashMap;
use super::object::Object;

pub trait Component: Any + Object {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ComponentContainer {
    fn add_component<T: Component>(&mut self, component: T);
    fn get_component<T: Component>(&self) -> Option<&T>;
    fn get_component_mut<T: Component>(&mut self) -> Option<&mut T>;
    fn remove_component<T: Component>(&mut self) -> Option<Box<dyn Component>>;
}