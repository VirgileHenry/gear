use foundry::iterate_over_component_mut;

extern crate cgmath;
extern crate gl;

const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct CameraComponent {
    view_matrix: cgmath::Matrix4::<f32>,
    field_of_view_y: f32,
    znear: f32,
    zfar: f32,
    is_main: bool,
}

impl CameraComponent {
    /// Create a new perspective camera from field of view, aspect ratio, znear and zfar
    pub fn new_perspective_camera(fovy: f32, aspect: f32, znear: f32, zfar:f32) -> CameraComponent {
        CameraComponent {
            view_matrix: OPENGL_TO_WGPU_MATRIX * cgmath::perspective(cgmath::Deg(fovy), aspect, znear, zfar),
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
        self.view_matrix = /* OPENGL_TO_WGPU_MATRIX * */ cgmath::perspective(cgmath::Deg(self.field_of_view_y), aspect_ratio, self.znear, self.zfar);
    }

    /// Set this camera as the one rendering the scene to the window
    #[allow(dead_code)]
    pub fn set_as_main(&mut self, components: &mut foundry::ecs::component_table::ComponentTable) {
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
    pub fn view_matrix(&self) -> cgmath::Matrix4<f32> {
        self.view_matrix
    }


}