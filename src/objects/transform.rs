extern crate cgmath;


pub struct Transform {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub world_pos: cgmath::Matrix4::<f32>,
}

impl Transform {
    pub fn origin() -> Transform {
        return Transform {
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            rotation: cgmath::Quaternion::new(1.0, 0.0, 0.0, 0.0),
            world_pos: cgmath::Matrix4::<f32>::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ),
        }
    }

    pub fn at(x:f32, y:f32, z:f32) -> Transform {
        let mut result = Transform {
            position: cgmath::Vector3::new(x, y, z),
            rotation: cgmath::Quaternion::new(1.0, 0.0, 0.0, 0.0),
            world_pos: cgmath::Matrix4::<f32>::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ),
        };

        // recompute world mat
        result.recompute_world_pos();

        // return the tf
        return result;
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