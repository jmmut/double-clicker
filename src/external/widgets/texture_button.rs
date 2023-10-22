use macroquad::prelude::{
    draw_rectangle, draw_texture_ex, is_mouse_button_down, is_mouse_button_released,
    mouse_position, MouseButton, Rect, BLACK, GRAY, LIGHTGRAY, WHITE,
};
use macroquad::prelude::{DrawTextureParams, Texture2D};

use crate::external::backends::Vec2;
use crate::external::texture_drawer::draw::draw_panel_border;
use crate::external::widgets::button::Interaction;
use crate::external::widgets::text::TextRect;

pub struct TextureButton {
    texture: Texture2D,
    texture_highlighted: Option<Texture2D>,
    anchor: Anchor,
    rect: Rect,
    interaction: Interaction,
}

pub enum Anchor {
    Center(Vec2),
    TopLeft(Vec2),
    TopRight(Vec2),
    BottomLeft(Vec2),
    BottomRight(Vec2),
    // TODO: TopCenter, BottomCenter
}

impl Anchor {
    pub fn get_top_left_pixel(&self, size: Vec2) -> Vec2 {
        match self {
            Anchor::Center(Vec2 { x, y }) => {
                todo!()
            }
            Anchor::TopLeft(pos) => *pos,
            Anchor::TopRight(Vec2 { x, y }) => {
                todo!()
            }
            Anchor::BottomLeft(Vec2 { x, y }) => {
                todo!()
            }
            Anchor::BottomRight(Vec2 { x, y }) => {
                todo!()
            }
        }
    }
}

impl TextureButton {
    pub fn new(
        texture: Texture2D,
        texture_highlighted: Option<Texture2D>,
        anchor: Anchor,
        size_pixels: Vec2,
    ) -> Self {
        let top_left = anchor.get_top_left_pixel(size_pixels);
        let rect = Rect::new(
            (top_left.x).round(),
            (top_left.y).round(),
            (size_pixels.x).round(),
            (size_pixels.y).round(),
        );

        Self {
            texture,
            texture_highlighted,
            anchor,
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
    pub fn render(&self) {
        let chosen_texture = match self.interaction {
            Interaction::Clicked | Interaction::Pressing => self.texture,
            Interaction::Hovered => self.texture_highlighted.unwrap_or(self.texture),
            Interaction::None => self.texture,
        };

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
