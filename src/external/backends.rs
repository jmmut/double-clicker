use crate::external::basic_input::BasicInput;
use crate::external::text_drawer::TextDrawer;
use crate::screen::Screen;
use crate::world::World;
use macroquad::prelude::Texture2D;

use crate::external::textureless_drawer::TexturelessDrawer;
pub use macroquad::prelude::Vec2;
use crate::external::texture_drawer::TextureDrawer;

pub fn factory(t: Texture2D) -> (Screen, World) {
    (
        Screen {
            drawer: Box::new(TextureDrawer::new_with_texture(t)),
            // drawer: Box::new(TexturelessDrawer::new_with_texture(t)),
            // drawer: Box::new(TextDrawer::new()),
            input_source: Box::new(BasicInput),
        },
        World::new(),
    )
}

pub type Seconds = f64;

pub fn now() -> Seconds {
    macroquad::miniquad::date::now()
}
