use crate::external::backends::{now, Seconds};
use crate::screen::drawer_trait::DrawerTrait;
use crate::world::World;
use macroquad::prelude::*;
use macroquad::ui::root_ui;


pub struct BasicDrawer {
    frame: i64,
    previous_time: Seconds,
}

impl BasicDrawer {
    pub fn new() -> Self {
        Self {
            frame: 0,
            previous_time: now(),
        }
    }
    fn debug_fps(&mut self, world: &World) {
        let new_time = now();
        root_ui().label(None, &format!("now: {}", new_time));
        root_ui().label(None, &format!("drawing frame: {}", self.frame));
        root_ui().label(None, &format!("physics frame: {}", world.frame));
        let new_time = now();
        root_ui().label(
            None,
            &format!("drawing fps: {}", 1.0 / (new_time - self.previous_time)),
        );
        root_ui().label(
            None,
            &format!(
                "physics fps: {}",
                1.0 / (world.time_since_last_frame)
            ),
        );
        self.previous_time = new_time;

    }
}
impl DrawerTrait for BasicDrawer {
    fn draw(&mut self, world: &World) {
        self.frame += 1;
        self.debug_fps(world);
        root_ui().label(
            None,
            &format!(
                "remaining time until next payment: {}",
                &world.remaining_until_next_trigger
            ),
        );
        clear_background(LIGHTGRAY);
        draw_rectangle(100.0, 150.0, 10.0, 10.0, RED);

    }

}
