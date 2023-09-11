use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::GuiActions;

pub trait InputSourceTrait {
    fn get_gui_actions(&self, drawer: &dyn DrawerTrait) -> GuiActions;
}
