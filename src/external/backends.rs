use macroquad::prelude::Texture2D;
pub use macroquad::prelude::Vec2;

use crate::external::basic_input::BasicInput;
use crate::external::texture_drawer::TextureDrawer;
use crate::screen::textures::Textures;
use crate::screen::translations::{get_translation, Language};
use crate::screen::Screen;
use crate::world::World;

const DEFAULT_LANGUAGE: Language = Language::Spanish;
pub fn factory(textures: Vec<Texture2D>) -> (Screen, World) {
    (
        Screen {
            drawer: Box::new(TextureDrawer::new(
                Textures::new(textures),
                get_translation(DEFAULT_LANGUAGE),
            )),
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
