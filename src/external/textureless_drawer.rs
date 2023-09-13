use crate::external::backends::{now, Seconds};
use crate::screen::drawer_trait::{Button, DrawerTrait};
use crate::world::{should_receive_payment, World, SALARY};
use macroquad::prelude::*;
use macroquad::ui::root_ui;

const EMPTY_COLOR: Color = GRAY;
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
        draw_bar(world, width, height);
    }

    fn button(&self, button: Button) -> bool {
        let width = screen_width();
        let height = screen_height();
        match button {
            Button::Clean => {
                root_ui().button(Some(Vec2::new(width * 0.4, height * 0.25)), "Limpiar")
            }
            Button::Dirty => {
                root_ui().button(Some(Vec2::new(width * 0.52, height * 0.25)), "Ensuciar")
            }
        }
    }
}

fn draw_bar(world: &World, width: f32, height: f32) {
    let bar_width = 0.8;
    let bar_height = 0.15;
    draw_rectangle(
        width * 0.1,
        height * 0.05,
        width * bar_width,
        height * bar_height,
        EMPTY_COLOR,
    );

    let empty = world.cleaned + world.dirtied == 0;
    if !empty {
        let dirtiness = bar_width * world.dirtied as f32 / (world.cleaned + world.dirtied) as f32;
        let cleanliness = bar_width * world.cleaned as f32 / (world.cleaned + world.dirtied) as f32;
        draw_rectangle(
            width * 0.1,
            height * 0.05,
            width * cleanliness,
            height * bar_height,
            CLEAN_COLOR,
        );
        draw_rectangle(
            width * (0.9 - dirtiness),
            height * 0.05,
            width * dirtiness,
            height * bar_height,
            DIRTY_COLOR,
        );
    }
    let rewarding_zone_start = world.min_valid_percentage() as f32 / 100.0 * bar_width;
    let rewarding_zone_width =
        (world.max_valid_percentage() - world.min_valid_percentage()) as f32 / 100.0 * bar_width;
    draw_rectangle(
        width * (0.1 + rewarding_zone_start),
        height * 0.05,
        width * rewarding_zone_width,
        height * bar_height,
        REWARDING_ZONE_COLOR,
    );

    draw_salary(world, width, height);
    draw_savings(world, width, height);
    draw_cleaned(world, width, height);
    draw_dirtied(world, width, height);
}

fn draw_salary(world: &World, width: f32, height: f32) {
    let font_size = FONT_SIZE;
    let money_text = format!(
        "Salario (en {:.1}): {} €",
        world.remaining_until_next_trigger,
        world.expected_payment()
    );
    let money_size = measure_text(&money_text, None, font_size as u16, 1.0);
    // root_ui().label(Some(Vec2::new(width * 0.5 - money_size.width * 0.5, height * 0.1 - money_size.height)), &money_text);
    draw_text(
        &money_text,
        width * 0.5 - (money_size.width * 0.5).round(),
        (height * 0.1).round(),
        font_size,
        BLACK,
    );
}

fn draw_savings(world: &World, width: f32, height: f32) {
    let font_size = FONT_SIZE;
    let money_text = format!("Ahorros: {} €", world.money);
    let money_size = measure_text(&money_text, None, font_size as u16, 1.0);
    // root_ui().label(Some(Vec2::new(width * 0.5 - money_size.width * 0.5, height * 0.1 - money_size.height)), &money_text);
    draw_text(
        &money_text,
        width * 0.5 - (money_size.width * 0.5).round(),
        (height * 0.15).round(),
        font_size,
        BLACK,
    );
}

fn draw_cleaned(world: &World, width: f32, height: f32) {
    let cleaned_str = format!("Tareas de limpieza: {}", world.cleaned);
    draw_text(
        &cleaned_str,
        width * 0.15,
        (height * 0.12).round(),
        FONT_SIZE,
        BLACK,
    );
}

fn draw_dirtied(world: &World, width: f32, height: f32) {
    let dirtied_str = format!("Tareas de suciedad: {}", world.dirtied);
    draw_text(
        &dirtied_str,
        width * 0.65,
        (height * 0.12).round(),
        FONT_SIZE,
        BLACK,
    );
}
