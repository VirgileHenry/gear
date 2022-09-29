extern crate cgmath;

#[allow(dead_code)]
pub struct Vertex {
    position: cgmath::Vector3::<f32>,
    normal: cgmath::Vector3::<f32>,
    uv: cgmath::Vector2::<f32>,
}

impl Vertex {
    pub fn new(p0: f32, p1: f32, p2: f32, n0: f32, n1: f32, n2: f32, u: f32, v: f32) -> Vertex {
        Vertex { 
            position: cgmath::Vector3 { x: p0, y: p1, z: p2 },
            normal: cgmath::Vector3 { x: n0, y: n1, z: n2 },
            uv: cgmath::Vector2 { x: u, y: v }
        }
    }
}

