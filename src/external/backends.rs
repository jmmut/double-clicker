use crate::external::basic_input::BasicInput;
use crate::external::text_drawer::TextDrawer;
use crate::screen::Screen;
use crate::world::World;

pub use macroquad::prelude::Vec2;

pub async fn factory() -> (Screen, World) {
    (
        Screen {
            drawer: Box::new(TextDrawer::new()),
            input_source: Box::new(BasicInput),
        },
        World::new(),
    )
}

pub type Seconds = f64;

pub fn now() -> Seconds {
    macroquad::miniquad::date::now()
}
