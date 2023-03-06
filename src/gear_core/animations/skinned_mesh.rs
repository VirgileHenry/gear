use crate::{SkeletalVertex, Skeleton};



pub struct SkinnedMesh {
    vertices: Vec<SkeletalVertex>,
    skeleton: Skeleton,
}