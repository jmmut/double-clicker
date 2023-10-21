use std::ops::AddAssign;

use macroquad::prelude::{
    draw_rectangle, draw_rectangle_lines, draw_text, measure_text, Color, Rect, TextDimensions,
    BLACK,
};
use macroquad::text::Font;

use crate::external::backends::Vec2;

pub type Pixels = f32;

pub const ALERT_COLOR: Color = Color::new(0.98, 0.95, 0.3, 1.00);
pub const TEXT_PANEL_COLOR: Color = Color::new(1.0, 0.97, 0.8, 1.00);

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
    if panel_width < 0.0 || panel_height < 0.0 {
        return Vec::new();
    }
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
        ALERT_COLOR,
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
