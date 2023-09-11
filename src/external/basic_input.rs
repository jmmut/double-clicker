use macroquad::input::is_key_pressed;
use macroquad::prelude::KeyCode;

use crate::screen::drawer_trait::{Button, DrawerTrait};
use crate::screen::input_source_trait::InputSourceTrait;
use crate::screen::GuiActions;

pub struct BasicInput;

impl InputSourceTrait for BasicInput {
    fn get_gui_actions(&self, drawer: &dyn DrawerTrait) -> GuiActions {
        let dirty_pressed = drawer.button(Button::Dirty) || is_key_pressed(KeyCode::E);
        let clean_pressed = drawer.button(Button::Clean) || is_key_pressed(KeyCode::L);
        GuiActions {
            quit: is_key_pressed(KeyCode::Escape),
            clean_pressed,
            dirty_pressed,
        }
    }
}
