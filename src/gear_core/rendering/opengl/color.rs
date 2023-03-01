extern crate cgmath;

#[derive(Clone, Copy)]
pub struct Color {
    rgb_values: cgmath::Vector3<f32>,
}

impl Color {
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Color {
        Color {
            rgb_values: cgmath::Vector3 { x: r, y: g, z: b }, 
        }
    }

    pub fn as_vector(&self) -> cgmath::Vector3<f32> {
        self.rgb_values
    }

    pub fn lerp(col1: Color, col2: Color, t: f32) -> Color {
        Color { rgb_values: col1.as_vector() * t + col2.as_vector() * (1. - t) }
    }
}