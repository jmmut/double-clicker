use crate::screen::input_source_trait::InputSourceTrait;
use crate::screen::GuiActions;
use macroquad::input::is_key_pressed;
use macroquad::prelude::KeyCode;

pub struct BasicInput;

impl InputSourceTrait for BasicInput {
    fn get_gui_actions(&self) -> GuiActions {
        GuiActions {
            quit: is_key_pressed(KeyCode::Escape),
        }
    }
}
