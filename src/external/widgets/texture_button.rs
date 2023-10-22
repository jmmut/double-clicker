use macroquad::prelude::{
    draw_texture_ex, is_mouse_button_down, is_mouse_button_released,
    mouse_position, MouseButton, Rect, WHITE,
};
use macroquad::prelude::{DrawTextureParams, Texture2D};

use crate::external::backends::Vec2;
use crate::external::widgets::button::Interaction;

pub struct TextureButton {
    rect: Rect,
    interaction: Interaction,
}

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

impl TextureButton {
    pub fn new(anchor: Anchor, size_pixels: Vec2) -> Self {
        let top_left = anchor.get_top_left_pixel(size_pixels);
        let rect = Rect::new(
            (top_left.x).round(),
            (top_left.y).round(),
            (size_pixels.x).round(),
            (size_pixels.y).round(),
        );

        Self {
            rect,
            interaction: Interaction::None,
        }
    }
}
impl TextureButton {
    pub fn rect(&self) -> Rect {
        self.rect
    }
    pub fn interact(&mut self) -> Interaction {
        self.interaction = if self.rect.contains(Vec2::from(mouse_position())) {
            if is_mouse_button_down(MouseButton::Left) {
                Interaction::Pressing
            } else if is_mouse_button_released(MouseButton::Left) {
                Interaction::Clicked
            } else {
                Interaction::Hovered
            }
        } else {
            Interaction::None
        };
        self.interaction
    }
    pub fn render(&self, textures: Vec<Texture2D>, textures_highlighted: Option<Vec<Texture2D>>) {
        let chosen_textures = match self.interaction {
            Interaction::Clicked | Interaction::Pressing => textures,
            Interaction::Hovered => textures_highlighted.unwrap_or(textures),
            Interaction::None => textures,
        };

        for chosen_texture in chosen_textures {
            draw_texture_ex(
                chosen_texture,
                self.rect.x,
                self.rect.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(self.rect.size()),
                    ..Default::default()
                },
            );
        }
    }
}
