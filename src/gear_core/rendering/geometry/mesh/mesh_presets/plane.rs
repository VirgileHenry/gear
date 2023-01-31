use cgmath::{InnerSpace, Vector3};
use crate::{Mesh, Vertex};

impl Mesh {

    /// Generate a plane mesh along the two provided axis, centered in 0.
    /// The length of each side of the plane is proportional to the length of the two axes.
    /// If the axes are orthonormal, the plane is a unit square.
    /// The normal of the plane has the same orientation as the cross product of A with B.
    pub fn plane(axisA: Vector3<f32>, axisB: Vector3<f32>) -> Mesh {

        // set each corner vertex of the plane
        let corners = vec![
            -0.5*axisA - 0.5*axisB,
            -0.5*axisA + 0.5*axisB,
            0.5*axisA + 0.5*axisB,
            0.5*axisA - 0.5*axisB,
        ];

        let normal = Vector3::normalize(axisA.cross(axisB));

        let vertices = vec![
            Vertex::new(corners[0].x, corners[0].y, corners[0].z, normal.x, normal.y, normal.z, 0., 0.),
            Vertex::new(corners[1].x, corners[1].y, corners[1].z, normal.x, normal.y, normal.z, 0., 1.),
            Vertex::new(corners[2].x, corners[2].y, corners[2].z, normal.x, normal.y, normal.z, 1., 1.),
            Vertex::new(corners[3].x, corners[3].y, corners[3].z, normal.x, normal.y, normal.z, 1., 0.),
        ];

        // connect the two triangles
        let triangles = vec![
            0, 1, 3,
            1, 2, 3,
        ];

        Mesh {
            vertices,
            triangles,
        }
    }
}