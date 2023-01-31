use crate::gear_core::material::texture::TexturePresets;
use crate::material::texture::Texture2D;
use crate::Text;
use foundry::*;
use gl::types::{GLint, GLsizei, GLuint};

extern crate cgmath;
extern crate gl;

const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct CameraComponent {
    // todo : handle position/rotation with transform
    perspective_matrix: cgmath::Matrix4<f32>,
    field_of_view_y: f32,
    znear: f32,
    zfar: f32,
    is_main: bool, // todo : autre manière de gérer ca mieux
    viewport_dimensions: (i32, i32),
    color_attachment: Texture2D,
    depth_attachment: Texture2D,
    framebuffer_id: GLuint,
}

impl CameraComponent {
    fn generate_framebuffer(&mut self) {
        self.framebuffer_id = 0;
        unsafe {
            gl::Viewport(0, 0, self.viewport_dimensions.0, self.viewport_dimensions.1);

            gl::GenFramebuffers(1, &mut self.framebuffer_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                self.color_attachment.get_id(),
                0,
            );

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::DEPTH_ATTACHMENT,
                gl::TEXTURE_2D,
                self.depth_attachment.get_id(),
                0,
            );

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT
            {
                panic!("Framebuffer is not complete !")
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0); // unbind the framebuffer until needed
        }
    }

    pub fn resize_viewport(&mut self, dimensions: (i32, i32)) {
        self.viewport_dimensions = dimensions;
        self.color_attachment.resize(dimensions);
        self.depth_attachment.resize(dimensions);
    }

    /// Create a new perspective camera from field of view, aspect ratio, znear and zfar
    pub fn new_perspective_camera(
        dimensions: (i32, i32),
        fovy: f32,
        aspect_ratio: f32,
        znear: f32,
        zfar: f32,
    ) -> CameraComponent {
        let mut depth_presets = TexturePresets::default();
        depth_presets.internal_format = gl::DEPTH_ATTACHMENT;
        depth_presets.format = gl::DEPTH_COMPONENT;

        let mut camera = CameraComponent {
            perspective_matrix: OPENGL_TO_WGPU_MATRIX
                * cgmath::perspective(cgmath::Deg(fovy), aspect_ratio, znear, zfar),
            field_of_view_y: fovy,
            znear: znear,
            zfar: zfar,
            is_main: false,
            viewport_dimensions: dimensions,
            color_attachment: Texture2D::new_from_presets(
                dimensions,
                TexturePresets::color_default(),
                None,
            ),
            depth_attachment: Texture2D::new_from_presets(
                dimensions,
                TexturePresets::depth_default(),
                None,
            ),
            framebuffer_id: 0,
        };
        camera.generate_framebuffer();
        camera
    }

    /// Change the aspect ratio of the camera.
    /// This recomputes a projection matrix from the internal stored values.
    #[allow(dead_code)]
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.perspective_matrix = OPENGL_TO_WGPU_MATRIX
            * cgmath::perspective(
            cgmath::Deg(self.field_of_view_y),
            aspect_ratio,
            self.znear,
            self.zfar,
        );
    }

    /// Set this camera as the one rendering the scene to the window
    #[allow(dead_code)]
    pub fn set_as_main(&mut self, components: &mut ComponentTable) {
        // todo: mieux traiter
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

    #[allow(dead_code)]
    pub unsafe fn bind(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id);
    }

    #[allow(dead_code)]
    pub unsafe fn unbind(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    pub fn get_color_attachment(&self) -> &Texture2D {
        &self.color_attachment
    }
    pub fn get_depth_attachment(&self) -> &Texture2D {
        &self.depth_attachment
    }

    pub fn get_dimensions(&self) -> (i32, i32) {
        self.viewport_dimensions
    }

}
