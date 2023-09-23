use macroquad::prelude::*;
use macroquad::ui::root_ui;

use crate::external::backends::{now, Seconds};
use crate::screen::drawer_trait::{Button, DrawerTrait};
use crate::world::heores::Hero;
use crate::world::{World, HERO_PRICE};
use crate::GIT_VERSION;

const EMPTY_COLOR: Color = GRAY;
const CLEAN_COLOR: Color = SKYBLUE;
const DIRTY_COLOR: Color = PURPLE;
const REWARDING_ZONE_COLOR: Color = Color::new(0.7, 0.8, 0.6, 0.9);
const FONT_SIZE: f32 = 16.0;

const BUY_BUTTON_START_HEIGHT: f32 = 0.3;
const BUY_BUTTON_HEIGHT: f32 = 0.15;
const BUY_BUTTON_WIDTH: f32 = 0.25;
const TOOLTIP_WIDTH: f32 = 0.3;

pub struct TexturelessDrawer {
    frame: i64,
    previous_time: Seconds,
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
        let start_height = 0.3;
        let button_width = width * BUY_BUTTON_WIDTH;
        let button_height = height * BUY_BUTTON_HEIGHT;
        for (i, hero) in Hero::list().iter().enumerate() {
            let (horizontal_offset, vertical_offset) = Self::get_buy_button_offset(i);
            let panel_color = if i % 2 == 0 { CLEAN_COLOR } else { DIRTY_COLOR };
            let panel_rect = Rect::new(
                width * (0.05 + horizontal_offset),
                height * (start_height + vertical_offset),
                button_width,
                button_height,
            );
            let (mouse_x, mouse_y) = mouse_position();
            if panel_rect.contains(Vec2::new(mouse_x, mouse_y)) {
                let (horizontal_offset, vertical_offset) = Self::get_tooltip_offset(i);

                draw_rectangle(
                    width * (0.05 + BUY_BUTTON_WIDTH + 0.01 + horizontal_offset),
                    height * (start_height + vertical_offset),
                    width * TOOLTIP_WIDTH,
                    button_height,
                    panel_color,
                );

                root_ui().label(
                    Vec2::new(
                        width * (0.05 + BUY_BUTTON_WIDTH + 0.01 + 0.01 + horizontal_offset),
                        height * (start_height + 0.01 + vertical_offset),
                    ),
                    &hero.short_description(),
                );
                let (production, kind) = if i % 2 == 0 {
                    (
                        hero.production_clean() * world.heroes_count[hero] as i64,
                        "limpiezas",
                    )
                } else {
                    (
                        hero.production_dirty() * world.heroes_count[hero] as i64,
                        "suciedades",
                    )
                };
                root_ui().label(
                    Vec2::new(
                        width * (0.05 + BUY_BUTTON_WIDTH + 0.01 + 0.01 + horizontal_offset),
                        height * (start_height + 0.01 + vertical_offset) + FONT_SIZE * 1.2,
                    ),
                    &format!("Produciendo {} {} por segundo", production, kind),
                );
            }
            draw_rectangle(
                panel_rect.x,
                panel_rect.y,
                panel_rect.w,
                panel_rect.h,
                panel_color,
            );
            draw_rectangle_lines(
                panel_rect.x,
                panel_rect.y,
                panel_rect.w,
                panel_rect.h,
                2.0,
                BLACK,
            );
            draw_line(
                width * (0.05 + horizontal_offset),
                height * (start_height + vertical_offset) + FONT_SIZE * 1.2,
                width * (0.05 + horizontal_offset) + button_width,
                height * (start_height + vertical_offset) + FONT_SIZE * 1.2,
                1.0,
                BLACK,
            );
            let text_pos_x = width * (0.06 + horizontal_offset);
            root_ui().label(
                Vec2::new(text_pos_x, height * (start_height + vertical_offset)),
                &hero.name(),
            );
            root_ui().label(
                Vec2::new(
                    text_pos_x,
                    height * (start_height + vertical_offset) + FONT_SIZE * 1.2,
                ),
                &format!(
                    "Tienes: {}. Precio: {}",
                    world.heroes_count[&hero], HERO_PRICE
                ),
            );
        }
    }

    /// Returns coefficients [0, 1] that you have to multiply by screen_width and screen_height.
    fn get_buy_button_offset(hero_index: usize) -> (f32, f32) {
        let horizontal_offset = if hero_index % 2 == 0 { 0.0 } else { 0.65 };
        let vertical_offset = (hero_index / 2) as f32 * (BUY_BUTTON_HEIGHT + 0.05);
        (horizontal_offset, vertical_offset)
    }

    /// Returns coefficients [0, 1] that you have to multiply by screen_width and screen_height.
    fn get_tooltip_offset(hero_index: usize) -> (f32, f32) {
        let (horizontal_button_offset, vertical_offset) = Self::get_buy_button_offset(hero_index);
        let horizontal_offset = if hero_index % 2 == 0 {
            0.0
        } else {
            horizontal_button_offset - BUY_BUTTON_WIDTH - TOOLTIP_WIDTH - 0.02
        };
        (horizontal_offset, vertical_offset)
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
        draw_text_bar(world, width, height);
        draw_version(width, height);
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
            Button::Arrangement => root_ui().button(None, "Cambiar estilo"),
            Button::Restart => root_ui().button(None, "Reiniciar"),
            Button::Buy(hero) => {
                let (horizontal_offset, vertical_offset) =
                    TexturelessDrawer::get_buy_button_offset(hero.index());
                is_button_clicked(
                    0.10 + horizontal_offset,
                    BUY_BUTTON_START_HEIGHT + 0.1 + vertical_offset,
                    "Comprar",
                )
            }
            Button::Sell(hero) => {
                let (horizontal_offset, vertical_offset) =
                    TexturelessDrawer::get_buy_button_offset(hero.index());
                is_button_clicked(
                    0.20 + horizontal_offset,
                    BUY_BUTTON_START_HEIGHT + 0.1 + vertical_offset,
                    "Vender",
                )
            }
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

fn draw_text_bar(_world: &World, width: f32, height: f32) {
    draw_line(
        width * 0.0,
        height * 0.9,
        width * 1.0,
        height * 0.9,
        2.0,
        BLACK,
    );
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    let dimensions = measure_text(text, None, FONT_SIZE as u16, 1.0);
    root_ui().label(
        Vec2::new(
            width * 0.5 - (dimensions.width * 0.5).round(),
            height * (0.9 + 0.01),
        ),
        text,
    );
}

fn draw_version(_width: f32, height: f32) {
    root_ui().label(
        Vec2::new(0.0, height - FONT_SIZE),
        &format!("v{}", GIT_VERSION),
    );
}
