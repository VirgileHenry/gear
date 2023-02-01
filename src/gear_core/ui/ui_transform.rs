use cgmath::{
    SquareMatrix,
    Matrix3,
    Vector3,
    Vector2,
    Rad,
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



/// Position of a ui element on the screen.
pub struct UITransform {
    position: Vector2<f32>,
    relative_pos: Vector2<f32>,
    size: Vector2<f32>,
    relative_size: Vector2<f32>,
    anchor_point: UIAnchorPoints,
    rotation: Rad<f32>,
    screen_pos: Option<Matrix3<f32>>,
    inverted_screen_pos: Option<Matrix3<f32>>,
}


impl UITransform {
    pub fn new(position: Vector2<f32>,
        relative_pos: Vector2<f32>,
        size: Vector2<f32>,
        relative_size: Vector2<f32>,
        anchor_point: UIAnchorPoints,
        rotation: Rad<f32>) -> UITransform {
        UITransform {
            position,
            relative_pos,
            size,
            relative_size,
            anchor_point,
            rotation,
            screen_pos: None,
            inverted_screen_pos: None,
        }
    }

    pub fn origin() -> UITransform {
        UITransform {
            position: Vector2::new(0., 0.),
            relative_pos: Vector2::new(0., 0.),
            size: Vector2::new(1., 1.),
            relative_size: Vector2::new(0., 0.),
            anchor_point: UIAnchorPoints::Center,
            rotation: Rad(0.),
            screen_pos: None,
            inverted_screen_pos: None,
        }
    }

    pub fn anchored(mut self, anchor: UIAnchorPoints) -> UITransform {
        self.anchor_point = anchor;
        self
    }

    pub fn at(mut self, position: Vector2<f32>) -> UITransform {
        self.position = position;
        self
    }

    pub fn relative_at(mut self, position: Vector2<f32>) -> UITransform {
        self.relative_pos = position;
        self
    }

    pub fn sized(mut self, size: Vector2<f32>) -> UITransform {
        self.size = size;
        self
    }

    pub fn relative_sized(mut self, size: Vector2<f32>) -> UITransform {
        self.relative_size = size;
        self
    }

    pub fn rotated(mut self, rotation: Rad<f32>) -> UITransform {
        self.rotation = rotation;
        self
    }

    pub fn recompute_screen_pos(&mut self, width: i32, height: i32) -> Matrix3<f32> {
        // translation (we add a small -1 correction, could think harder about why it's there tbh)
        let translation = Matrix3::<f32>::from_translation(self.position + Vector2::new(self.relative_pos.x * width as f32 - 1., self.relative_pos.y * height as f32 - 1.));
        // scale
        let size = Matrix3::<f32>::from_nonuniform_scale(self.size.x + self.relative_size.x * width as f32, self.size.y + self.relative_size.y * height as f32);
        // rotation
        let rotation = Matrix3::<f32>::from_angle_z(self.rotation);
        // do all ops in order, right to left
        let screen_pos = translation * rotation * size * Self::anchor_matrix(&self.anchor_point);
        self.screen_pos = Some(screen_pos);
        self.inverted_screen_pos = match screen_pos.invert() {
            Some(inverted) => Some(inverted),
            None => {
                println!("[GEAR ENGINE] -> [UI TRANSFORM] -> Unable to invert transform matrix.");
                Some(Matrix3::<f32>::from_scale(1.)) // identity
            },
        };
        screen_pos
    }

    fn anchor_matrix(from: &UIAnchorPoints) -> Matrix3<f32> {
        match from {
            UIAnchorPoints::Center => Matrix3::<f32>::from_translation(Vector2::new(-0.5, -0.5)),
            UIAnchorPoints::Top => Matrix3::<f32>::from_translation(Vector2::new(-0.5, 0.)),
            UIAnchorPoints::TopRight => Matrix3::<f32>::from_translation(Vector2::new(-1., -0.)),
            UIAnchorPoints::Right => Matrix3::<f32>::from_translation(Vector2::new(-1., -0.5)),
            UIAnchorPoints::BottomRight => Matrix3::<f32>::from_translation(Vector2::new(-1., -1.)),
            UIAnchorPoints::Bottom => Matrix3::<f32>::from_translation(Vector2::new(-0.5, -1.)),
            UIAnchorPoints::BottomLeft => Matrix3::<f32>::from_translation(Vector2::new(-0., -1.)),
            UIAnchorPoints::Left => Matrix3::<f32>::from_translation(Vector2::new(-0., -0.5)),
            UIAnchorPoints::TopLeft => Matrix3::<f32>::from_translation(Vector2::new(-0., -0.)),
        }
    }

    pub fn screen_pos(&self) -> Option<Matrix3<f32>> {
        self.screen_pos
    }

    pub fn inverted_screen_pos(&self) -> Option<Matrix3<f32>> {
        self.inverted_screen_pos
    }

    pub fn contains_point(&mut self, point: Vector2<f64>) -> bool {
        let ui_space_point = match self.inverted_screen_pos{Some(v) => v, None => return false} * Vector3::<f32>::new(point.x as f32, point.y as f32, 1.);
        0. <= ui_space_point.x && ui_space_point.x <= 1. && 0. <= ui_space_point.y && ui_space_point.y <= 1.
    }

}
