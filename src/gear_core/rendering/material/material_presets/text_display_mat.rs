use cgmath::Vector2;

use crate::{MaterialProperties, ShaderProgram};

const CHAR_SIZE: f32 = 0.01;

// can change in the future to hold other kind of information relative to the char placement in the atlas texture
struct CharInfo {
    index: u8,
    top_left: Vector2<f32>,
}

fn compute_char_placement(
    text: &str,
    column_spacing: f32,
    line_spacing: f32,
    line_max_length: Option<f32>,
) -> Vec<CharInfo> {
    unimplemented!();
    /*
    let mut current_pos = (0., 0.);
    text.char_indices()
        .fold(
            Vec::new(),
            |mut v, (pos, char)| {
                match char {
                    ' ' => (),
                    '\n' => {
                        current_pos.0 = 0.;
                        current_pos.1 += line_spacing;
                        return v;
                    },
                    _ => {
                        let index = char.try_into().expect("Displayed string must be ascii char only");
                        if index < 33 || index > 126 {
                            panic!("Displayed string must be ascii char only");
                        }
                        v.push(CharInfo {
                            index: index-33,
                            top_left: Vector2::new(current_pos.0, current_pos.0),
                        });
                    }
                }

                current_pos += column_spacing;
                if let Some(max_length) = line_max_length {
                    if current_pos.0 > max_length {
                        current_pos.0 = 0.;
                        current_pos.1 += line_spacing;
                    }
                }
                v
            }
        )

     */
}

pub struct TextDisplayProp {
    text: String,
    chars_placement: Vec<CharInfo>,
    column_spacing: f32,
    line_spacing: f32,
    line_max_length: Option<f32>,
}

impl TextDisplayProp {
    pub fn new_default(text: &str) -> Self {
        Self {
            text: text.to_string(),
            chars_placement: compute_char_placement(text, 0.0, 0.0, None),
            column_spacing: CHAR_SIZE,
            line_spacing: CHAR_SIZE*2.,
            line_max_length: Some(1.),
        }
    }
    pub fn set_text() {

    }
}

impl MaterialProperties for TextDisplayProp {
    fn set_properties_to_shader(&self, shader: &ShaderProgram) {
        /*
        shader.set_int("char_count", self.text.len() as i32);
        for (index, char) in self.chars_placement.enumerate() {
            shader.set_vec2(&*format!("char_pos[{index}]"), char.top_left);
            shader.set_int(&*format!("char_code[{index}]"), char.index);
        }

         */
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
