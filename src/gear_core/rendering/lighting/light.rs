use crate::gear_core::rendering::opengl::color::Color;

/// Used as the main scene light
pub struct MainLight {
    pub ambient_color: Color,
    pub main_color: Color,
}

impl MainLight {
    pub fn new(main: Color, ambient: Color) -> MainLight {
        MainLight { 
            ambient_color: ambient,
            main_color: main
        }
    }

    pub fn color_as_vec(&self) -> cgmath::Vector3<f32> {
        self.main_color.as_vector()
    }

    pub fn ambient_as_vec(&self) -> cgmath::Vector3<f32> {
        self.ambient_color.as_vector()
    }

    pub fn set_ambiant(&mut self, color: Color) {
        self.ambient_color = color;
    }

    pub fn set_main(&mut self, color: Color) {
        self.main_color = color;
    }
}