use git_version::git_version;

use crate::screen::Screen;
use crate::world::World;

pub mod screen;
pub mod world;
pub mod external {
    pub mod backends;
    pub mod basic_input;
    // pub mod text_drawer;
    pub mod texture_drawer;
    // pub mod textureless_drawer;
    pub mod loader_stage;
    pub mod texture_loader;
    pub mod widgets {
        pub mod anchor;
        pub mod button;
        pub mod text;
        pub mod texture_button;
    }
}

pub const GIT_VERSION: &str = git_version!(args = ["--tags"]);

/// returns if should continue looping. In other words, if there should be another future frame.
pub fn frame(screen: &mut Screen, world: &mut World) -> bool {
    let gui_actions = screen.get_gui_actions(world);
    let should_continue = world.update(gui_actions);
    screen.draw(world);
    should_continue
}
