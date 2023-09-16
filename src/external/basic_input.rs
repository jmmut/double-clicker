use macroquad::input::is_key_pressed;
use macroquad::prelude::KeyCode;
use std::collections::HashMap;

use crate::screen::drawer_trait::{Button, DrawerTrait};
use crate::screen::input_source_trait::InputSourceTrait;
use crate::screen::GuiActions;
use crate::world::heores::Hero;

pub struct BasicInput;

impl InputSourceTrait for BasicInput {
    fn get_gui_actions(&self, drawer: &dyn DrawerTrait) -> GuiActions {
        let dirty_pressed = drawer.button(Button::Dirty) || is_key_pressed(KeyCode::E);
        let clean_pressed = drawer.button(Button::Clean) || is_key_pressed(KeyCode::L);
        let next_arrangement = drawer.button(Button::Arrangement) || is_key_pressed(KeyCode::C);
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
        GuiActions {
            quit: is_key_pressed(KeyCode::Escape),
            clean_pressed,
            dirty_pressed,
            next_arrangement,
            heroes_bought,
            heroes_sold,
        }
    }
}
