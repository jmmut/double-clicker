use macroquad::prelude::{draw_rectangle, draw_line, draw_rectangle_lines, draw_text, draw_texture_ex, is_mouse_button_down, is_mouse_button_released, measure_text, mouse_position, Color, DrawTextureParams, MouseButton, Rect, TextDimensions, Texture2D, BLACK, GRAY, LIGHTGRAY, WHITE, DARKGRAY};
use macroquad::text::Font;
use std::ops::AddAssign;

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
        bottom_right_pixel: Vec2,
        font_size: f32,
        measure_text: &F,
    ) -> Self
    where
        F: Fn(&str, Option<Font>, u16, f32) -> TextDimensions,
    {
        let mut button = Self::from_top_left_pos(text, bottom_right_pixel, font_size, measure_text);
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
        draw_panel_border(self.rect, self.interaction);
        draw_text(
            &self.text,
            (self.rect.x + self.pad.x).round(),
            (self.rect.y + self.pad.y + self.text_dimensions.offset_y).round(),
            self.font_size,
            BLACK,
        );
    }
}

pub fn draw_panel_border(rect: Rect, interaction: Interaction) {
    draw_windows_95_border(rect, interaction);
    // draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, BLACK);
}

// I swear I didn't realise what I was doing until I saw it running XD
pub fn draw_windows_95_border(rect: Rect, interaction: Interaction) {
    let lighter_gray = Color::new(0.88, 0.88, 0.88, 1.00);
    let (border_color_high, border_color_low) = if interaction.is_down() {
        // (BLACK, WHITE)
        (DARKGRAY, lighter_gray)
    } else {
        // (WHITE, BLACK)
        (lighter_gray, DARKGRAY)
    };
    let left = rect.x;
    let right = rect.x + rect.w;
    let top = rect.y;
    let bottom = rect.y + rect.h;
    draw_line(left, top, right, top, 1.0, border_color_high);
    draw_line(left, top, left, bottom, 1.0, border_color_high);
    draw_line(left, bottom, right, bottom, 1.0, border_color_low);
    draw_line(right, top, right, bottom, 1.0, border_color_low);
}

pub type Pixels = f32;
pub fn wrap_or_hide_text(
    text: &str,
    font_size: f32,
    line_height: Pixels,
    panel_width: Pixels,
    panel_height: Pixels,
) -> Vec<String> {
    wrap_or_hide_text_generic(
        text,
        font_size,
        line_height,
        panel_width,
        panel_height,
        &measure_text,
    )
}

#[allow(unused)]
pub fn wrap_or_hide_text_mocked(
    text: &str,
    font_size: f32,
    line_height: Pixels,
    panel_width: Pixels,
    panel_height: Pixels,
) -> Vec<String> {
    wrap_or_hide_text_generic(
        text,
        font_size,
        line_height,
        panel_width,
        panel_height,
        &|text, _font, font_size, _scale| {
            return TextDimensions {
                width: text.len() as f32 * font_size as f32,
                height: font_size as f32,
                offset_y: font_size as f32,
            };
        },
    )
}

