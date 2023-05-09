use cgmath::Vector3;

use crate::{Mesh, Vertex};

impl Mesh {
    pub fn cube(side_size: f32) -> Mesh {
        let half_side_size = side_size / 2.;
        Mesh {
            vertices: vec![
                // top
                Vertex::new(-half_side_size, half_side_size, -half_side_size, 0.0, 1.0, 0.0, 0.25, 0.333),
                Vertex::new(-half_side_size, half_side_size, half_side_size, 0.0, 1.0, 0.0, 0.25, 0.666),
                Vertex::new(half_side_size, half_side_size, half_side_size, 0.0, 1.0, 0.0, 0.50, 0.333),
                Vertex::new(half_side_size, half_side_size, -half_side_size, 0.0, 1.0, 0.0, 0.50, 0.666),
                // front
                Vertex::new(-half_side_size, -half_side_size, -half_side_size, 0.0, 0.0, -1.0, 0.25, 0.00),
                Vertex::new(-half_side_size, half_side_size, -half_side_size, 0.0, 0.0, -1.0, 0.25, 0.333),
                Vertex::new(half_side_size, half_side_size, -half_side_size, 0.0, 0.0, -1.0, 0.50, 0.333),
                Vertex::new(half_side_size, -half_side_size, -half_side_size, 0.0, 0.0, -1.0, 0.50, 0.000),
                // right
                Vertex::new(half_side_size, -half_side_size, -half_side_size, 1.0, 0.0, 0.0, 0.75, 0.333),
                Vertex::new(half_side_size, -half_side_size, half_side_size, 1.0, 0.0, 0.0, 0.75, 0.666),
                Vertex::new(half_side_size, half_side_size, half_side_size, 1.0, 0.0, 0.0, 0.50, 0.666),
                Vertex::new(half_side_size, half_side_size, -half_side_size, 1.0, 0.0, 0.0, 0.50, 0.333),
                // bottom
                Vertex::new(-half_side_size, -half_side_size, -half_side_size, 0.0, 1.0, 0.0, 1.00, 0.333),
                Vertex::new(-half_side_size, -half_side_size, half_side_size, 0.0, 1.0, 0.0, 1.00, 0.666),
                Vertex::new(half_side_size, -half_side_size, half_side_size, 0.0, 1.0, 0.0, 0.75, 0.666),
                Vertex::new(half_side_size, -half_side_size, -half_side_size, 0.0, 1.0, 0.0, 0.75, 0.333),
                // back
                Vertex::new(-half_side_size, -half_side_size, half_side_size, 0.0, 0.0, 1.0, 0.25, 1.00),
                Vertex::new(-half_side_size, half_side_size, half_side_size, 0.0, 0.0, 1.0, 0.25, 0.666),
                Vertex::new(half_side_size, half_side_size, half_side_size, 0.0, 0.0, 1.0, 0.50, 0.666),
                Vertex::new(half_side_size, -half_side_size, half_side_size, 0.0, 0.0, 1.0, 0.50, 1.00),
                // left
                Vertex::new(-half_side_size, -half_side_size, -half_side_size, -1.0, 0.0, 0.0, 0.00, 0.333),
                Vertex::new(-half_side_size, -half_side_size, half_side_size, -1.0, 0.0, 0.0, 0.00, 0.666),
                Vertex::new(-half_side_size, half_side_size, half_side_size, -1.0, 0.0, 0.0, 0.25, 0.666),
                Vertex::new(-half_side_size, half_side_size, -half_side_size, -1.0, 0.0, 0.0, 0.25, 0.333),
            ],
            triangles: vec![
                0, 2, 1,    // top
                0, 3, 2,
                4, 6, 5,    // front
                4, 7, 6,
                8, 9, 10,   // right
                8, 10, 11,
                12, 13, 14, // bottom
                12, 14, 15,
                16, 17, 18, // back
                16, 18, 19,
                20, 22, 21, //left
                20, 23, 22,
            ],
            bounding_box: Some([Vector3::new(-half_side_size, -half_side_size, -half_side_size), Vector3::new(half_side_size, half_side_size, half_side_size)])
        }
    }
}