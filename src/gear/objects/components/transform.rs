extern crate cgmath;
use super::super::super::objects::{
    components::component::{
        Component,
        ComponentTable,
    },
};

pub struct Transform {
    object_id: u32,
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub world_pos: cgmath::Matrix4::<f32>,
    parent: u32,
    children: Vec<u32>,
}

impl Transform {
    pub fn origin(object_id: u32) -> Transform {
        return Transform {
            object_id: object_id,
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            rotation: cgmath::Quaternion::new(1.0, 0.0, 0.0, 0.0),
            world_pos: cgmath::Matrix4::<f32>::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ),
            parent: 0,
            children: Vec::new(),
        }
    }

    pub fn at(x:f32, y:f32, z:f32, object_id: u32) -> Transform {
        let mut result = Transform {
            object_id: object_id,
            position: cgmath::Vector3::new(x, y, z),
            rotation: cgmath::Quaternion::new(1.0, 0.0, 0.0, 0.0),
            world_pos: cgmath::Matrix4::<f32>::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ),
            parent: 0,
            children: Vec::new(),
        };

        // recompute world mat
        result.recompute_world_pos();

        // return the tf
        return result;
    }

    pub fn set_parent(&mut self, parent_id: u32, component_table: &mut ComponentTable) {
        // check the parent exist, assign ourselves as child and set it as parent
        match component_table.get_component_mut_on::<Transform>(parent_id) {
            Some(new_parent_tf) => {
                new_parent_tf.add_child(self.object_id);
                self.parent = parent_id;
            }
            None => println!("WARNING -> Unable to set object {} as parent : transform not found", parent_id),
        }
    }

    pub fn add_child(&mut self, child_id: u32) {
        // assume the children is a valid object id
        self.children.push(child_id);
    }

    pub fn recompute_world_pos(&mut self) -> cgmath::Matrix4<f32> {
        // compute matrix as translate + rotation
        self.world_pos = cgmath::Matrix4::<f32>::from(self.rotation) * cgmath::Matrix4::<f32>::from_translation(self.position);
        return self.world_pos
    }

    pub fn rotate(&mut self, rotation: &cgmath::Quaternion::<f32>) {
        self.rotation = self.rotation * rotation;
        self.recompute_world_pos();
    }
}


impl Component for Transform {
    fn id() -> u32 where Self: Sized {
        return 1;
    }

    fn new(object_id: u32) -> Transform where Self: Sized {
        return Transform::origin(object_id);
    }

    fn on_created(&mut self) { 
        // nothing to do for transform
    }
}