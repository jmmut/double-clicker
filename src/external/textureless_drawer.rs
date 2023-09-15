use crate::external::backends::{now, Seconds};
use crate::screen::drawer_trait::{Button, DrawerTrait};
use crate::world::heores::Hero;
use crate::world::{should_receive_payment, World, HERO_PRICE, SALARY};
use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

const EMPTY_COLOR: Color = GRAY;
const CLEAN_COLOR: Color = SKYBLUE;
const DIRTY_COLOR: Color = PURPLE;
const REWARDING_ZONE_COLOR: Color = Color::new(0.7, 0.8, 0.6, 0.9);
const FONT_SIZE: f32 = 16.0;

pub struct TexturelessDrawer {
    frame: i64,
    previous_time: Seconds,
    t: Option<Texture2D>,
    arrangement_index: usize,
}

#[derive(Copy, Clone, Debug)]
struct Arrangement {
    borders: bool,
    overlapping: bool,
}

#[rustfmt::skip]
const AVAILABLE_ARRANGEMENTS: [Arrangement; 4] = [
    Arrangement { borders: true, overlapping: false },
    Arrangement { borders: false, overlapping: false },
    Arrangement { borders: true, overlapping: true },
    Arrangement { borders: false, overlapping: true },
];

impl TexturelessDrawer {
    pub fn new() -> Self {
        Self {
            frame: 0,
            previous_time: now(),
            t: None,
            arrangement_index: 0,
        }
    }
    pub fn new_with_texture(t: Texture2D) -> Self {
        Self {
            frame: 0,
            previous_time: now(),
            t: Some(t),
            arrangement_index: 0,
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

    fn draw_bar_and_money(&self, world: &World, width: f32, height: f32) {
        let Arrangement {
            borders,
            overlapping,
        } = AVAILABLE_ARRANGEMENTS[self.arrangement_index];

        draw_bar(world, width, height, overlapping, borders);
        draw_salary(world, width, height, overlapping);
        draw_savings(world, width, height, overlapping);
        draw_cleaned(world, width, height, overlapping);
        draw_dirtied(world, width, height, overlapping);
    }

    fn draw_buy_heroes(&self, world: &World, width: f32, height: f32) {
        let button_height = height * 0.15;
        let button_width = width * 0.25;
        draw_rectangle(
            width * 0.05,
            height * 0.4,
            button_width,
            button_height,
            CLEAN_COLOR,
        );
        draw_rectangle_lines(
            width * 0.05,
            height * 0.4,
            button_width,
            button_height,
            2.0,
            BLACK,
        );
        draw_line(
            width * 0.05,
            height * 0.4 + FONT_SIZE * 1.2,
            width * 0.05 + button_width,
            height * 0.4 + FONT_SIZE * 1.2,
            1.0,
            BLACK,
        );
        root_ui().label(
            Vec2::new(width * 0.06, height * 0.4),
            &format!("{}: Limpia 10 tareas", Hero::Hero1.name()),
        );
        root_ui().label(
            Vec2::new(width * 0.06, height * 0.4 + FONT_SIZE * 1.2),
            &format!(
                "Tienes: {}. Precio: {}",
                world.heroes_count[&Hero::Hero1],
                HERO_PRICE
            ),
        )
    }
}

impl DrawerTrait for TexturelessDrawer {
    fn draw(&mut self, world: &World) {
        self.frame += 1;
        // self.debug_fps(world);
        clear_background(LIGHTGRAY);
        let width = screen_width();
        let height = screen_height();
        self.draw_bar_and_money(world, width, height);
        self.draw_buy_heroes(world, width, height);
    }

    fn button(&self, button: Button) -> bool {
        let width = screen_width();
        let height = screen_height();
        let is_button_clicked = |x_coef: f32, y_coef: f32, label: &str| -> bool {
            return root_ui().button(Some(Vec2::new(width * x_coef, height * y_coef)), label);
        };
        match button {
            Button::Clean => is_button_clicked(0.4, 0.25, "Limpiar"),
            Button::Dirty => is_button_clicked(0.52, 0.25, "Ensuciar"),
            Button::Arrangement => is_button_clicked(0.45, 0.85, "Cambiar estilo"),
            Button::Buy(Hero::Hero1) => is_button_clicked(0.10, 0.5, "Comprar"),
            Button::Sell(Hero::Hero1) => is_button_clicked(0.20, 0.5, "Vender"),
            _ => false,
        }
    }

    fn next_arrangement(&mut self) {
        self.arrangement_index += 1;
        self.arrangement_index %= AVAILABLE_ARRANGEMENTS.len();
        info!(
            "using arrangement {}: {:?}",
            self.arrangement_index, AVAILABLE_ARRANGEMENTS[self.arrangement_index]
        );
    }
}

fn draw_bar(world: &World, width: f32, height: f32, overlapping: bool, borders: bool) {
    let bar_width = 0.8;
    let bar_height = if overlapping { 0.15 } else { 0.05 };

    draw_rectangle(
        width * 0.1,
        height * 0.05,
        width * bar_width,
        height * bar_height,
        EMPTY_COLOR,
    );

    let empty = world.cleaned + world.dirtied == 0;
    let border = 0.1
        + if !empty {
            let cleanliness =
                bar_width * world.cleaned as f32 / (world.cleaned + world.dirtied) as f32;
            let dirtiness =
                bar_width * world.dirtied as f32 / (world.cleaned + world.dirtied) as f32;
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
            cleanliness
        } else {
            0.0
        };
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

    if borders {
        draw_rectangle_lines(
            width * 0.1,
            height * 0.05,
            width * bar_width,
            height * bar_height,
            2.0,
            BLACK,
        );
        if !empty {
            draw_line(
                width * border,
                height * 0.05,
                width * border,
                height * (0.05 + bar_height),
                1.0,
                BLACK,
            )
        }
    }
}
fn draw_salary(world: &World, width: f32, height: f32, overlapping: bool) {
    let vertical_offset = if overlapping { 0.0 } else { 0.05 };
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
        (height * (0.1 + vertical_offset)).round(),
        font_size,
        BLACK,
    );
}

fn draw_savings(world: &World, width: f32, height: f32, overlapping: bool) {
    let vertical_offset = if overlapping { 0.0 } else { 0.05 };
    let font_size = FONT_SIZE;
    let money_text = format!("Ahorros: {} €", world.money);
    let money_size = measure_text(&money_text, None, font_size as u16, 1.0);
    // root_ui().label(Some(Vec2::new(width * 0.5 - money_size.width * 0.5, height * 0.1 - money_size.height)), &money_text);
    draw_text(
        &money_text,
        width * 0.5 - (money_size.width * 0.5).round(),
        (height * (0.15 + vertical_offset)).round(),
        font_size,
        BLACK,
    );
}

fn draw_cleaned(world: &World, width: f32, height: f32, overlapping: bool) {
    let vertical_offset = if overlapping { 0.0 } else { 0.05 };
    let cleaned_str = format!("Tareas de limpieza: {}", world.cleaned);
    draw_text(
        &cleaned_str,
        width * 0.15,
        (height * (0.12 + vertical_offset)).round(),
        FONT_SIZE,
        BLACK,
    );
}

fn draw_dirtied(world: &World, width: f32, height: f32, overlapping: bool) {
    let vertical_offset = if overlapping { 0.0 } else { 0.05 };
    let dirtied_str = format!("Tareas de suciedad: {}", world.dirtied);
    draw_text(
        &dirtied_str,
        width * 0.65,
        (height * (0.12 + vertical_offset)).round(),
        FONT_SIZE,
        BLACK,
    );
}
