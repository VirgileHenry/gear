extern crate cgmath;
use crate::objects::gearobject::GearObject;

pub struct GameScene {
    // array of all objects
    // array of lights
    pub objects: Vec<GearObject>,
    // pub camera: Camera,
}

impl GameScene {
    pub fn load_scene(name: &str) -> GameScene {
        // load a scene from it's name
    
        match name {
            _ => GameScene {
                objects: vec![],
                //camera: Camera::new_perspective_camera(60.0, 1.0, 0.1, 100.0),
            },
        }
    }

    pub fn update(&mut self, delta:f32) {
        
    }

}

