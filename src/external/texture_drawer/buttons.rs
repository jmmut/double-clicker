use std::collections::HashMap;

use macroquad::prelude::*;

use crate::external::texture_drawer::{
    TextureDrawer, BUY_PANEL_HORIZONTAL_PAD, BUY_PANEL_START_HEIGHT, CLEAN_BACKGROUND_COLOR,
    DIRTY_BACKGROUND_COLOR,
};
use crate::external::widgets::anchor::Anchor;
use crate::external::widgets::button::Button;
use crate::external::widgets::text::Pixels;
use crate::external::widgets::texture_button::TextureButton;
use crate::screen::textures::Textures;
use crate::screen::translations::Translation;
use crate::world::heores::Hero;

const BUTTON_PAD: Pixels = 2.0;
pub struct Buttons {
    pub clean: TextureButton,
    pub dirty: TextureButton,
    pub buy: HashMap<Hero, Button>,
    pub sell: HashMap<Hero, Button>,
    pub continue_playing: Button,
    pub continue_after_game_over: Button,
    pub change_language_to_spanish: Button,
    pub change_language_to_english: Button,
    pub extra: ExtraControls,
}

pub struct ExtraControls {
    pub show_extra_controls: Button,
    pub show_debug_fps: Button,
    pub change_arrangement: Button,
    pub restart: Button,
}

pub fn create_buttons(
    font_size: f32,
    width: f32,
    height: f32,
    textures: &Textures,
    translation: &Translation,
) -> Buttons {
    let spanish = Button::new(
        "Español",
        Anchor::bottom_right(width - BUTTON_PAD, height - BUTTON_PAD),
        font_size,
    );
    let english = Button::new(
        "English",
        Anchor::top_left(
            spanish.rect().x - (spanish.rect().w + BUTTON_PAD),
            spanish.rect().y,
        ),
        font_size,
    );
    let (clean, dirty) = create_clean_and_dirty_buttons(width, height);
    Buttons {
        clean,
        dirty,
        continue_after_game_over: Button::new(
            translation.restart,
            Anchor::center(width * 0.5, height * 0.7),
            font_size,
        ),
        buy: create_buy_hero_buttons(font_size, width, height, textures, translation),
        sell: create_sell_hero_buttons(font_size, width, height, textures, translation),
        continue_playing: Button::new(
            translation.continue_playing,
            Anchor::center(width * 0.5, height * 0.7),
            font_size,
        ),
        change_language_to_spanish: spanish,
        change_language_to_english: english,
        extra: create_extra_buttons(font_size, width, height, translation),
    }
}

fn create_clean_and_dirty_buttons(width: f32, height: f32) -> (TextureButton, TextureButton) {
    let size = (width * 0.1).min(height * 0.2);
    let size = Vec2::new(size, size);
    let clean_pos = Anchor::TopRight {
        x: width * (0.5 - 0.001),
        y: height * BUY_PANEL_START_HEIGHT,
    };
    let dirty_pos = Anchor::TopLeft {
        x: width * (0.5 + 0.001),
        y: height * BUY_PANEL_START_HEIGHT,
    };
    (
        TextureButton::new(clean_pos, size),
        TextureButton::new(dirty_pos, size),
    )
}

fn create_buy_hero_buttons(
    font_size: f32,
    width: f32,
    height: f32,
    textures: &Textures,
    translation: &Translation,
) -> HashMap<Hero, Button> {
    create_buy_or_sell_hero_buttons(font_size, width, height, textures, translation.buy, 0.02)
}

fn create_sell_hero_buttons(
    font_size: f32,
    width: f32,
    height: f32,
    textures: &Textures,
    translation: &Translation,
) -> HashMap<Hero, Button> {
    create_buy_or_sell_hero_buttons(font_size, width, height, textures, translation.sell, 0.1)
}

fn create_buy_or_sell_hero_buttons(
    font_size: f32,
    width: f32,
    height: f32,
    textures: &Textures,
    text: &str,
    extra_horizontal_offset: f32,
) -> HashMap<Hero, Button> {
    let mut buttons = HashMap::new();
    for (i, hero) in Hero::list().iter().enumerate() {
        let (horizontal_offset, vertical_offset) =
            TextureDrawer::get_buy_panel_offset(hero.index());
        let texture_offset = TextureDrawer::get_buy_text_offset_from_texture(
            hero.index(),
            width,
            height,
            textures.get(hero.texture_index()),
        );
        let x_coef =
            BUY_PANEL_HORIZONTAL_PAD + extra_horizontal_offset + horizontal_offset + texture_offset;
        let y_coef = BUY_PANEL_START_HEIGHT + 0.14 + vertical_offset;
        let font_size = font_size;
        let mut button = Button::new(
            text,
            Anchor::top_left(width * x_coef, height * y_coef),
            font_size,
        );

        let color = if i % 2 == 0 {
            CLEAN_BACKGROUND_COLOR
        } else {
            DIRTY_BACKGROUND_COLOR
        };
        button.set_color(color);
        buttons.insert(*hero, button);
    }
    buttons
}

fn create_extra_buttons(
    font_size: f32,
    _width: f32,
    height: f32,
    translation: &Translation,
) -> ExtraControls {
    let show_extra_controls = Button::new(
        translation.extra_controls,
        Anchor::bottom_left(0.0, height - BUTTON_PAD),
        font_size,
    );
    let next_button = |text, prev_rect: Rect| {
        let button = Button::new(
            text,
            Anchor::top_left(
                prev_rect.point().x + prev_rect.w + BUTTON_PAD,
                prev_rect.point().y,
            ),
            font_size,
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
