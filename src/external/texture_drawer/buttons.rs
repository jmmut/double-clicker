use std::collections::HashMap;

use macroquad::prelude::*;

use crate::external::texture_drawer::{BUY_PANEL_HORIZONTAL_PAD, BUY_PANEL_START_HEIGHT, draw, TextureDrawer};
use crate::external::texture_drawer::draw::Pixels;
use crate::screen::textures::Textures;
use crate::screen::translations::Translation;
use crate::world::heores::Hero;

const BUTTON_PAD: Pixels = 2.0;
pub struct Buttons {
    pub buy: HashMap<Hero, draw::Button>,
    pub sell: HashMap<Hero, draw::Button>,
    pub continue_playing: draw::Button,
    pub continue_after_game_over: draw::Button,
    pub change_language_to_spanish: draw::Button,
    pub change_language_to_english: draw::Button,
    pub extra: ExtraControls,
}

pub struct ExtraControls {
    pub show_extra_controls: draw::Button,
    pub show_debug_fps: draw::Button,
    pub change_arrangement: draw::Button,
    pub restart: draw::Button,
}

pub(crate) fn create_buttons<F>(
    font_size: f32,
    width: f32,
    height: f32,
    textures: &Textures,
    translation: &Translation,
    measure_text: &F,
) -> Buttons
    where
        F: Fn(&str, Option<Font>, u16, f32) -> TextDimensions,
{
    let spanish = draw::Button::from_bottom_right_pos(
        "Español",
        Vec2::new(width - BUTTON_PAD, height - BUTTON_PAD),
        font_size,
        &measure_text,
    );
    let spanish_rect = spanish.rect();
    Buttons {
        continue_after_game_over: draw::Button::from_center_pos(
            translation.restart,
            Vec2::new(width * 0.5, height * 0.7),
            font_size,
            &measure_text,
        ),
        buy: create_buy_hero_buttons(
            font_size,
            width,
            height,
            textures,
            translation,
            &measure_text,
        ),
        sell: create_sell_hero_buttons(
            font_size,
            width,
            height,
            textures,
            translation,
            &measure_text,
        ),
        continue_playing: draw::Button::from_center_pos(
            translation.continue_playing,
            Vec2::new(width * 0.5, height * 0.7),
            font_size,
            &measure_text,
        ),
        change_language_to_spanish: spanish,
        change_language_to_english: draw::Button::from_top_left_pos(
            "English",
            spanish_rect .point() - Vec2::new(spanish_rect.w + BUTTON_PAD, 0.0 ),
            font_size,
            &measure_text,
        ),
        extra: create_extra_buttons(font_size, width, height, translation, measure_text),
    }
}

fn create_buy_hero_buttons<F>(
    font_size: f32,
    width: f32,
    height: f32,
    textures: &Textures,
    translation: &Translation,
    measure_text: &F,
) -> HashMap<Hero, draw::Button>
    where
        F: Fn(&str, Option<Font>, u16, f32) -> TextDimensions,
{
    create_buy_or_sell_hero_buttons(
        font_size,
        width,
        height,
        textures,
        measure_text,
        translation.buy,
        0.02,
    )
}

fn create_sell_hero_buttons<F>(
    font_size: f32,
    width: f32,
    height: f32,
    textures: &Textures,
    translation: &Translation,
    measure_text: &F,
) -> HashMap<Hero, draw::Button>
    where
        F: Fn(&str, Option<Font>, u16, f32) -> TextDimensions,
{
    create_buy_or_sell_hero_buttons(
        font_size,
        width,
        height,
        textures,
        measure_text,
        translation.sell,
        0.1,
    )
}

fn create_buy_or_sell_hero_buttons<F>(
    font_size: f32,
    width: f32,
    height: f32,
    textures: &Textures,
    measure_text: &F,
    text: &str,
    extra_horizontal_offset: f32,
) -> HashMap<Hero, draw::Button>
    where
        F: Fn(&str, Option<Font>, u16, f32) -> TextDimensions,
{
    let mut buttons = HashMap::new();
    for hero in Hero::list() {
        let (horizontal_offset, vertical_offset) =
            TextureDrawer::get_buy_panel_offset(hero.index());
        let texture_offset = TextureDrawer::get_buy_text_offset_from_texture(
            hero.index(),
            width,
            height,
            textures.get(hero.texture_index()),
        );
        let x_coef = BUY_PANEL_HORIZONTAL_PAD
            + extra_horizontal_offset
            + horizontal_offset
            + texture_offset;
        let y_coef = BUY_PANEL_START_HEIGHT + 0.14 + vertical_offset;
        let font_size = font_size;
        let button = draw::Button::from_top_left_pos(
            text,
            Vec2::new(width * x_coef, height * y_coef),
            font_size,
            &measure_text,
        );
        buttons.insert(*hero, button);
    }
    buttons
}

fn create_extra_buttons<F>(
    font_size: f32,
    width: f32,
    height: f32,
    translation: &Translation,
    measure_text: &F,
) -> ExtraControls
    where
        F: Fn(&str, Option<Font>, u16, f32) -> TextDimensions
{
    let show_extra_controls = draw::Button::from_bottom_right_pos(
        translation.extra_controls,
        Vec2::new(width * 0.25, height - BUTTON_PAD),
        font_size,
        &measure_text,
    );
    let next_button = |text, prev_rect :Rect| {
        let button = draw::Button::from_top_left_pos(
            text,
            prev_rect.point() + Vec2::new(prev_rect.w + BUTTON_PAD, 0.0),
            font_size,
            &measure_text,
        );
        let rect = button.rect();
        (button, rect)
    };
    let prev_rect = show_extra_controls.rect();
    let (show_debug_fps, prev_rect) = next_button("Debug FPS", prev_rect);
    let (restart, prev_rect) = next_button(translation.restart, prev_rect);
    let (change_arrangement, _prev_rect) = next_button(translation.change_style, prev_rect);
    ExtraControls {
        show_extra_controls,
        show_debug_fps,
        restart,
        change_arrangement,
    }
}

