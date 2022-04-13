extern crate cgmath;
use crate::rendering::{camera::Camera, shaders::ShaderProgram, mesh::Mesh};
use crate::objects::{cube::Cube, gameobject::GameObject};

pub struct GameScene {
    // array of all objects
    // array of lights
    pub objects: Vec<Box<dyn GameObject>>,
    pub camera: Camera,
}

impl GameScene {
    pub fn load_scene(name: &str) -> GameScene {
        // load a scene from it's name
    
        match name {
            _ => GameScene {
                objects: vec![Box::new(Cube::new())],
                camera: Camera::new_perspective_camera(60.0, 1.0, 0.1, 100.0),
            },
        }
    }

    pub fn update(&mut self, delta:f32) {
        for object in self.objects.iter_mut() {
            object.update(delta);
        }
    }

}

