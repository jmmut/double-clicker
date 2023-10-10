use macroquad::prelude::{draw_rectangle, draw_text, draw_texture_ex, is_mouse_button_down, is_mouse_button_released, mouse_position, DrawTextureParams, MouseButton, Rect, TextDimensions, Texture2D, BLACK, GRAY, LIGHTGRAY, WHITE, measure_text};
use macroquad::text::Font;

use crate::external::backends::Vec2;

/// Returns if the texture was clicked this frame.
///
/// Renders texture_pressed instead of texture if the mouse is pressing on the rect of the texture.
/// The rect is in pixels.
pub fn is_texture_clicked(
    rect_pixels: Rect,
    texture: Texture2D,
    texture_highlighted: Option<Texture2D>,
) -> bool {
    let hovered = rect_pixels.contains(Vec2::from(mouse_position()));
    let clicking = rect_pixels.contains(Vec2::from(mouse_position()))
        && is_mouse_button_down(MouseButton::Left);

    let mut chosen_texture = texture;
    if hovered && !clicking {
        if let Some(tp) = texture_highlighted {
            chosen_texture = tp
        }
    }
    draw_texture_ex(
        chosen_texture,
        rect_pixels.x,
        rect_pixels.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(rect_pixels.size()),
            ..Default::default()
        },
    );
    return rect_pixels.contains(Vec2::from(mouse_position()))
        && is_mouse_button_released(MouseButton::Left);
}

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
        let pad = Vec2::new(font_size, font_size * 0.5);
        let rect = Rect::new(
            (top_left_pixel.x).round(),
            (top_left_pixel.y).round(),
            (text_dimensions.width + pad.x * 2.0).round(),
            (font_size + pad.y).round(),
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
        center_pixel: Vec2,
        font_size: f32,
        measure_text: &F,
    ) -> Self
    where
        F: Fn(&str, Option<Font>, u16, f32) -> TextDimensions,
    {
        let mut button = Self::from_top_left_pos(text, center_pixel, font_size, measure_text);
        button.rect = button.rect.offset(-2.0 * button.center_offset());
        button
    }

    fn center_offset(&self) -> Vec2 {
        Vec2::new(
            self.text_dimensions.width * 0.5 + self.pad.x,
            self.text_dimensions.offset_y * 0.5 + self.pad.y,
        )
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
        draw_text(
            &self.text,
            (self.rect.x + self.pad.x).round(),
            (self.rect.y + self.pad.y + self.text_dimensions.offset_y).round(),
            self.font_size,
            BLACK,
        );
    }
}

type Pixels = f32;
pub fn wrap_or_hide_text(text: &str, font_size: f32, line_height: Pixels, panel_width: Pixels, panel_height: Pixels) -> Vec<String> {
    assert!(panel_width >= 0.0);
    assert!(panel_height >= 0.0);
    let dimensions = measure_text(text, None, font_size as u16, 1.0);
    if dimensions.height > panel_height {
        return Vec::new(); // not enough space for a single line, hide all text
    } else if dimensions.width <= panel_width && dimensions.height <= panel_height {
        return vec![text.to_string()];
    } else {
        let mut remaining_text = text;
        let mut result = Vec::new();
        let letter_width_estimate : Pixels = dimensions.width / remaining_text.len() as f32;
        let letters_per_line_estimate = (panel_width / letter_width_estimate).trunc() as usize;
        while result.len() as f32 * line_height < panel_height {
            if remaining_text.len() <= letters_per_line_estimate {
                result.push(remaining_text.to_string());
                break;
            } else {
                let mut letters_per_line_estimate_utf8 = letters_per_line_estimate;
                while !remaining_text.is_char_boundary(letters_per_line_estimate_utf8+1) {
                    letters_per_line_estimate_utf8 -= 1;
                }
                let line_break_index = remaining_text[0..=letters_per_line_estimate_utf8].rfind(" ")
                    .unwrap_or(letters_per_line_estimate_utf8 - 1); // TODO: put a dash for cut words?
                result.push(remaining_text[0..=line_break_index].to_string());
                remaining_text = &remaining_text[(line_break_index + 1)..];
                if remaining_text.is_empty() {
                    break;
                }
            }
        }
        result
    }
}
