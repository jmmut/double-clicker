use crate::external::basic_drawer::BasicDrawer;
use crate::external::basic_input::BasicInput;
use crate::screen::Screen;
use crate::world::World;

pub async fn factory() -> (Screen, World) {
    (
        Screen {
            drawer: Box::new(BasicDrawer),
            input_source: Box::new(BasicInput),
        },
        World::new(),
    )
}

pub type Seconds = f64;

pub fn now() -> Seconds {
    macroquad::miniquad::date::now()
}
