use crate::external::backends::Vec2;
use crate::world::World;

pub enum Button {
    Clean,
    Dirty,
}

pub trait DrawerTrait {
    fn draw(&mut self, world: &World);

    fn get_button_pos(&self, button: Button) -> Option<Vec2>;
}
