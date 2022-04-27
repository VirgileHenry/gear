extern crate cgmath;
use std::collections::HashMap;
use crate::objects::{
    gearobject::GearObject,
    components::component::{
        ComponentTable,
        Component,
    },
};

pub struct GameScene {
    // array of all objects
    // array of lights
    objects: Vec<GearObject>,
    components: ComponentTable,
    last_object_id: u32,
}

impl GameScene {
    pub fn load_scene(name: &str) -> GameScene {
        // load a scene from it's name
    
        match name {
            _ => GameScene {
                objects: Vec::new(),
                components: ComponentTable::new(),
                last_object_id: 0,
            },
        }
    }

    pub fn empty() -> GameScene {
        return GameScene {
            objects: Vec::new(),
            components: ComponentTable::new(),
            last_object_id: 0,
        }
    }

    pub fn instantiate_empty_object(&mut self) {
        // creates a new object to the scene and return a reference to it
        self.last_object_id += 1;
        self.objects.push(GearObject::empty(self.last_object_id, self));
    }

    pub fn add_component_to<C: Component>(&mut self, object: &GearObject) {
        self.components.add_component_to::<C>(object);
    }

    pub fn get_component_on<C: Component>(&self, object: &GearObject) -> Option<&C> {
        return self.components.get_component_on::<C>(object);
    }

    pub fn render_scene(&self) {
        // convention : first camera of camera component array is the main camera
        
    }

}

