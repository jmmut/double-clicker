use crate::world::heores::Hero;
use crate::world::World;

pub enum Button {
    Clean,
    Dirty,
    Arrangement,
    Restart,
    ContinuePlaying,
    ContinueAfterGameOver,
    Buy(Hero),
    Sell(Hero),
}

pub trait DrawerTrait {
    fn draw(&mut self, world: &mut World); // TODO: remove mut for world

    /// Returns true if the button was pressed this frame
    fn button(&mut self, button: Button) -> bool;

    fn next_arrangement(&mut self) {}
    fn next_clean(&mut self) {}
    fn next_dirty(&mut self) {}
}
