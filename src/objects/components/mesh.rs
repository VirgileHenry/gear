extern crate gl;
extern crate cgmath;
use crate::rendering::colors::Color;
use crate::rendering::colors::ColorPrimitives;
use crate::rendering::material::Material;
use crate::rendering::shaders::ShaderProgram;

use super::component::Component;


pub struct Vertex {
    position: cgmath::Vector3<f32>,
    color: Color,
}

impl Vertex {
    pub fn zero() -> Vertex {
        return Vertex {
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            color: Color::from_primitive(ColorPrimitives::White),
        }
    }

    pub fn new(position:cgmath::Vector3<f32>, color:Color) -> Vertex {
        return Vertex {
            position: position,
            color: color,
        }
    }

    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}


pub struct Mesh {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    ebo: gl::types::GLuint,
    vertices: Vec<Vertex>,
    triangles: Vec<u32>,
    material: Material,
    initialized: bool,
}

impl Mesh {
    pub fn new() -> Mesh {
        let mut result = Mesh {
            vao: 0,
            vbo: 0,
            ebo: 0,
            vertices: vec![
                Vertex::new(cgmath::Vector3::new(-0.5, -0.5, 0.0), Color::from_primitive(ColorPrimitives::Red)),
                Vertex::new(cgmath::Vector3::new(0.5, -0.5, 0.0), Color::from_primitive(ColorPrimitives::Green)),
                Vertex::new(cgmath::Vector3::new(0.0, 0.5, 0.0), Color::from_primitive(ColorPrimitives::Blue))],
            triangles: vec![0, 1, 2],
            material: Material::default(),
            initialized: false,
        };

        return result;
    }

    pub fn load(&mut self, from_vertices: Vec<Vertex>, from_triangles: Vec<u32>) {
        self.vao = 0;
        self.vbo = 0;
        self.ebo = 0;
        self.vertices = from_vertices;
        self.triangles = from_triangles;
        self.material = Material::default();
        self.initialized = false;
    }

    fn initialize(&mut self) {
        // create the buffers
        unsafe {
            // ask opengl to create the buffers
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);
            gl::GenVertexArrays(1, &mut self.vao);
            
            // populate the vbo buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (self.vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr, // size of data in bytes
                self.vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );
            
            // populate the ebo buffer
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, // target
                (self.triangles.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr, // size of data in bytes
                self.triangles.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );            

            // populate the vao buffer
            gl::BindVertexArray(self.vao);

            Vertex::vertex_attrib_pointer(std::mem::size_of::<Vertex>(), 0, 0);
            Vertex::vertex_attrib_pointer(std::mem::size_of::<Vertex>(), 1, std::mem::size_of::<cgmath::Vector3<f32>>());
            
            // unbind buffers
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        self.initialized = true;
    }

    pub fn get_shader_prog(&self) -> &ShaderProgram {
        return &self.material.shader_program;
    }

    pub fn use_shader_program(&self) {
        self.material.shader_program.set_used();
    }

    pub fn draw(&self) {
        // assume the right shader program is in use and uniforms are set
        if !self.initialized {
            println!("GEAR ERROR -> Unable to draw mesh : mesh have not been initialized. consider using mesh.initialize()")
        }
        
        unsafe {
            gl::BindVertexArray(self.vao);

            gl::DrawElements(
                gl::TRIANGLES,
                self.triangles.len() as gl::types::GLint,
                gl::UNSIGNED_INT,
                self.triangles.as_ptr() as *const gl::types::GLvoid
            );
            
            gl::BindVertexArray(0);
        }
    }
}

impl Component for Mesh {
    fn id() -> u32 where Self: Sized {
        return 1;
    }

    fn new() -> Self where Self: Sized {
        return Mesh::new();
    }

    fn is_active(&self) -> bool {
        return false; // a mesh does nothing
    }

    fn set_active(&mut self, _active: bool) { }

    fn on_created(&mut self) {
        // initialize mesh maybe ?
    }

    fn render(&self) {
        self.draw();
    }

    fn update(&mut self, _delta: f32) { }
}

