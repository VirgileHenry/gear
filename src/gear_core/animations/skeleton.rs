use cgmath::{Vector3, Quaternion};



pub struct Bone {
    position: Vector3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>,
    parent: usize,
}

pub struct Skeleton {
    bones: Vec<Bone>
}


