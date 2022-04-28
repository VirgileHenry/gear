extern crate cgmath;
extern crate gl;
use super::super::super::rendering::shaders::ShaderProgram;
use super::super::{
    components::component::Component,
    scene::GameScene,
};

const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);


pub struct Camera {
    is_active: bool,
    view_matrix: cgmath::Matrix4::<f32>,
    field_of_view_y: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new_perspective_camera(fovy: f32, aspect: f32, znear: f32, zfar:f32) -> Camera {
        return Camera {
            is_active: true,
            view_matrix: OPENGL_TO_WGPU_MATRIX * cgmath::perspective(cgmath::Deg(fovy), aspect, znear, zfar),
            field_of_view_y: fovy,
            znear: znear,
            zfar: zfar,
        }
    }

    pub fn set_camera_uniform(&self, shader_program: &ShaderProgram) {
        unsafe {
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

        }
    }
}

impl Component for Camera {
    fn id() -> u32 where Self: Sized {
        return 2;
    }

    fn new(_object_id: u32) -> Camera where Self: Sized {
        return Camera::new_perspective_camera(70.0, 1.0, 0.1, 100.0)
    }

    fn on_created(&mut self) {
        // camera init
    }

    fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    fn is_active(&self) -> bool {
        return self.is_active;
    }

    fn update(&mut self, _scene: &mut GameScene, delta: f32) {
        // update for camera ?
    }

    fn render(&self) {
        // render the camera is not rendering the scene !
    }
}
