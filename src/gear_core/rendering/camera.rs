extern crate cgmath;
extern crate gl;

use foundry::*;
use gl::types::GLuint;

use crate::gear_core::material::texture::TexturePresets;
use crate::material::texture::Texture2D;

/*
const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
*/

pub struct CameraComponent {
    field_of_view_y: f32,
    znear: f32,
    zfar: f32,
    is_main: bool, // todo : autre manière de gérer ca mieux    
    gl_camera: Option<GlCamera>, // None if not yet defined
}

pub struct GlCamera {
    perspective_matrix: cgmath::Matrix4<f32>,
    viewport_dimensions: (i32, i32),
    color_attachment: Texture2D,
    depth_attachment: Texture2D,
    framebuffer_id: GLuint,
    show_wireframe: bool, // todo : better way to handle camera render options
}

impl GlCamera {
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

    pub unsafe fn set_render_options(&self) {
        if self.show_wireframe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        } else {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }
    }
    
    pub fn toggle_show_wireframe(&mut self) {
        self.show_wireframe = !self.show_wireframe;
    }
}

impl CameraComponent {

    /// Generate a opengl framebuffer. Returns a boolean telling if it succeeded
    pub fn generate_gl_cam(&mut self, dimensions: (i32, i32)) -> &GlCamera {

        let mut id = 0;
        let color_text = Texture2D::new_from_presets(dimensions, TexturePresets::color_default(), None);
        let depth_text = Texture2D::new_from_presets(dimensions, TexturePresets::depth_default(), None);

        unsafe {
            gl::Viewport(0, 0, dimensions.0, dimensions.1);

            gl::GenFramebuffers(1, &mut id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, id);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                color_text.get_id(),
                0,
            );

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::DEPTH_ATTACHMENT,
                gl::TEXTURE_2D,
                depth_text.get_id(),
                0,
            );

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT
            {
                panic!("Framebuffer is not complete !")
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0); // unbind the framebuffer until needed
        }

        // assign 
        self.gl_camera = Some(GlCamera {
            perspective_matrix: cgmath::perspective(cgmath::Deg(self.field_of_view_y), dimensions.0 as f32 / dimensions.1 as f32, self.znear, self.zfar),
            viewport_dimensions: dimensions,
            color_attachment: color_text,
            depth_attachment: depth_text,
            framebuffer_id: id,
            show_wireframe: false,
        });

        match &self.gl_camera {
            Some(gl_cam) => gl_cam,
            None => panic!("gl camera not found but was just created."),
        }

    }

    pub fn resize_viewport(&mut self, dimensions: (i32, i32)) {
        match &mut self.gl_camera {
            Some(cam_buffer) => {
                cam_buffer.perspective_matrix = cgmath::perspective(cgmath::Deg(self.field_of_view_y), dimensions.0 as f32 / dimensions.1 as f32, self.znear, self.zfar);
                cam_buffer.viewport_dimensions = dimensions;
                cam_buffer.color_attachment.resize(dimensions);
                cam_buffer.depth_attachment.resize(dimensions);
            },
            None => {self.generate_gl_cam(dimensions);},
        }

    }

    /// Create a new perspective camera from field of view, aspect ratio, znear and zfar
    pub fn new_perspective_camera(
        dimensions: Option<(i32, i32)>,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> CameraComponent {
        let mut depth_presets = TexturePresets::default();
        depth_presets.internal_format = gl::DEPTH_ATTACHMENT;
        depth_presets.format = gl::DEPTH_COMPONENT;

        let mut camera = CameraComponent {
            field_of_view_y: fovy,
            znear: znear,
            zfar: zfar,
            is_main: false,
            gl_camera: None,
        };
        
        match dimensions {
            Some(dim) => {camera.generate_gl_cam(dim);},
            _ => {},
        }
        camera
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

    pub fn get_gl_camera(&self) -> &Option<GlCamera> {
        &self.gl_camera
    }

    pub fn toggle_show_wireframe(&mut self) {
        if let Some(camera) = &mut self.gl_camera {
            camera.show_wireframe = !camera.show_wireframe;
        }
    }


}
