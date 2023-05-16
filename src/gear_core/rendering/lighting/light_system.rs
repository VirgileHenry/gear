use std::any::Any;
use std::rc::Rc;

use cgmath::{Vector3, Vector4};
use foundry::{ComponentTable, iterate_over_component, iterate_over_component_mut, System, Updatable, UpdateFrequency};
use refbox::RefBox;

use crate::{MainLight, MaterialProperties, MeshRenderer, PointLight, PointLightSensitive, ShaderProgram};
use crate::gear_core::rendering::opengl::color::Color;
use crate::gear_core::Transform;

/// Used as the main scene light
pub struct LightRegister {
    positions: RefBox<Vec<Vector4<f32>>>,
    colors: RefBox<Vec<Vector3<f32>>>,
}

impl LightRegister {
    pub fn new() -> System {
        System::new(
            Box::new(LightRegister {
                positions: RefBox::new(Vec::new()),
                colors: RefBox::new(Vec::new()),
            }),
            UpdateFrequency::PerFrame,
        )

    }
}

impl Updatable for LightRegister {
    fn update(&mut self, components: &mut ComponentTable, delta: f32, user_data: &mut dyn Any) {
        match (self.positions.try_borrow_mut(), self.colors.try_borrow_mut()) {
            (Ok(mut positions), Ok(mut colors)) => {
                positions.clear();
                colors.clear();
                let mut count = 0;
                for (point_light_tf, point_light) in iterate_over_component!(components; Transform, PointLight) {
                    let pos = point_light_tf.position();
                    positions.push(Vector4::new(pos.x, pos.y, pos.z, point_light.get_distance()));
                    colors.push(point_light.color_as_vec().clone());
                    count += 1;
                }
                for mesh_renderer in iterate_over_component_mut!(components; MeshRenderer) {
                    if let Some(prop) = mesh_renderer.material.get_mat_properties::<PointLightSensitive>() {
                        prop.set_color_and_pos(&self.colors.create_ref(), &self.positions.create_ref());
                    }
                }
            }
            _ => panic!("Erreur accès RefBox")
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
