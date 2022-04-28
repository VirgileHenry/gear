extern crate cgmath;
extern crate sdl2;

use sdl2::event::Event;
use super::super::objects::{
    gearobject::GearObject,
    components::component::{
        ComponentTable,
    },
};

// allow scene loading, saving, etc
pub struct SceneManager {
    pub main_scene: Option<GameScene>,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        return SceneManager {
            main_scene: None,
        }
    }

    pub fn load_scene() {
        //main_scene = Option<GameScene>
    }
}

pub struct GameScene {
    // array of all objects
    // array of lights
    objects: Vec<GearObject>,
    components: ComponentTable,
    last_object_id: u32,
}

impl GameScene {
    pub fn empty() -> GameScene {
        // empty scene 
        return GameScene {
            objects: Vec::new(),
            components: ComponentTable::new(),
            last_object_id: 0,
        }
    }

    pub fn instantiate_empty_object(&mut self) {
        // creates a new object to the scene and return a reference to it
        self.last_object_id += 1;
        self.objects.push(GearObject::empty(self.last_object_id, &mut self.components));
    }

    pub fn handle_events(&mut self, event: &Event) {

    }

    pub fn update_scene(&mut self) {

    }

    pub fn render_scene(&self) {
        // convention : first camera of camera component array is the main camera
        
    }

}

