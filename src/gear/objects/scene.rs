extern crate cgmath;
extern crate sdl2;
use std::time::Instant;
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
    last_instant: Instant,
}

impl GameScene {
    pub fn empty() -> GameScene {
        // empty scene 
        return GameScene {
            objects: Vec::new(),
            components: ComponentTable::new(),
            last_object_id: 0,
            last_instant: Instant::now(),
        }
    }

    pub fn instantiate_empty_object(&mut self) {
        // creates a new object to the scene and return a reference to it
        self.last_object_id += 1;
        self.objects.push(GearObject::empty(self.last_object_id, &mut self.components));
    }

    pub fn handle_events(&mut self, event: &Event) {
        for id in self.components.require_inputs.iter() {
            match self.components.table.get_mut(&id) {
                Some(map) => {
                    for component in map.values_mut() {
                        component.handle_event(event);
                    }
                }
                None => {},
            }
        }
    }

    pub fn update_scene(&mut self) {
        // compute scene delta time
        let delta_time = Instant::now() - self.last_instant; // get a duration between the two instants
        let delta_time: f32 = delta_time.as_nanos() as f32 / 1_000_000_000.0; // delta time in seconds
        self.last_instant = Instant::now();
        
        for id in self.components.require_update.iter() {
            match self.components.table.get_mut(&id) {
                Some(map) => {
                    for component in map.values_mut() {
                        component.update(self, delta_time);
                    }
                }
                None => {},
            }
        } 
    }

    pub fn render_scene(&self) {
        for id in self.components.require_render.iter() {
            match self.components.table.get(&id) {
                Some(map) => {
                    for component in map.values() {
                        component.render();
                    }
                }
                None => {},
            }
        }
    }

}

