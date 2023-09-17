use crate::external::backends::Vec2;
use macroquad::prelude::{
    draw_texture_ex, is_mouse_button_down, is_mouse_button_pressed, mouse_position,
    DrawTextureParams, MouseButton, Rect, Texture2D, WHITE,
};

/// Returns if the texture was clicked this frame.
///
/// Renders texture_pressed instead of texture if the mouse is pressing on the rect of the texture.
/// The rect is in pixels.
pub fn is_texture_clicked(
    rect_pixels: Rect,
    texture: Texture2D,
    texture_pressed: Option<Texture2D>,
) -> bool {
    let clicking = rect_pixels.contains(Vec2::from(mouse_position()))
        && is_mouse_button_down(MouseButton::Left);
    let mut chosen_texture = texture;
    if clicking {
        if let Some(tp) = texture_pressed {
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
        && is_mouse_button_pressed(MouseButton::Left);
}
