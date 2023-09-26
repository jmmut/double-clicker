use macroquad::prelude::{
    draw_rectangle, draw_text, draw_texture_ex, is_mouse_button_down, is_mouse_button_released,
    measure_text, mouse_position, DrawTextureParams, MouseButton, Rect, TextDimensions, Texture2D,
    BLACK, GRAY, LIGHTGRAY, WHITE,
};

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
    pub fn from_top_left_pos(text: &str, top_left_pixel: Vec2, font_size: f32) -> Self {
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
    pub fn from_center_pos(text: &str, center_pixel: Vec2, font_size: f32) -> Self {
        Self::offset_from_center(Self::from_top_left_pos(text, center_pixel, font_size))
    }
    fn offset_from_center(mut self) -> Self {
        self.rect.x += -self.text_dimensions.width * 0.5 - self.pad.x;
        self.rect.y += -self.text_dimensions.offset_y * 0.5 - self.pad.y;
        self
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
