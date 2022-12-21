use std::collections::VecDeque;
use foundry::*;

#[allow(dead_code)]
pub struct Transform {
    world_pos: cgmath::Matrix4<f32>,
    local_pos: cgmath::Matrix4<f32>,
    children: Vec<EntityRef>, // vec of indexes of childrens
    parent: Option<EntityRef>, // None if it's a root object
}

impl Transform {
    pub fn origin() -> Transform {
        Transform { 
            world_pos: cgmath::Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 
                0.0, 1.0, 0.0, 0.0, 
                0.0, 0.0, 1.0, 0.0, 
                0.0, 0.0, 0.0, 1.0
            ),
            local_pos: cgmath::Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 
                0.0, 1.0, 0.0, 0.0, 
                0.0, 0.0, 1.0, 0.0, 
                0.0, 0.0, 0.0, 1.0
            ),
            children: Vec::new(),
            parent: None,
        }
    }

    pub fn translated(mut self, tx: f32, ty: f32, tz: f32) -> Transform {
        self.translate(tx, ty, tz);
        self
    }

    pub fn euler(mut self, rx: f32, ry: f32, rz: f32) -> Transform {
        self.rotate_euler(rx, ry, rz);
        self
    }

    #[allow(dead_code)]
    pub fn translate(&mut self, tx: f32, ty: f32, tz: f32) {
        self.local_pos = self.local_pos * cgmath::Matrix4::from_translation(cgmath::Vector3 { x: tx, y: ty, z: tz });
        self.recompute_world_pos(None);
    }

    #[allow(dead_code)]
    pub fn rotate(&mut self, axis: cgmath::Vector3<f32>, angle: f32) {
        self.local_pos = self.local_pos * cgmath::Matrix4::from_axis_angle(axis, cgmath::Rad(angle));
        self.recompute_world_pos(None);
    }

    pub fn rotate_euler(&mut self, rx: f32, ry: f32, rz: f32) {
        self.local_pos = self.local_pos * cgmath::Matrix4::from(cgmath::Quaternion::from(cgmath::Euler::new(cgmath::Rad(rx), cgmath::Rad(ry), cgmath::Rad(rz))));
        self.recompute_world_pos(None);
    }

    pub fn position(&self) -> cgmath::Vector3<f32> {
        cgmath::Vector3 { x: self.world_pos.w.x, y: self.world_pos.w.y, z: self.world_pos.w.z }
    }

    #[allow(dead_code)]
    pub fn world_pos(&self) -> cgmath::Matrix4<f32> {
        self.world_pos
    }

    #[allow(dead_code)]
    fn recompute_world_pos(&mut self, parent_world_pos: Option<cgmath::Matrix4<f32>>) {
        self.world_pos = match parent_world_pos {Some(parent_tf) => self.local_pos * parent_tf, None => self.local_pos};
    }

    #[allow(dead_code)]
    #[allow(unreachable_code)]
    fn update_world_pos(&mut self, _components: &mut ComponentTable, _parent: Option<cgmath::Matrix4<f32>>) {
        unimplemented!("this is unusable");
        self.world_pos = match _parent {Some(parent_tf) => self.local_pos * parent_tf, None => self.local_pos};
        // unimplemented!("[GEAR ENGINE] -> Transform : not updating child movement");
        // store all children to update in a deque : with their parents transform (better way to do this ? we are copying lots of 4x4 matrix)
        let mut transforms: VecDeque<(EntityRef, cgmath::Matrix4<f32>)> = VecDeque::new();
        for child in self.children.iter() {
            transforms.push_back((*child, self.world_pos));
        }
        loop {
            match transforms.pop_front() {
                None => break, // no more elements
                Some(current) => {
                    match _components.get_component_mut::<Transform>(current.0) {
                        Some(current_transform) => {
                            // update this transform position
                            current_transform.recompute_world_pos(Some(current.1));
                            // add childrens to transforms to update
                            for child in current_transform.children.iter() {
                                transforms.push_back((*child, current_transform.world_pos()));
                            }
                        },
                        None => {},
                    }
                }
            }
        }

    }

}