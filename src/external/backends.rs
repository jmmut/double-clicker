use std::thread::sleep;
use std::time::Duration;
use macroquad::prelude::Texture2D;
use crate::external::basic_input::BasicInput;
use crate::external::text_drawer::TextDrawer;
use crate::screen::Screen;
use crate::world::World;

use crate::external::textureless_drawer::TexturelessDrawer;
pub use macroquad::prelude::Vec2;

pub fn factory(t: Texture2D) -> (Screen, World) {
    // sleep(Duration::from_secs(5));
    (
        Screen {
            drawer: Box::new(TexturelessDrawer::new_with_texture(t)),
            input_source: Box::new(BasicInput),
        },
        World::new(),
    )
}

pub type Seconds = f64;

pub fn now() -> Seconds {
    macroquad::miniquad::date::now()
}
