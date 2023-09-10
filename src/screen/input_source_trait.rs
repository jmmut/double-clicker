use crate::screen::GuiActions;

pub trait InputSourceTrait {
    fn get_gui_actions(&self) -> GuiActions;
}
