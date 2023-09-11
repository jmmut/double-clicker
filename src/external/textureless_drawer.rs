use crate::external::backends::{now, Seconds};
use crate::screen::drawer_trait::{Button, DrawerTrait};
use crate::world::{should_receive_payment, World};
use macroquad::prelude::*;
use macroquad::ui::root_ui;

const CLEAN_COLOR: Color = SKYBLUE;
const DIRTY_COLOR: Color = PURPLE;

pub struct TexturelessDrawer {
    frame: i64,
    previous_time: Seconds,
}

impl TexturelessDrawer {
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

impl DrawerTrait for TexturelessDrawer {
    fn draw(&mut self, world: &World) {
        self.frame += 1;
        // self.debug_fps(world);
        clear_background(LIGHTGRAY);
        let width = screen_width();
        let height = screen_height();
        draw_rectangle(width * 0.1, height * 0.05, width * 0.8, height * 0.1, CLEAN_COLOR);
        let dirtiness = world.dirtied as f32 / (world.cleaned + world.dirtied + 1) as f32;
        let dirtiness_screen = dirtiness * 0.8;
        draw_rectangle(width * (0.9 - dirtiness_screen), height * 0.05, width * dirtiness_screen, height * 0.1, DIRTY_COLOR);
    }

    fn button(&self, button: Button) -> bool {
        let width = screen_width();
        let height = screen_height();
        match button {
            Button::Clean => root_ui().button(Some(Vec2::new(width * 0.4, height * 0.2)), "Limpiar"),
            Button::Dirty => root_ui().button(Some(Vec2::new(width * 0.52, height * 0.2)), "Ensuciar"),
        }
    }
}
