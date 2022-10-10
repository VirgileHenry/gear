extern crate cgmath;
extern crate gl;
use crate::gear_core::rendering::{
    geometry::primitives::Vertex, 
    material::Material,
    opengl,
};

pub struct Mesh {
    vertices: Vec<Vertex>,
    triangles: Vec<u32>,
}

impl Mesh {
    pub fn cube(side_size: f32) -> Mesh {
        Mesh {
            vertices: vec![
                // top
                Vertex::new(-side_size, side_size, -side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(-side_size, side_size, side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(side_size, side_size, side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(side_size, side_size, -side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                // front
                Vertex::new(-side_size, -side_size, -side_size, 0.0, 0.0, -1.0, 0.0, 0.0),
                Vertex::new(-side_size, side_size, -side_size, 0.0, 0.0, -1.0, 0.0, 0.0),
                Vertex::new(side_size, side_size, -side_size, 0.0, 0.0, -1.0, 0.0, 0.0),
                Vertex::new(side_size, -side_size, -side_size, 0.0, 0.0, -1.0, 0.0, 0.0),
                // right
                Vertex::new(side_size, -side_size, -side_size, 1.0, 0.0, 0.0, 0.0, 0.0),
                Vertex::new(side_size, -side_size, side_size, 1.0, 0.0, 0.0, 0.0, 0.0),
                Vertex::new(side_size, side_size, side_size, 1.0, 0.0, 0.0, 0.0, 0.0),
                Vertex::new(side_size, side_size, -side_size, 1.0, 0.0, 0.0, 0.0, 0.0),
                // bottom
                Vertex::new(-side_size, -side_size, -side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(-side_size, -side_size, side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(side_size, -side_size, side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(side_size, -side_size, -side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                // back
                Vertex::new(-side_size, -side_size, side_size, 0.0, 0.0, 1.0, 0.0, 0.0),
                Vertex::new(-side_size, side_size, side_size, 0.0, 0.0, 1.0, 0.0, 0.0),
                Vertex::new(side_size, side_size, side_size, 0.0, 0.0, 1.0, 0.0, 0.0),
                Vertex::new(side_size, -side_size, side_size, 0.0, 0.0, 1.0, 0.0, 0.0),
                // left
                Vertex::new(-side_size, -side_size, -side_size, -1.0, 0.0, 0.0, 0.0, 0.0),
                Vertex::new(-side_size, -side_size, side_size, -1.0, 0.0, 0.0, 0.0, 0.0),
                Vertex::new(-side_size, side_size, side_size, -1.0, 0.0, 0.0, 0.0, 0.0),
                Vertex::new(-side_size, side_size, -side_size, -1.0, 0.0, 0.0, 0.0, 0.0),

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
        }
    }

    pub fn sphere(radius: f32, mut definition: u32) -> Mesh {
        if definition < 3 {
            definition = 3;
            println!("[GEAR ENGINE] -> [MESH BUILDER] -> Unable to build sphere with definition less than 3.");
        }
        // create the vec with the north pole vertex
        let mut vertices = vec!(Vertex::new(0.0, radius, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0));
        let delta_theta = 6.28318531794 / definition as f32;
        let delta_phi = 3.14159265897 / definition as f32;
        
        // loop through paraleles
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
        for i in 1..definition+1 {
            triangles.push(0);
            triangles.push(i);
            triangles.push(i % definition + 1);
        }

        for phi_int in 0..definition-2 {
            for theta_int in 0..definition {
                triangles.push(phi_int * definition + theta_int + 1); // +1 to avoid north pole vertex
                triangles.push((phi_int + 1) * definition + theta_int + 1);
                triangles.push(phi_int * definition + (theta_int + 1) % definition + 1);
                triangles.push(phi_int * definition + (theta_int + 1) % definition + 1);
                triangles.push((phi_int + 1) * definition + theta_int + 1);
                triangles.push((phi_int + 1) * definition + (theta_int + 1) % definition + 1);
            }
        }

        for i in 1..definition+1 {
            triangles.push(definition * (definition - 1) + 1);
            triangles.push(definition * (definition - 2) + i);
            triangles.push(definition * (definition - 2) + i % definition + 1);
        }

        Mesh {
            vertices: vertices,
            triangles: triangles,
        }

    }

}

pub enum MeshType {
    Owned(Mesh),
    // Shared(&Mesh),
}


pub struct MeshRenderer {
    pub mesh: MeshType,
    pub material: Material,
    // opengl stuff
    vao: opengl::buffers::VertexArray, // holds rendering parameter data
    _vbo: opengl::buffers::BufferObject, // holds vertex data
    _ebo: opengl::buffers::BufferObject, // holds index data
}

impl MeshRenderer {
    pub fn new(mesh: MeshType, material: Material) -> MeshRenderer {
        match mesh {
            MeshType::Owned(mesh) => {
                // create and bind the vao
                let vao = opengl::buffers::VertexArray::new();
                vao.bind();
                // create the vbo, bind it, upload data to it and give the vertex the attrib pointers
                let vbo = opengl::buffers::BufferObject::new(gl::ARRAY_BUFFER);
                vbo.bind();
                vbo.upload_data(&mesh.vertices, gl::STATIC_DRAW);
                Vertex::vertex_attrib_pointer();
                // create the ebo, bind it, upload data to it
                let ebo = opengl::buffers::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER);
                ebo.bind();
                ebo.upload_data(&mesh.triangles, gl::STATIC_DRAW);
                // unbind the vao first, otherwise we will unbind the vbo and ebo from the vao
                vao.unbind();
                // ubind vbo and ebo
                ebo.unbind();
                vbo.unbind();
                
                MeshRenderer { 
                    mesh: MeshType::Owned(mesh),
                    material: material,
                    vao: vao,
                    _vbo: vbo,
                    _ebo: ebo,
                }
            }
        }
    }

    pub fn vao(&self) -> &opengl::buffers::VertexArray {
        &self.vao
    }

    pub fn triangles_len(&self) -> usize {
        match &self.mesh {
            MeshType::Owned(mesh) => mesh.triangles.len(),
        }
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}