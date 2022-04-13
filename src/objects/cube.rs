use crate::rendering::{camera, mesh::{Mesh, Vertex}, colors::{Color, ColorPrimitives}, shaders::ShaderProgram, renderer::RenderObject};
use crate::objects::{transform::Transform, gameobject::{RenderType, GameObject}};

pub struct Cube {
    transform: Transform,
    mesh: Mesh,
}

impl Cube {
    pub fn new() -> Cube {
        let cube_mesh = Mesh::from_data(
            vec![
                Vertex::new(cgmath::Vector3::new(-0.5, -0.5, -0.5), Color::from_primitive(ColorPrimitives::Red)),
                Vertex::new(cgmath::Vector3::new(-0.5, -0.5, 0.5), Color::from_primitive(ColorPrimitives::White)),
                Vertex::new(cgmath::Vector3::new(0.5, -0.5, -0.5), Color::from_primitive(ColorPrimitives::Cyan)),
                Vertex::new(cgmath::Vector3::new(0.5, -0.5, 0.5), Color::from_primitive(ColorPrimitives::Yellow)),
                Vertex::new(cgmath::Vector3::new(0.5, 0.5, -0.5), Color::from_primitive(ColorPrimitives::Blue)),
                Vertex::new(cgmath::Vector3::new(0.5, 0.5, 0.5), Color::from_primitive(ColorPrimitives::Black)),
                Vertex::new(cgmath::Vector3::new(-0.5, 0.5, -0.5), Color::from_primitive(ColorPrimitives::Magenta)),
                Vertex::new(cgmath::Vector3::new(-0.5, 0.5, 0.5), Color::from_primitive(ColorPrimitives::Green)) ],
            vec![
                0, 2, 3,   0, 3, 1, // front
                4, 6, 7,   4, 7, 5, // back
                3, 2, 4,   3, 4, 5, // right
                7, 6, 0,   7, 0, 1, // left
                6, 4, 2,   6, 2, 0, // bottom 
                1, 3, 5,   1, 5, 7  // top
            ],
        ); 
        
        let result = Cube {
            transform: Transform::origin(),
            mesh: cube_mesh,
        };
        
        return result;
    }
}

impl GameObject for Cube {    
    fn update(&mut self, delta:f32) {
        self.transform.rotate(&cgmath::Quaternion::<f32>::from_sv(delta, cgmath::Vector3::<f32>::new(0.0, 1.0, 0.0)));
    }

    fn set_uniform(&self, shader_program: &ShaderProgram) {
        unsafe {
            // assume the shader program is used
            // projection cam uniform
            use cgmath::Matrix; // to use as_ptr() on the matrix

            let world_pos_mat_loc = gl::GetUniformLocation(
                self.mesh.get_shader_prog().id(),
                "modelWorldPos\0".as_ptr() as *const gl::types::GLbyte
            );
            gl::UniformMatrix4fv(
                world_pos_mat_loc, // the data itself
                1 as gl::types::GLsizei, // the -number of element-
                gl::FALSE,
                self.transform.world_pos.as_ptr() as *const gl::types::GLfloat
            );
        }
    }

    fn to_render_objects(&self) -> RenderType {
        // give the mesh the set_uniform method
        return RenderType::Simple(RenderObject {
            mesh: &self.mesh,
            world_tf: &self.transform.world_pos,
        });
    }

    fn render(&self, camera: &camera::Camera) {
        self.mesh.use_shader_program();

        // set uniforms
        unsafe {
            self.set_uniform(self.mesh.get_shader_prog());
            camera.set_uniform(self.mesh.get_shader_prog());
        }

        self.mesh.draw();
    }
}