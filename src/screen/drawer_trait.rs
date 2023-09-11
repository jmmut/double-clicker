use crate::external::backends::Vec2;
use crate::world::World;

pub enum Button {
    Clean,
    Dirty,
}

pub trait DrawerTrait {
    fn draw(&mut self, world: &World);

    /// Returns true if the button was pressed this frame
    fn button(&self, button: Button) -> bool;
}
