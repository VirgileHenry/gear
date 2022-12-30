use crate::{Mesh, Vertex};

impl Mesh {
    pub fn sphere(radius: f32, mut definition: u32) -> Mesh {
        if definition < 3 {
            definition = 3;
            println!("[GEAR ENGINE] -> [MESH BUILDER] -> Unable to build sphere with definition less than 3.");
        }
        // create the vec with the north pole vertex
        let mut vertices = vec!(Vertex::new(0.0, radius, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0));
        let delta_theta = 6.28318531794 / definition as f32;
        let delta_phi = 3.14159265897 / definition as f32;

        // loop through parallels
        for phi_int in 1..definition {
            let phi = phi_int as f32 * delta_phi;
            for theta_int in 0..definition {
                let theta = theta_int as f32 * delta_theta;
                vertices.push(Vertex::new(radius * phi.sin() * theta.cos(), radius * phi.cos(), radius * phi.sin() * theta.sin(),
                                          phi.sin() * theta.cos(), phi.cos(), phi.sin() * theta.sin(), 0.0, 0.0));
            }
        }

        vertices.push(Vertex::new(0.0, -radius, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0));

        let mut triangles = vec!();
        for i in 1..definition + 1 {
            triangles.push(0);
            triangles.push(i);
            triangles.push(i % definition + 1);
        }

        for phi_int in 0..definition - 2 {
            for theta_int in 0..definition {
                triangles.push(phi_int * definition + theta_int + 1); // +1 to avoid north pole vertex
                triangles.push((phi_int + 1) * definition + theta_int + 1);
                triangles.push(phi_int * definition + (theta_int + 1) % definition + 1);
                triangles.push(phi_int * definition + (theta_int + 1) % definition + 1);
                triangles.push((phi_int + 1) * definition + theta_int + 1);
                triangles.push((phi_int + 1) * definition + (theta_int + 1) % definition + 1);
            }
        }

        for i in 1..definition + 1 {
            triangles.push(definition * (definition - 1) + 1);
            triangles.push(definition * (definition - 2) + i);
            triangles.push(definition * (definition - 2) + i % definition + 1);
        }

        Mesh {
            vertices,
            triangles,
        }
    }
}