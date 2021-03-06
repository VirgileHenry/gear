extern crate sdl2;
use sdl2::event::Event;
use std::any::Any;
use std::collections::HashMap;
use super::super::super::objects::scene::GameScene;

// Helper traits:
// allows to cast dyn components back to the implemented components
pub trait ComponentToAny: 'static {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// allows to cast dyn Trait back to desired components
// used in 'get_component<C>()" function
impl<T: 'static> ComponentToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait Component: ComponentToAny {
    fn id() -> u32 where Self: Sized;

    fn require_render() -> bool where Self: Sized {
        return false;
    }

    fn require_update() -> bool where Self: Sized {
        return false;
    }

    fn require_inputs() -> bool where Self: Sized {
        return false;
    }

    fn new(object_id: u32) -> Self where Self: Sized;

    fn on_created(&mut self) {
        // nothing to do by default
    }

    fn update(&mut self, _scene: &mut GameScene, _delta: f32) {
        // nothing to do by default
    }

    fn render(&self) {
        // nothing to do by default
    }

    fn handle_event(&mut self, _event: &Event) {
        // nothing to do by default
    }
}
// ====== Component id table =====
/*
Component : 0 (default)
Transform: 1
Camera : 2
Mesh : 3

*/

pub struct ComponentTable {
    // hashtable of array of components: 
    // used in scene to store all components !
    pub table: HashMap<u32, HashMap<u32, Box<dyn Component>>>,
    pub require_render: Vec<u32>,
    pub require_update: Vec<u32>,
    pub require_inputs: Vec<u32>,
}

impl ComponentTable {
    pub fn new() -> ComponentTable {
        return ComponentTable {
            table: HashMap::new(),
            require_render: Vec::new(),
            require_update: Vec::new(),
            require_inputs: Vec::new(),
        }
    }

    pub fn get_component_on<C: Component>(&self, object_id: u32) -> Option<&C> {
        match self.table.get(&C::id()) {
            Some(map) => {
                match map.get(&object_id) {
                    Some(boxed_component) => {
                        // cast the dyn Trait back to component using ComponentToAny
                        let it = boxed_component.as_any();
                        match it.downcast_ref::<C>() {
                            Some(component) => Some(component),
                            // if we find none here, we couldn't cast the component :
                            // it was stored under a wrong id ! check unique ids !
                            None => {
                                println!("WARNING -> unable to cast component to desired type.
                                    This may be due to components ids not being uniques !");
                                None
                            },
                        }
                    }
                    // specified object does not contain the component
                    None => None,
                }
            }
            // no objects contains this component
            None => None,
        }
    }

    pub fn get_component_mut_on<C: Component>(&mut self, object_id: u32) -> Option<&mut C> {
        match self.table.get_mut(&C::id()) {
            Some(map) => {
                match map.get_mut(&object_id) {
                    Some(boxed_component) => {
                        // cast the dyn Trait back to component using ComponentToAny
                        match boxed_component.as_any_mut().downcast_mut::<C>() {
                            Some(component) => Some(component),
                            // if we find none here, we couldn't cast the component :
                            // it was stored under a wrong id ! check unique ids !
                            None => {
                                println!("WARNING -> unable to cast component to desired type.
                                    This may be due to components ids not being uniques !");
                                None
                            },
                        }
                    }
                    // specified object does not contain the component
                    None => None,
                }
            }
            // no objects contains this component
            None => None,
        }
    }

    pub fn add_component_to<C :Component>(&mut self, object_id: u32) -> Option<&C> {
        // create and insert a component of the given type in the table
        // then return it. returns None if unable to create / insert it

        // if there is no vector of that component type, create one
        if !self.table.contains_key(&C::id()) {
            self.table.insert(C::id(), HashMap::new());
        }

        // update components needs
        if C::require_render() {
            if !self.require_render.contains(&C::id()) {
                self.require_render.push(C::id());
            }
        }

        if C::require_update() {
            if !self.require_update.contains(&C::id()) {
                self.require_update.push(C::id());
            }
        }

        if C::require_inputs() {
            if !self.require_inputs.contains(&C::id()) {
                self.require_inputs.push(C::id());
            }
        }

        // get the vector where we insert the component
        match self.table.get_mut(&C::id()) {
            Some(map) => {
                // found the array ! push a new component, and get a reference to it to return it
                map.insert(object_id, Box::new(C::new(object_id)));
                match map.get(&object_id) {
                    Some(boxed_component) => match boxed_component.as_any().downcast_ref::<C>() {
                        Some(result) => Some(result),
                        // Unable to downcast component, returns none (interpreted as "can't create component")
                        None => None,
                    }
                    // this should never happen
                    None => None,
                }
            },
            // Couldn't find the vector, so there have been an error while inserting it
            None => None,
        }
    }

    pub fn remove_component_on<C: Component>(&mut self, object_id: u32) {
        match self.table.get_mut(&C::id()) {
            Some(map) => {
                map.remove(&object_id);
            }
            // no objects contains this component
            None => { },
        }
    }

}

