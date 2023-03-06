use cgmath::{Vector3, Quaternion};

use crate::Skeleton;


pub struct BonePosition {
    position: Vector3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>,
}

pub struct AnimationNode {
    bone_positions: Vec<BonePosition>,
}

pub struct Animation {
    nodes: Vec<(f32, AnimationNode)>,
    duration: f32,
}

impl Animation {
    pub fn duration(&self) -> f32 {
        self.duration
    }

    pub fn evaluate(&self, at: f32) -> Skeleton {
        todo!()
    }
}

