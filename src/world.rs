use crate::screen::GuiActions;

pub struct World;

impl World {
    pub fn update(&self, gui_actions: GuiActions) -> bool {
        gui_actions.should_continue()
    }
}
