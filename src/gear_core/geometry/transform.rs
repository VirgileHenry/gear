use cgmath::*;

#[allow(dead_code)]
pub struct Transform {
    position: Vector3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>,
    world_pos: Matrix4<f32>,
}

impl Transform {
    pub fn origin() -> Transform {
        Transform { 
            position: Vector3 { x: 0., y: 0., z: 0. },
            rotation: Quaternion::from(Euler::new(Rad(0.), Rad(0.), Rad(0.))),
            scale: Vector3 { x: 1., y: 1., z: 1. },
            world_pos: Matrix4 { 
                x: Vector4::new(1., 0., 0., 0.), 
                y: Vector4::new(0., 1., 0., 0.), 
                z: Vector4::new(0., 0., 1., 0.), 
                w: Vector4::new(0., 0., 0., 1.) }, // none mean uncomputed yet
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

    pub fn recompute_world_pos(&mut self) {
        self.world_pos = Matrix4::from_translation(self.position) *
            Matrix4::from_diagonal(Vector4::new(self.scale.x, self.scale.y, self.scale.z ,1.)) *
            Matrix4::from(self.rotation);
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
        self.world_pos
    }

    pub fn transform_direction(&self, direction: Vector3<f32>) -> Vector3<f32> {
        self.rotation * direction
    }


}