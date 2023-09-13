use crate::screen::Screen;
use crate::world::World;

pub mod screen;
pub mod world;
pub mod external {
    pub mod backends;
    pub mod basic_input;
    pub mod text_drawer;
    pub mod textureless_drawer;
}

/// returns if should continue looping. In other words, if there should be another future frame.
pub fn frame(screen: &mut Screen, world: &mut World) -> bool {
    let gui_actions = screen.get_gui_actions(world);
    let should_continue = world.update(gui_actions);
    screen.draw(world);
    should_continue
}
