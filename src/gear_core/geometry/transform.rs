use cgmath::*;
use refbox::{BorrowError, Ref, RefBox,};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Transform {
    position: Vector3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>,
    world_pos: RefBox<Matrix4<f32>>,
    parent_world_pos: Option<Ref<Matrix4<f32>>>,
}

impl Transform {
    pub fn origin() -> Transform {
        Transform { 
            position: Vector3 { x: 0., y: 0., z: 0. },
            rotation: Quaternion::from(Euler::new(Rad(0.), Rad(0.), Rad(0.))),
            scale: Vector3 { x: 1., y: 1., z: 1. },
            world_pos: RefBox::new(Matrix4 { 
                x: Vector4::new(1., 0., 0., 0.), 
                y: Vector4::new(0., 1., 0., 0.), 
                z: Vector4::new(0., 0., 1., 0.), 
                w: Vector4::new(0., 0., 0., 1.) }), // none mean uncomputed yet
            parent_world_pos: None,
        }
    }

    pub fn translated(mut self, translation: Vector3<f32>) -> Transform {
        self.translate(translation);
        self
    }

    pub fn rotated(mut self, rotation: Euler<Rad<f32>>) -> Transform {
        self.rotate_euler(rotation);
        self
    }

    pub fn scaled(mut self, s: f32) -> Transform {
        self.scale(s);
        self
    }

    pub fn scaled_axis(mut self, s: Vector3<f32>) -> Transform {
        self.scale_axis(s);
        self
    }

    pub fn recompute_world_pos(&mut self) {
        match self.world_pos.try_borrow_mut() {
            Ok(mut world_pos) => {
                *world_pos = Matrix4::from_translation(self.position) *
                    Matrix4::from_diagonal(Vector4::new(self.scale.x, self.scale.y, self.scale.z ,1.)) *
                    Matrix4::from(self.rotation);
                match &self.parent_world_pos {
                    Some(parent_ref) => {
                        match parent_ref.try_borrow_mut() {
                            Ok(_parent_matrix) => {
                                // *world_pos = *parent_matrix * *world_pos;
                            }
                            Err(e) => {
                                match e {
                                    BorrowError::Borrowed => println!("[GEAR ENGINE] -> [TRANSFORM] -> Unable to update world position : parent transform already in use."),
                                    BorrowError::Dropped => println!("[GEAR ENGINE] -> [TRANSFORM] -> Unable to update world position : parent transform have been dropped."),
                                    // todo : maybe instead of error here, set this transform as root ?
                                }
                            }
                        }
                    }
                    _ => {/* no parent, nothing to do */}
                }
                },
            Err(_) => println!("[GEAR ENGINE] -> [TRANSFORM] -> Unable to recompute world pos : matrix already in use."),
        }
        
    }

    pub fn get_world_pos_ref(&self) -> Ref<Matrix4<f32>> {
        self.world_pos.create_ref()
    }

    pub fn set_parent(&mut self, parent: Option<&Transform>) {
        match parent {
            Some(parent) => {
                self.parent_world_pos = Some(parent.get_world_pos_ref());
                self.recompute_world_pos();
            },
            None => self.parent_world_pos = None,
        }
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.position += translation;
        self.recompute_world_pos();
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
        self.recompute_world_pos();
    }

    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation;
        self.recompute_world_pos();
    }

    pub fn scale(&mut self, s: f32) {
        self.scale *= s;
        self.recompute_world_pos();
    }

    pub fn scale_axis(&mut self, s: Vector3<f32>) {
        self.scale.mul_element_wise(s);
        self.recompute_world_pos();
    }

    pub fn rotate_euler(&mut self, rotation: Euler<Rad<f32>>) {
        self.rotation = self.rotation * Quaternion::from(rotation);
        self.recompute_world_pos();
    }

    pub fn rotate_around(&mut self, axis: Vector3<f32>, angle: Rad<f32>) {
        self.rotation = self.rotation * Quaternion::from_axis_angle(axis, angle);
        self.recompute_world_pos();
    }

    pub fn rotated_around(&mut self, axis: Vector3<f32>, angle: Rad<f32>) {
        self.rotation = Quaternion::from_axis_angle(axis, angle);
        self.recompute_world_pos();
    }

    pub fn set_euler(&mut self, rotation: Euler<Rad<f32>>) {
        self.rotation = Quaternion::from(rotation);
        self.recompute_world_pos();
    }

    pub fn position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn rotation(&self) -> Quaternion<f32> {
        self.rotation
    }

    pub fn euler(&self) -> Euler<Rad<f32>> {
        Euler::from(self.rotation)
    }

    pub fn world_pos(&self) -> Matrix4<f32> {
        match (self.world_pos.try_borrow_mut(), &self.parent_world_pos) {
            (Ok(world_pos), None) => *world_pos,
            (Ok(world_pos), Some(parent)) => *parent.try_borrow_mut().unwrap() * *world_pos,
            _ => panic!("[GEAR ENGINE] -> [TRANSFORM] -> Unable to get world pos, matrix already in use."),
        }
    }

    pub fn transform_direction(&self, direction: Vector3<f32>) -> Vector3<f32> {
        self.rotation * direction
    }

    pub fn transform(&self, point: Vector3<f32>) -> Vector3<f32> {
        let v4 = self.world_pos() * Vector4::new(point.x, point.y, point.z, 1.0);
        Vector3::new(v4.x, v4.y, v4.z)
    }

    pub fn inverse_transform(&self, point: Vector3<f32>) -> Vector3<f32> {
        let v4 = self.world_pos().invert().unwrap() * Vector4::new(point.x, point.y, point.z, 1.0);
        Vector3::new(v4.x, v4.y, v4.z)
    }


}