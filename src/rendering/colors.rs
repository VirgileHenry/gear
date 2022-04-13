


pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

pub enum ColorPrimitives {
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
}

impl Color {
    pub fn from_primitive(primitive: ColorPrimitives) -> Color {
        match primitive {
            ColorPrimitives::Black => Color { r:0.0, g:0.0, b:0.0 },
            ColorPrimitives::Red => Color { r:1.0, g:0.0, b:0.0 },
            ColorPrimitives::Green => Color { r:0.0, g:1.0, b:0.0 },
            ColorPrimitives::Blue => Color { r:0.0, g:0.0, b:1.0 },
            ColorPrimitives::Yellow => Color { r:1.0, g:1.0, b:0.0 },
            ColorPrimitives::Magenta => Color { r:1.0, g:0.0, b:1.0 },
            ColorPrimitives::Cyan => Color { r:0.0, g:1.0, b:1.0 },
            ColorPrimitives::White => Color { r:1.0, g:1.0, b:1.0 },
        }
    }

    pub fn from_rgb(r:f32, g:f32, b:f32) -> Color {
        return Color {
            r:r, g:g, b:b
        }
    }
}