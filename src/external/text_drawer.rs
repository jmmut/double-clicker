use crate::external::backends::{now, Seconds};
use crate::screen::drawer_trait::{Button, DrawerTrait};
use crate::world::{should_receive_payment, World};
use macroquad::prelude::*;
use macroquad::ui::root_ui;

pub struct TextDrawer {
    frame: i64,
    previous_time: Seconds,
}

impl TextDrawer {
    pub fn new() -> Self {
        Self {
            frame: 0,
            previous_time: now(),
        }
    }

    #[allow(unused)]
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
            &format!("physics fps: {}", 1.0 / (world.time_since_last_frame)),
        );
        self.previous_time = new_time;
    }
}
impl DrawerTrait for TextDrawer {
    fn draw(&mut self, world: &World) {
        self.frame += 1;
        // self.debug_fps(world);
        clear_background(LIGHTGRAY);
        root_ui().label(
            None,
            &format!(
                "remaining time until next payment: {:.1}",
                &world.remaining_until_next_trigger
            ),
        );
        root_ui().label(None, &format!("cleaned: {}", &world.cleaned));
        root_ui().label(None, &format!("dirtied: {}", &world.dirtied));
        root_ui().label(
            None,
            &format!(
                "money: {}, will get paid? {}",
                &world.money,
                world.should_receive_payment()
            ),
        );
    }

    fn button(&self, button: Button) -> bool {
        match button {
            Button::Clean => root_ui().button(None, "Limpiar"),
            Button::Dirty => root_ui().button(None, "Ensuciar"),
        }
    }
}
