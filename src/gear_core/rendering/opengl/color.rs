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

    pub fn lerp(a: Color, b: Color, x: f32) -> Self {
        Self { rgb_values: a.rgb_values*(1. - x) + b.rgb_values*x }
    }

    pub fn as_vector(&self) -> cgmath::Vector3<f32> {
        self.rgb_values
    }

}