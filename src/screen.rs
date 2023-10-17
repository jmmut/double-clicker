use crate::screen::drawer_trait::DrawerTrait;
pub use crate::screen::gui_actions::GuiActions;
use crate::screen::input_source_trait::InputSourceTrait;
use crate::world::World;

pub mod drawer_trait;
mod gui_actions;
pub mod input_source_trait;
pub mod textures;
pub mod translations;

pub struct Screen {
    pub drawer: Box<dyn DrawerTrait>,
    pub input_source: Box<dyn InputSourceTrait>,
}

impl Screen {
    pub fn get_gui_actions(&mut self, _world: &World) -> GuiActions {
        let gui_actions = self.input_source.get_gui_actions(&mut *self.drawer);
        self.drawer.apply_gui_actions(&gui_actions);
        gui_actions
    }

    pub fn draw(&mut self, world: &mut World) {
        self.drawer.draw(world)
    }
}
