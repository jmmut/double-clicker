use macroquad::prelude::{
    draw_rectangle, is_mouse_button_down, is_mouse_button_released, mouse_position, MouseButton,
    Rect, BLACK, GRAY, LIGHTGRAY, WHITE,
};

use crate::external::backends::Vec2;
use crate::external::texture_drawer::draw::draw_panel_border;
use crate::external::widgets::text::TextRect;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Interaction {
    Pressing,
    Clicked,
    Hovered,
    None,
}

impl Interaction {
    pub fn is_clicked(&self) -> bool {
        *self == Interaction::Clicked
    }

    pub fn is_down(&self) -> bool {
        *self == Interaction::Pressing || *self == Interaction::Clicked
    }

    #[allow(unused)]
    pub fn is_hovered(&self) -> bool {
        *self == Interaction::Hovered
    }

    #[allow(unused)]
    pub fn is_hovered_or_clicked(&self) -> bool {
        *self == Interaction::Hovered || *self == Interaction::Clicked
    }
}

pub struct Button {
    text_rect: TextRect,
    interaction: Interaction,
}

impl Button {
    pub fn from_top_left_pixel(text: &str, top_left: Vec2, font_size: f32) -> Self {
        Self::from_text_rect(TextRect::from_top_left_pixel(text, top_left, font_size))
    }

    fn from_text_rect(text_rect: TextRect) -> Self {
        Self {
            text_rect,
            interaction: Interaction::None,
        }
    }

    pub fn from_center_pixel(text: &str, center_pixel: Vec2, font_size: f32) -> Self {
        Self::from_text_rect(TextRect::from_center_pixel(text, center_pixel, font_size))
    }
    pub fn from_bottom_right_pixel(text: &str, bottom_right: Vec2, font_size: f32) -> Self {
        let text_rect = TextRect::from_bottom_right_pixel(text, bottom_right, font_size);
        Self::from_text_rect(text_rect)
    }
    pub fn from_top_right_pixel(text: &str, top_right: Vec2, font_size: f32) -> Self {
        Self::from_text_rect(TextRect::from_top_right_pixel(text, top_right, font_size))
    }
    pub fn from_bottom_left_pixel(text: &str, bottom_left: Vec2, font_size: f32) -> Self {
        let text_rect = TextRect::from_bottom_left_pixel(text, bottom_left, font_size);
        Self::from_text_rect(text_rect)
    }

    pub fn rect(&self) -> Rect {
        self.text_rect.rect
    }
    pub fn interact(&mut self) -> Interaction {
        self.interaction = if self.text_rect.rect.contains(Vec2::from(mouse_position())) {
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
        let color = match self.interaction {
            Interaction::Clicked | Interaction::Pressing => GRAY,
            Interaction::Hovered => WHITE,
            Interaction::None => LIGHTGRAY,
        };
        let rect = self.text_rect.rect;
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
        draw_panel_border(rect, self.interaction);
        self.text_rect.render_text(BLACK);
    }
}
