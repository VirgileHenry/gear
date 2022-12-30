use foundry::*;
extern crate cgmath;
extern crate gl;

const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct CameraComponent { // todo : handle position/rotation with transform
    perspective_matrix: cgmath::Matrix4::<f32>,
    field_of_view_y: f32,
    znear: f32,
    zfar: f32,
    is_main: bool, // todo : autre manière de gérer ca mieux
    //texture_id: Texture,
    //framebuffer_id: gl::types::GLuint,
}

impl CameraComponent {
    fn generate_camera_buffer() -> () {
        //let mut framebuffer_id;
        //gl::GenFramebuffers();
    }

    /// Create a new perspective camera from field of view, aspect ratio, znear and zfar
    pub fn new_perspective_camera(fovy: f32, aspect_ratio: f32, znear: f32, zfar:f32) -> CameraComponent {
        CameraComponent {
            perspective_matrix: OPENGL_TO_WGPU_MATRIX * cgmath::perspective(cgmath::Deg(fovy), aspect_ratio, znear, zfar),
            field_of_view_y: fovy,
            znear: znear,
            zfar: zfar,
            is_main: false,
        }
    }

    /// Change the aspect ratio of the camera.
    /// This recomputes a projection matrix from the internal stored values.
    #[allow(dead_code)]
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.perspective_matrix = OPENGL_TO_WGPU_MATRIX * cgmath::perspective(cgmath::Deg(self.field_of_view_y), aspect_ratio, self.znear, self.zfar);
    }

    /// Set this camera as the one rendering the scene to the window
    #[allow(dead_code)]
    pub fn set_as_main(&mut self, components: &mut ComponentTable) { // todo: mieux traiter
        // set all cameras to not main
        for cam_comp in iterate_over_component_mut!(components; CameraComponent) {
            cam_comp.is_main = false;
        }
        self.is_main = true;
    }

    #[allow(dead_code)]
    pub fn is_main(&self) -> bool {
        self.is_main
    }

    #[allow(dead_code)]
    pub fn get_perspective_mat(&self) -> cgmath::Matrix4<f32> {
        self.perspective_matrix
    }

}