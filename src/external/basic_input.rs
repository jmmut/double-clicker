use std::collections::HashMap;

use macroquad::input::is_key_pressed;
use macroquad::prelude::{clear_background, draw_texture_ex, Color, KeyCode, LIGHTGRAY};

use crate::screen::drawer_trait::{Button, DrawerTrait};
use crate::screen::input_source_trait::InputSourceTrait;
use crate::screen::GuiActions;
use crate::world::heores::Hero;

const CLEAN_BACKGROUND_COLOR: Color = Color::new(0.75, 0.85, 1.0, 1.0);
const DIRTY_BACKGROUND_COLOR: Color = Color::new(0.85, 0.75, 1.0, 1.0);

pub struct BasicInput;

impl InputSourceTrait for BasicInput {
    fn get_gui_actions(&self, drawer: &mut dyn DrawerTrait) -> GuiActions {

        let dirtiness = drawer.dirtiness();
        let bg_color = get_background_color(dirtiness);
        // clear_background(Color::from_rgba(0x01, 0x00, 0x30, 255)); // TODO: remove this shit out of here. blocked until root_ui is not used.
        clear_background(bg_color); // TODO: remove this shit out of here. blocked until root_ui is not used.
        // clear_background(Color::new(0.85, 0.75, 1.0, 1.0)); // TODO: remove this shit out of here. blocked until root_ui is not used.
        // clear_background(Color::new(0x30, 0x00, 0x2f)); // TODO: remove this shit out of here. blocked until root_ui is not used.


        let dirty_pressed = drawer.button(Button::Dirty) || is_key_pressed(KeyCode::E);
        let clean_pressed = drawer.button(Button::Clean) || is_key_pressed(KeyCode::L);
        let next_arrangement = drawer.button(Button::Arrangement) || is_key_pressed(KeyCode::C);
        let restart =
            drawer.button(Button::Restart) || drawer.button(Button::ContinueAfterGameOver);
        let continue_playing = drawer.button(Button::ContinuePlaying);
        let heroes_bought = HashMap::from_iter(
            Hero::list()
                .iter()
                .map(|hero| (*hero, drawer.button(Button::Buy(*hero)))),
        );
        let heroes_sold = HashMap::from_iter(
            Hero::list()
                .iter()
                .map(|hero| (*hero, drawer.button(Button::Sell(*hero)))),
        );

        drawer.button(Button::ChangeLanguageToSpanish);
        drawer.button(Button::ChangeLanguageToEnglish);
        drawer.button(Button::DebugFps);
        drawer.button(Button::ExtraControls);
        drawer.button(Button::Restart);

        GuiActions {
            quit: is_key_pressed(KeyCode::Escape),
            clean_pressed,
            dirty_pressed,
            next_arrangement,
            restart,
            continue_playing,
            heroes_bought,
            heroes_sold,
        }
    }
}

pub fn get_background_color(dirtiness: f32) -> Color {
    let clean_color = CLEAN_BACKGROUND_COLOR.clone();
    let dirty_color = DIRTY_BACKGROUND_COLOR.clone();
    let bg_color = Color::new(
        ((1.0 - dirtiness) * clean_color.r + dirtiness * dirty_color.r),
        ((1.0 - dirtiness) * clean_color.g + dirtiness * dirty_color.g),
        ((1.0 - dirtiness) * clean_color.b + dirtiness * dirty_color.b),
        ((1.0 - dirtiness) * clean_color.a + dirtiness * dirty_color.a),
    );
    bg_color
}
