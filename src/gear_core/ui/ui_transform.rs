use foundry::EntityRef;
use cgmath::{
    Matrix3,
    Vector2,
};

pub enum UIAnchorPoints {
    Center,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

// Todo : replace this with 3D matrices.
/// Position of a ui element on the screen.
pub struct UITransform {
    absolute_pos: cgmath::Matrix3<f32>,
    relative_pos: cgmath::Matrix3<f32>,
}


impl UITransform {
    pub fn origin() -> UITransform {
        UITransform {
            absolute_pos: Matrix3::<f32>::from_translation(Vector2::new(0., 0.)),
            relative_pos: Matrix3::<f32>::from_translation(Vector2::new(0., 0.)),
        }
    }

    pub fn recompute_screen_pos(&mut self, width: i32, height: i32) {
        unimplemented!();
    }

    pub fn contains_point(&mut self, point: Vector2<f64>) -> bool {
        unimplemented!();
    }

}
