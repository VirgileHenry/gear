use std::rc::Rc;

use cgmath::{Vector3, Vector4};
use foundry::iterate_over_component;
use refbox::{Ref, RefBox};

use crate::{MaterialProperties, ShaderProgram};
use crate::gear_core::rendering::opengl::color::Color;

/// Used as the main scene light
pub struct MainLight {
    pub ambient_intensity: f32,
    pub ambient_color: Color,
    pub main_intensity: f32,
    pub main_color: Color,
}

impl MainLight {
    pub fn new(main: Color, ambient: Color) -> MainLight {
        MainLight {
            ambient_intensity: 1.0,
            ambient_color: ambient,
            main_intensity: 1.0,
            main_color: main,
        }
    }

    pub fn main_color_as_vec(&self) -> cgmath::Vector3<f32> {
        self.main_color.as_vector() * self.main_intensity
    }

    pub fn ambient_color_as_vec(&self) -> cgmath::Vector3<f32> {
        self.ambient_color.as_vector() * self.ambient_intensity
    }

    pub fn set_ambiant(&mut self, color: Color) {
        self.ambient_color = color;
    }

    pub fn set_main(&mut self, color: Color) {
        self.main_color = color;
    }
}

/// Used as the main scene light
pub struct PointLight {
    color: Color,
    distance: f32,
}

impl PointLight {
    pub fn new(color: Color, distance: f32) -> PointLight {
        PointLight {
            color,
            distance,
        }
    }

    pub fn color_as_vec(&self) -> cgmath::Vector3<f32> {
        self.color.as_vector()
    }

    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
    }

    pub fn get_distance(&self) -> f32 {
        self.distance
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}


pub struct PointLightSensitive {
    color: Option<Ref<Vec<Vector3<f32>>>>,
    pos: Option<Ref<Vec<Vector4<f32>>>>,
}

impl PointLightSensitive {
    pub fn new() -> Self {
        Self {
            color: None,
            pos: None,
        }
    }

    pub fn set_color_and_pos(&mut self, color: &Ref<Vec<Vector3<f32>>>, pos: &Ref<Vec<Vector4<f32>>>) {
        self.color = Some(color.clone());
        self.pos = Some(pos.clone());
    }
}

impl MaterialProperties for PointLightSensitive {
    fn set_properties_to_shader(&self, shader: &ShaderProgram) {
        let mut i = 0;
        if let (Some(color_ref), Some(pos_ref)) = (&self.color, &self.pos) {
            match (color_ref.try_borrow_mut(), pos_ref.try_borrow_mut()) {
                (Ok(color), Ok(pos))=> {
                    for (point_pos, point_col) in pos.iter().zip(color.iter()) {
                        unsafe {
                            shader.set_vec4(&*format!("lightPos[{}]", i), *point_pos);
                            shader.set_vec3(&*format!("lightCol[{}]", i), *point_col);
                        }
                        i+=1;
                    }
                    unsafe {
                        shader.set_int("lightCount", color.len() as i32);
                    }
                }
                _ => panic!("Should not happen"),
            }

        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
