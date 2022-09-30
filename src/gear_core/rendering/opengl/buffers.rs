extern crate gl;

pub struct BufferObject {
    buffer_type: gl::types::GLenum,
    vbo: gl::types::GLuint,
}

impl BufferObject {
    pub fn new(buffer_type: gl::types::GLenum) -> BufferObject {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        BufferObject {
            buffer_type,
            vbo
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type, 0);
        }
    }

    pub fn upload_data<T>(&self, data: &[T], usage: gl::types::GLenum) {
        unsafe {
            gl::BufferData(
                self.buffer_type, // target
                (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                usage, // usage
            );
        }
    }
}

impl Drop for BufferObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    }
}


pub struct VertexArray {
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        VertexArray {
            vao
        }
    }
    
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
}