use macroquad::prelude::{
    draw_rectangle, draw_text, is_mouse_button_down, is_mouse_button_released, mouse_position,
    Font, MouseButton, Rect, TextDimensions, BLACK, GRAY, LIGHTGRAY, WHITE,
};

use crate::external::backends::Vec2;
use crate::external::texture_drawer::draw::draw_panel_border;

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
    text: String,
    text_dimensions: TextDimensions,
    font_size: f32,
    rect: Rect,
    pad: Vec2,
    interaction: Interaction,
}

impl Button {
    pub fn from_top_left_pos<F>(
        text: &str,
        top_left_pixel: Vec2,
        font_size: f32,
        measure_text: F,
    ) -> Self
    where
        F: Fn(&str, Option<Font>, u16, f32) -> TextDimensions,
    {
        let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
        let pad = Vec2::new(font_size, font_size * 0.25);
        let rect = Rect::new(
            (top_left_pixel.x).round(),
            (top_left_pixel.y).round(),
            (text_dimensions.width + pad.x * 2.0).round(),
            (font_size + pad.y * 2.0).round(),
        );

        Self {
            text: text.to_string(),
            text_dimensions,
            font_size,
            rect,
            pad,
            interaction: Interaction::None,
        }
    }
    pub fn from_center_pos<F>(
        text: &str,
        center_pixel: Vec2,
        font_size: f32,
        measure_text: &F,
    ) -> Self
    where
        F: Fn(&str, Option<Font>, u16, f32) -> TextDimensions,
    {
        let mut button = Self::from_top_left_pos(text, center_pixel, font_size, measure_text);
        button.rect = button.rect.offset(-button.center_offset());
        button
    }
    pub fn from_bottom_right_pos<F>(
        text: &str,
        bottom_right_pixel: Vec2,
        font_size: f32,
        measure_text: &F,
    ) -> Self
    where
        F: Fn(&str, Option<Font>, u16, f32) -> TextDimensions,
    {
        let mut button = Self::from_top_left_pos(text, bottom_right_pixel, font_size, measure_text);
        button.rect = button.rect.offset(-button.rect.size());
        button
    }
    pub fn from_top_right_pos<F>(
        text: &str,
        top_right_pixel: Vec2,
        font_size: f32,
        measure_text: &F,
    ) -> Self
    where
        F: Fn(&str, Option<Font>, u16, f32) -> TextDimensions,
    {
        let mut button = Self::from_top_left_pos(text, top_right_pixel, font_size, measure_text);
        button.rect.x -= button.rect.w;
        button
    }
    pub fn from_bottom_left_pos<F>(
        text: &str,
        bottom_left_pixel: Vec2,
        font_size: f32,
        measure_text: &F,
    ) -> Self
    where
        F: Fn(&str, Option<Font>, u16, f32) -> TextDimensions,
    {
        let mut button = Self::from_top_left_pos(text, bottom_left_pixel, font_size, measure_text);
        button.rect.y -= button.rect.h;
        button
    }

    fn center_offset(&self) -> Vec2 {
        self.rect.size() * Vec2::new(0.5, 0.5)
    }

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
        let color = match self.interaction {
            Interaction::Clicked | Interaction::Pressing => GRAY,
            Interaction::Hovered => WHITE,
            Interaction::None => LIGHTGRAY,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
        draw_panel_border(self.rect, self.interaction);

        // draw_text() draws from the baseline of the text
        // https://en.wikipedia.org/wiki/Baseline_(typography)
        // I don't use self.text_dimensions.offset_y because that changes depending on the letters,
        // so I prefer an approximate distance that makes all buttons at the same baseline
        let approx_height_from_baseline_to_top = 0.75 * self.font_size;

        draw_text(
            &self.text,
            (self.rect.x + self.pad.x).round(),
            (self.rect.y + self.pad.y + approx_height_from_baseline_to_top).round(),
            self.font_size,
            BLACK,
        );
    }
}
