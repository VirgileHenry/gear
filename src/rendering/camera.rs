extern crate cgmath;
extern crate gl;
use crate::objects::gameobject::{RenderType, GameObject};
use crate::rendering::{shaders::ShaderProgram, mesh::Mesh};
use crate::objects::transform::Transform;

const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);


pub struct Camera {
    pub transform: Transform,
    view_matrix: cgmath::Matrix4::<f32>,
    field_of_view_y: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new_perspective_camera(fovy: f32, aspect: f32, znear: f32, zfar:f32) -> Camera {
        return Camera {
            transform: Transform::at(0.0, 0.0, 5.0),
            view_matrix: OPENGL_TO_WGPU_MATRIX * cgmath::perspective(cgmath::Deg(fovy), aspect, znear, zfar),
            field_of_view_y: fovy,
            znear: znear,
            zfar: zfar,
        }
    }

    pub fn set_camera_uniform(&self, shader_program: &ShaderProgram) {
        unsafe {
            use std::ffi::CString;
            use cgmath::Matrix; // to use as_ptr() on the matrix
            // projection cam uniform
            let projection_mat_location = gl::GetUniformLocation(
                shader_program.id(),
                "projectionMat\0".as_ptr() as *const gl::types::GLbyte
            );
            gl::UniformMatrix4fv(
                projection_mat_location, // the data itself
                1 as gl::types::GLsizei, // the -number of element-
                gl::FALSE,
                self.view_matrix.as_ptr() as *const gl::types::GLfloat
            );
            // inverse of cam pos uniform
            let world_cam_mat_location = gl::GetUniformLocation(
                shader_program.id(),
                "cameraWorldPos\0".as_ptr() as *const gl::types::GLbyte
            );
            use cgmath::SquareMatrix; // to invert matrices
            let inverted_world_matrix = self.transform.world_pos.invert().unwrap();
            gl::UniformMatrix4fv(
                world_cam_mat_location, // the data itself
                1 as gl::types::GLsizei, // the -number of element-
                gl::FALSE,
                inverted_world_matrix.as_ptr() as *const gl::types::GLfloat
            );
        }
    }
}

impl GameObject for Camera {
    fn update(&mut self, delta: f32) {

    }

    fn to_render_objects(&self) -> RenderType {
        return RenderType::None;
    }

    fn render(&self, camera: &Camera) {

    }

    fn set_uniform(&self, shader_program: &ShaderProgram) {
        self.set_camera_uniform(shader_program);
    }

}