use macroquad::prelude::{
    draw_rectangle, is_mouse_button_down, is_mouse_button_released, mouse_position, MouseButton,
    Rect, BLACK, GRAY, LIGHTGRAY, WHITE,
};

use crate::external::backends::Vec2;
use crate::external::texture_drawer::draw::draw_panel_border;
use crate::external::widgets::anchor::Anchor;
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
    pub fn new(text: &str, position_pixels: Anchor, font_size: f32) -> Self {
        Self {
            text_rect: TextRect::new(text, position_pixels, font_size),
            interaction: Interaction::Pressing,
        }
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
