use crate::external::backends::{now, Seconds};
use crate::screen::drawer_trait::{Button, DrawerTrait};
use crate::world::{should_receive_payment, World, SALARY};
use macroquad::prelude::*;
use macroquad::ui::root_ui;

const CLEAN_COLOR: Color = SKYBLUE;
const DIRTY_COLOR: Color = PURPLE;
const REWARDING_ZONE_COLOR: Color = Color::new(0.7, 0.8, 0.6, 0.75);
const FONT_SIZE: f32 = 16.0;

pub struct TexturelessDrawer {
    frame: i64,
    previous_time: Seconds,
    t: Option<Texture2D>,
}

impl TexturelessDrawer {
    pub fn new() -> Self {
        Self {
            frame: 0,
            previous_time: now(),
            t: None,
        }
    }
    pub fn new_with_texture(t: Texture2D) -> Self {
        Self {
            frame: 0,
            previous_time: now(),
            t: Some(t),
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
        draw_salary(world);
        draw_bar(world, width, height);
    }

    fn button(&self, button: Button) -> bool {
        let width = screen_width();
        let height = screen_height();
        match button {
            Button::Clean => {
                root_ui().button(Some(Vec2::new(width * 0.4, height * 0.2)), "Limpiar")
            }
            Button::Dirty => {
                root_ui().button(Some(Vec2::new(width * 0.52, height * 0.2)), "Ensuciar")
            }
        }
    }
}

fn draw_salary(world: &World) {
    root_ui().label(None, "Salario:");
    root_ui().label(None, &format!("{:.1} €", world.expected_payment()));
    root_ui().label(None, &format!("{:.1}", &world.remaining_until_next_trigger));
}

fn draw_bar(world: &World, width: f32, height: f32) {
    draw_rectangle(
        width * 0.1,
        height * 0.05,
        width * 0.8,
        height * 0.1,
        CLEAN_COLOR,
    );

    let dirtiness = if world.cleaned + world.dirtied == 0 {
        0.0
    } else {
        world.dirtied as f32 / (world.cleaned + world.dirtied) as f32
    };
    let dirtiness_screen = dirtiness * 0.8;
    draw_rectangle(
        width * (0.9 - dirtiness_screen),
        height * 0.05,
        width * dirtiness_screen,
        height * 0.1,
        DIRTY_COLOR,
    );
    let rewarding_zone_start = world.min_valid_percentage() as f32 / 100.0 * 0.8;
    let rewarding_zone_end =
        (world.max_valid_percentage() - world.min_valid_percentage()) as f32 / 100.0 * 0.8;
    draw_rectangle(
        width * (0.1 + rewarding_zone_start),
        height * 0.05,
        width * rewarding_zone_end,
        height * 0.1,
        REWARDING_ZONE_COLOR,
    );

    draw_money(world, width, height);
}

fn draw_money(world: &World, width: f32, height: f32) {
    let font_size = FONT_SIZE;
    let money_text = format!("{} €", world.money);
    let money_size = measure_text(&money_text, None, font_size as u16, 1.0);
    // root_ui().label(Some(Vec2::new(width * 0.5 - money_size.width * 0.5, height * 0.1 - money_size.height)), &money_text);
    draw_text(
        &money_text,
        width * 0.5 - (money_size.width * 0.5).round(),
        height * 0.1 + (money_size.height * 0.5).round(),
        font_size,
        BLACK,
    );
}
