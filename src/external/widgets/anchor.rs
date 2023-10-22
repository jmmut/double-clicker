use crate::external::backends::Vec2;

#[derive(Copy, Clone)]
pub enum Anchor {
    Center { x: f32, y: f32 },
    TopLeft { x: f32, y: f32 },
    TopRight { x: f32, y: f32 },
    BottomLeft { x: f32, y: f32 },
    BottomRight { x: f32, y: f32 },
    // TODO: TopCenter, BottomCenter
}

impl Anchor {
    pub fn get_top_left_pixel(&self, size: Vec2) -> Vec2 {
        match *self {
            Anchor::Center { x, y } => Vec2::new(x - size.x * 0.5, y - size.y * 0.5),
            Anchor::TopLeft { x, y } => Vec2::new(x, y),
            Anchor::TopRight { x, y } => Vec2::new(x - size.x, y),
            Anchor::BottomLeft { x, y } => Vec2::new(x, y - size.y),
            Anchor::BottomRight { x, y } => Vec2::new(x - size.x, y - size.y),
        }
    }
}
