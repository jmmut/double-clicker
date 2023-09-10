use crate::screen::input_source_trait::InputSourceTrait;
use crate::screen::GuiActions;
use macroquad::input::is_key_pressed;
use macroquad::prelude::KeyCode;
use macroquad::ui::root_ui;

pub struct BasicInput;

impl InputSourceTrait for BasicInput {
    fn get_gui_actions(&self) -> GuiActions {
        let dirty_pressed = root_ui().button(None, "Ensuciar")
            || is_key_pressed(KeyCode::E);
        let clean_pressed = root_ui().button(None, "Limpiar")
            || is_key_pressed(KeyCode::L);
        GuiActions {
            quit: is_key_pressed(KeyCode::Escape),
            clean_pressed,
            dirty_pressed,
        }
    }
}
