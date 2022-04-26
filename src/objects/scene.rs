extern crate cgmath;
use std::collections::HashMap;
use crate::objects::{
    gearobject::GearObject,
    components::component::Component,
};

pub struct GameScene<'a> {
    // array of all objects
    // array of lights
    pub objects: Vec<GearObject<'a>>,
    components: HashMap<i32, Vec<Box<dyn Component>>>
}

impl<'a> GameScene<'a> {
    pub fn load_scene(name: &str) -> GameScene {
        // load a scene from it's name
    
        match name {
            _ => GameScene {
                objects: vec![],
                components: HashMap::new(),
            },
        }
    }

    pub fn create_new_component<C : Component>(&mut self) -> Option<&C> {
        // create and insert a component of the given type in the table
        // then return it. returns None if unable to create / insert it

        // if there is no vector of that component type, create one
        if !self.components.contains_key(&C::id()) {
            self.components.insert(C::id(), Vec::new());
        }

        // get the vector where we insert the component
        match self.components.get_mut(&C::id()) {
            Some(vec) => {
                // found the array ! push a new component, and get a reference to it to return it
                vec.push(Box::new(C::new()));
                match vec[vec.len()-1].as_any().downcast_ref::<C>() {
                    Some(result) => Some(result),
                    // Unable to downcast component, returns none (interpreted as "can't create component")
                    None => None,
                }
            },
            // Couldn't find the vector, so there have been an error while inserting it
            None => None,
        }
    }

    pub fn render_scene(&self) {
        // convention : first camera of camera component array is the main camera
        
    }

}

