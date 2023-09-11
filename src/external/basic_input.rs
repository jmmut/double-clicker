use crate::screen::input_source_trait::InputSourceTrait;
use crate::screen::GuiActions;
use macroquad::input::is_key_pressed;
use macroquad::prelude::KeyCode;
use macroquad::ui::root_ui;
use crate::screen::drawer_trait::{Button, DrawerTrait};

pub struct BasicInput;

impl InputSourceTrait for BasicInput {
    fn get_gui_actions(&self, drawer: &dyn DrawerTrait) -> GuiActions {
        let dirty_pressed = root_ui().button(drawer.get_button_pos(Button::Dirty), "Ensuciar")
            || is_key_pressed(KeyCode::E);
        let clean_pressed = root_ui().button(drawer.get_button_pos(Button::Clean), "Limpiar")
            || is_key_pressed(KeyCode::L);
        GuiActions {
            quit: is_key_pressed(KeyCode::Escape),
            clean_pressed,
            dirty_pressed,
        }
    }
}
