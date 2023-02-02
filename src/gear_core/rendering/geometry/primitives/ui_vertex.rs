extern crate cgmath;
extern crate gl;

#[allow(dead_code)]
pub struct UIVertex {
    position: cgmath::Vector2::<f32>,
    uv: cgmath::Vector2::<f32>,
    depth: i32,
}


impl UIVertex {
    pub fn new(p0: f32, p1: f32, u: f32, v: f32, d: i32) -> UIVertex {
        UIVertex { 
            position: cgmath::Vector2 { x: p0, y: p1 },
            uv: cgmath::Vector2 { x: u, y: v },
            depth: d,
        }
    }

    pub fn vertex_attrib_pointer() {
        // in here because this depends on the vertex data type !
        unsafe {
            // shader layouts 
            gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
            gl::VertexAttribPointer(
                0,         // index of the generic vertex attribute ("layout (location = 0)")
                2,         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (std::mem::size_of::<Self>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null(),                                     // offset of the first component
            );
            gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
            gl::VertexAttribPointer(
                1,         // index of the generic vertex attribute ("layout (location = 0)")
                2,         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (std::mem::size_of::<Self>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                (std::mem::size_of::<cgmath::Vector3<f32>>()) as *const std::ffi::c_void, // offset of the first component
            );
            gl::EnableVertexAttribArray(2); // this is "layout (location = 2)" in vertex shader
            gl::VertexAttribPointer(
                2,         // index of the generic vertex attribute ("layout (location = 0)")
                1,         // the number of components per generic vertex attribute
                gl::INT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (std::mem::size_of::<Self>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                (2 * std::mem::size_of::<cgmath::Vector3<f32>>()) as *const std::ffi::c_void, // offset of the first component
            );
        }
    }
}