pub fn wrap_or_hide_text_generic<F>(
    text: &str,
    font_size: f32,
    line_height: Pixels,
    panel_width: Pixels,
    panel_height: Pixels,
    measure_text: &F,
) -> Vec<String>
where
    F: Fn(&str, Option<Font>, u16, f32) -> TextDimensions,
{
    assert!(panel_width >= 0.0);
    assert!(panel_height >= 0.0);
    let dimensions = measure_text(text, None, font_size as u16, 1.0);
    if line_height.max(dimensions.height) > panel_height {
        return Vec::new(); // not enough space for a single line, hide all text
    } else if dimensions.width <= panel_width && dimensions.height <= panel_height {
        return vec![text.to_string()];
    } else {
        let mut remaining_text = text;
        let mut result: Vec<String> = Vec::new();
        let letter_width_estimate: Pixels = dimensions.width / remaining_text.len() as f32;
        let letters_per_line_estimate = (panel_width / letter_width_estimate).trunc() as usize;
        loop {
            if (result.len() + 1) as f32 * line_height >= panel_height {
                let mut last_line = result.pop().unwrap();
                // lines will usually end in a space, so the index points to the letter before the last one
                let mut last_letter_in_last_word_utf8 = last_line.len() - 2;
                while !last_line.is_char_boundary(last_letter_in_last_word_utf8) {
                    last_letter_in_last_word_utf8 -= 1;
                }
                let line_break_index = last_line[..last_letter_in_last_word_utf8].rfind(" ");
                let mut last_line = if let Some(previous_word_index) = line_break_index {
                    last_line[..previous_word_index].to_string()
                } else {
                    last_line.pop();
                    last_line.pop();
                    last_line.pop();
                    last_line
                };
                last_line.add_assign("...");
                result.push(last_line);
                break;
            }
            if remaining_text.len() <= letters_per_line_estimate {
                result.push(remaining_text.to_string());
                break;
            } else {
                let mut letters_per_line_estimate_utf8 = letters_per_line_estimate;
                while !remaining_text.is_char_boundary(letters_per_line_estimate_utf8 + 1) {
                    letters_per_line_estimate_utf8 -= 1;
                }
                let line_break_index = remaining_text[0..=letters_per_line_estimate_utf8]
                    .rfind(" ")
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

pub fn draw_tooltip_centered(text: &str, position: Vec2, width: f32, height: f32, font_size: f32) {
    let pad = font_size * 0.5;
    let tooltip_size = measure_text(&text, None, font_size as u16, 1.0);
    let text_rect = Rect::new(
        (width * position.x - tooltip_size.width * 0.5 - pad).round(),
        (height * position.y - tooltip_size.height - pad * 2.0).round(),
        tooltip_size.width + pad * 2.0,
        tooltip_size.height + pad * 2.0,
    );
    draw_rectangle(
        text_rect.x,
        text_rect.y,
        text_rect.w,
        text_rect.h,
        Color::new(0.98, 0.95, 0.3, 1.00),
    );
    draw_rectangle_lines(
        text_rect.x,
        text_rect.y,
        text_rect.w,
        text_rect.h,
        2.0,
        BLACK,
    );
    draw_text(
        &text,
        text_rect.x + pad,
        text_rect.y + pad + tooltip_size.offset_y,
        font_size,
        BLACK,
    );
}

pub fn draw_text_centered(text: &str, position: Vec2, width: f32, height: f32, font_size: f32) {
    let pad = font_size * 0.5;
    let tooltip_size = measure_text(&text, None, font_size as u16, 1.0);
    let text_rect = Rect::new(
        (width * position.x - tooltip_size.width * 0.5 - pad).round(),
        (height * position.y - tooltip_size.height - pad * 2.0).round(),
        tooltip_size.width + pad * 2.0,
        tooltip_size.height + pad * 2.0,
    );
    draw_text(
        &text,
        text_rect.x + pad,
        text_rect.y + pad + tooltip_size.offset_y,
        font_size,
        BLACK,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_text_basic() {
        let text = "word_1 word_2 word_3";
        let font_size = 10.0;
        let lines = wrap_or_hide_text_mocked(
            text,
            font_size,
            font_size,
            text.len() as f32 * font_size - 1.0,
            font_size * 3.0,
        );
        assert_eq!(lines, vec!["word_1 word_2 ", "word_3"]);
    }

    #[test]
    fn test_wrap_text_ellipsis() {
        let text = "word_1 word_2 word_3";
        let font_size = 10.0;
        let lines = wrap_or_hide_text_mocked(
            text,
            font_size,
            font_size,
            text.len() as f32 * font_size - 1.0,
            font_size * 1.5,
        );
        assert_eq!(lines, vec!["word_1..."]);
    }
    #[test]
    fn test_wrap_text_no_space() {
        let text = "word_1 word_2 word_3";
        let font_size = 10.0;
        let lines = wrap_or_hide_text_mocked(
            text,
            font_size,
            font_size,
            text.len() as f32 * font_size - 1.0,
            font_size * 0.5,
        );
        assert_eq!(lines, Vec::<String>::new());
    }
    #[test]
    fn test_wrap_text_long_word() {
        let text = "looooooooooooooooooooooong_word";
        let font_size = 10.0;
        let lines = wrap_or_hide_text_mocked(
            text,
            font_size,
            font_size,
            text.len() as f32 * font_size - 1.0,
            font_size * 1.5,
        );
        assert_eq!(lines, vec!["looooooooooooooooooooooong_..."]);
    }
}
