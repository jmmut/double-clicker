use macroquad::prelude::{
    draw_line, draw_texture_ex, is_mouse_button_down, is_mouse_button_released, mouse_position,
    Color, DrawTextureParams, MouseButton, Rect, Texture2D, DARKGRAY, WHITE,
};

use crate::external::backends::Vec2;
use crate::external::widgets::button::Interaction;

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
