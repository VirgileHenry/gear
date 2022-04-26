use std::any::Any;

use crate::objects::gearobject::GearObject;

// Helper traits:
// allows to cast dyn components back to the implemented components
pub trait ComponentToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

// allows to cast dyn Trait back to desired components
// used in 'get_component<C>()" function
impl<T: 'static> ComponentToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait Component: ComponentToAny {
    fn id() -> i32 where Self: Sized;

    fn new() -> Self where Self: Sized;

    fn set_active(&mut self, active: bool);

    fn is_active(&self) -> bool {
        return false; // default component is inactive 
    }

    fn on_created(&mut self); // method called when the component get added to an object

    fn update(&mut self, delta: f32);

    fn render(&self);
}

// ====== Component id table =====
/*
Component : 0 (default)
Mesh : 1
Camera : 2

*/