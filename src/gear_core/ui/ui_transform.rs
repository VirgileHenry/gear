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
    layer: usize,
    screen_pos: Option<Matrix3<f32>>,
    inverted_screen_pos: Option<Matrix3<f32>>,
}


impl UITransform {
    pub fn new(position: Vector2<f32>,
        relative_pos: Vector2<f32>,
        size: Vector2<f32>,
        relative_size: Vector2<f32>,
        anchor_point: UIAnchorPoints,
        rotation: Rad<f32>,
        layer: usize) -> UITransform {
        UITransform {
            position,
            relative_pos,
            size,
            relative_size,
            anchor_point,
            rotation,
            layer,
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
            layer: 1, // one, so child are in front of their parents
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

    pub fn at_layer(mut self, layer: usize) -> UITransform {
        self.layer = layer;
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

    pub fn invalidate_screen_pos(&mut self) {
        self.screen_pos = None;
        self.inverted_screen_pos = None;
    }

    pub fn recompute_screen_pos(&mut self, screen_dim: (i32, i32)) -> Matrix3<f32> {
        // translation (we add a small -1 correction, could think harder about why it's there tbh)
        let translation = Matrix3::<f32>::from_translation(Vector2::new(
            screen_dim.0 as f32 * self.relative_pos.x + self.position.x,
            screen_dim.1 as f32 * self.relative_pos.y + self.position.y
        ));
        // scale
        let size = Matrix3::<f32>::from_nonuniform_scale(
            screen_dim.0 as f32 * self.relative_size.x + self.size.x,
            screen_dim.1  as f32 * self.relative_size.y + self.size.y
        );
        // rotation
        let rotation = Matrix3::<f32>::from_angle_z(self.rotation);
        // matrix to go from [0, width] x [0, height] => [-1, 1] x [-1, 1]
        // can't compute everything in that space beforehand because that would assume the screen is squared and 
        // cause distortions with the rotations (I've tried already)
        let to_gl_cube = Matrix3::from_translation(Vector2::new(-1., -1.)) * // translate it back
            Matrix3::from_nonuniform_scale(2. / screen_dim.0 as f32, 2. / screen_dim.1 as f32); // resize it to size 2x2

        // do all ops in order, right to left
        let screen_pos = to_gl_cube * translation * rotation * size * Self::anchor_matrix(&self.anchor_point);
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
            UIAnchorPoints::Top => Matrix3::<f32>::from_translation(Vector2::new(-0.5, -1.)),
            UIAnchorPoints::TopRight => Matrix3::<f32>::from_translation(Vector2::new(-1., -1.)),
            UIAnchorPoints::Right => Matrix3::<f32>::from_translation(Vector2::new(-1., -0.5)),
            UIAnchorPoints::BottomRight => Matrix3::<f32>::from_translation(Vector2::new(-1., 0.)),
            UIAnchorPoints::Bottom => Matrix3::<f32>::from_translation(Vector2::new(-0.5, 0.)),
            UIAnchorPoints::BottomLeft => Matrix3::<f32>::from_translation(Vector2::new(-0., 0.)),
            UIAnchorPoints::Left => Matrix3::<f32>::from_translation(Vector2::new(-0., -0.5)),
            UIAnchorPoints::TopLeft => Matrix3::<f32>::from_translation(Vector2::new(-0., -1.)),
        }
    }

    pub fn screen_pos(&self) -> Option<Matrix3<f32>> {
        self.screen_pos
    }

    pub fn inverted_screen_pos(&self) -> Option<Matrix3<f32>> {
        self.inverted_screen_pos
    }

    pub fn layer(&self) -> usize {
        // todo : add parent
        self.layer
    }

    pub fn contains_point(&mut self, point: Vector2<f64>) -> bool {
        let ui_space_point = match self.inverted_screen_pos{Some(v) => v, None => return false} * Vector3::<f32>::new(point.x as f32, point.y as f32, 1.);
        0. <= ui_space_point.x && ui_space_point.x <= 1. && 0. <= ui_space_point.y && ui_space_point.y <= 1.
    }

    pub fn rotate(&mut self, angle: Rad<f32>) {
        self.rotation += angle;
        self.invalidate_screen_pos();
    }

}
