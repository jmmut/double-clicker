use macroquad::prelude::*;
use macroquad::ui::root_ui;

use crate::external::backends::{now, Seconds};
use crate::screen::drawer_trait::{Button, DrawerTrait};
use crate::screen::textures::Texture;
use crate::world::heores::Hero;
use crate::world::{accumulate_price, World};
use crate::GIT_VERSION;

mod draw;

const CLEAN_COLOR: Color = SKYBLUE;
const DIRTY_COLOR: Color = PURPLE;
const FONT_SIZE: f32 = 16.0;

const BAR_HORIZONTAL_PAD: f32 = 0.04;
const BAR_VERTICAL_PAD: f32 = 0.05;

const SAVINGS_HEIGHT: f32 = 0.14;

const BUY_PANEL_START_HEIGHT: f32 = 0.22;
const BUY_PANEL_HEIGHT: f32 = 0.2;
const BUY_PANEL_WIDTH: f32 = 0.3;
const BUY_PANEL_HORIZONTAL_PAD: f32 = BAR_HORIZONTAL_PAD;
const BUY_PANEL_VERTICAL_PAD: f32 = 0.02;

const TOOLTIP_WIDTH: f32 = 0.3;

pub struct TextureDrawer {
    frame: i64,
    previous_time: Seconds,
    textures: Vec<Texture2D>,
    arrangement_index: usize,
    clean_index: usize,
    dirty_index: usize,
    buy_buttons: Vec<draw::Button>,
    continue_button: Option<draw::Button>,
    // game_won_button: Option<draw::Button>,
    font_size: f32,
    won: bool,
    continued: bool,
}

#[derive(Copy, Clone, Debug)]
struct Arrangement {
    overlapping: bool,
}

#[rustfmt::skip]
const AVAILABLE_ARRANGEMENTS: [Arrangement; 2] = [
    Arrangement { overlapping: false },
    Arrangement { overlapping: true },
];

impl TextureDrawer {
    pub fn new(textures: Vec<Texture2D>) -> Self {
        Self {
            frame: 0,
            previous_time: now(),
            textures,
            arrangement_index: 0,
            clean_index: 0,
            dirty_index: 0,
            buy_buttons: Vec::new(),
            continue_button: None,
            // game_won_button: None,
            font_size: Self::choose_font_size(screen_width(), screen_height()),
            won: false,
            continued: false,
        }
    }

    fn choose_font_size(width: f32, height: f32) -> f32 {
        let min_side = width.min(height * 16.0 / 9.0);
        FONT_SIZE
            * if min_side < 1600.0 {
                1.0
            } else if min_side < 2500.0 {
                1.5
            } else {
                2.0
            }
    }
}

impl DrawerTrait for TextureDrawer {
    fn draw(&mut self, world: &mut World) {
        self.frame += 1;
        if world.won() {
            self.won = true;
        }
        if world.continued() {
            self.continued = true;
        }
        // self.debug_fps(world);
        let width = screen_width();
        let height = screen_height();
        self.font_size = Self::choose_font_size(width, height);
        self.draw_bar_and_money(world, width, height, self.font_size);
        self.draw_buy_heroes(world, width, height, self.font_size);
        draw_text_bar(world, width, height, self.font_size);
        draw_version(width, height, self.font_size);
        draw_alerts(world, width, height, self.font_size);
        self.draw_game_over(world, width, height, self.font_size);
        self.draw_game_won(world, width, height, self.font_size);
    }

    fn button(&mut self, button: Button) -> bool {
        let width = screen_width();
        let height = screen_height();
        let is_texture_clicked = |rect, texture: Texture, texture_pressed: Option<Texture>| {
            draw::is_texture_clicked(
                rect,
                self.textures[texture as usize],
                texture_pressed.map(|t| self.textures[t as usize]),
            )
        };
        use Texture::*;
        match button {
            Button::Clean => {
                let size = (width * 0.1).min(height * 0.2);
                let rect = Rect::new(
                    width * (0.5 - 0.001) - size,
                    height * BUY_PANEL_START_HEIGHT,
                    size,
                    size,
                );
                is_texture_clicked(rect, CleanBackground, Some(CleanBackgroundOff));
                is_texture_clicked(rect, self.clean_texture(), None)
            }
            Button::Dirty => {
                let size = (width * 0.1).min(height * 0.2);
                let rect = Rect::new(
                    width * (0.5 + 0.001),
                    height * BUY_PANEL_START_HEIGHT,
                    size,
                    size,
                );
                is_texture_clicked(rect, DirtyBackground, Some(DirtyBackgroundOff));
                is_texture_clicked(rect, self.dirty_texture(), None)
            }
            Button::Arrangement => root_ui().button(None, "Cambiar estilo"),
            Button::Restart => {
                if root_ui().button(None, "Reiniciar") {
                    self.restart();
                    true
                } else {
                    false
                }
            }
            Button::ContinuePlaying => {
                if self.won && !self.continued {
                    let mut button = draw::Button::from_center_pos("Continuar jugando", Vec2::new(width * 0.5, height * 0.7), self.font_size);
                    let interaction = button.interact();
                    self.continue_button = Some(button);
                    interaction.is_clicked()
                } else {
                    false
                }
            }
            Button::Buy(hero) => {
                let (horizontal_offset, vertical_offset) =
                    TextureDrawer::get_buy_panel_offset(hero.index());
                let texture_offset = self.get_buy_text_offset(hero.index(), width, height);
                self.is_button_clicked(
                    BUY_PANEL_HORIZONTAL_PAD + 0.02 + horizontal_offset + texture_offset,
                    BUY_PANEL_START_HEIGHT + 0.12 + vertical_offset,
                    "Comprar",
                    width,
                    height,
                    self.font_size,
                )
            }
            Button::Sell(hero) => {
                let (horizontal_offset, vertical_offset) =
                    TextureDrawer::get_buy_panel_offset(hero.index());
                let texture_offset = self.get_buy_text_offset(hero.index(), width, height);
                self.is_button_clicked(
                    BUY_PANEL_HORIZONTAL_PAD + 0.1 + horizontal_offset + texture_offset,
                    BUY_PANEL_START_HEIGHT + 0.12 + vertical_offset,
                    "Vender",
                    width,
                    height,
                    self.font_size,
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

    fn next_clean(&mut self) {
        self.clean_index = (self.clean_index + 1) % 3;
    }

    fn next_dirty(&mut self) {
        self.dirty_index = (self.dirty_index + 1) % 3;
    }
}

impl TextureDrawer {
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

    fn restart(&mut self) {
        // apparently, rust is not clever enough to reuse the textures doing this:
        // *self = Self::new(self.textures);
        // my guess is that it's because the assignment to *self happens after taking self.textures,
        // during which self is incomplete/invalid. Workaround:
        let mut dummy = Self::new(Vec::new());
        std::mem::swap(&mut dummy, self);
        *self = Self::new(dummy.textures);
    }

    fn is_button_clicked(
        &mut self,
        x_coef: f32,
        y_coef: f32,
        label: &str,
        width: f32,
        height: f32,
        font_size: f32,
    ) -> bool {
        let mut button = draw::Button::from_top_left_pos(
            label,
            Vec2::new(width * x_coef, height * y_coef),
            font_size,
        );
        let interaction = button.interact();
        self.buy_buttons.push(button);
        interaction.is_clicked()
        // return root_ui().button(Some(Vec2::new(width * x_coef, height * y_coef)), label);
    }

    fn draw_bar_and_money(&self, world: &World, width: f32, height: f32, font_size: f32) {
        let Arrangement { overlapping } = AVAILABLE_ARRANGEMENTS[self.arrangement_index];

        draw_bar(world, width, height, overlapping);
        // draw_salary(world, width, height, overlapping);
        draw_savings(world, width, height, overlapping, font_size);
        draw_speeds(world, width, height, overlapping, font_size);
        draw_dirtiness(world, width, height, overlapping, font_size);
    }

    fn draw_buy_heroes(&mut self, world: &World, width: f32, height: f32, font_size: f32) {
        let start_height = BUY_PANEL_START_HEIGHT;
        let button_width = width * BUY_PANEL_WIDTH;
        let button_height = height * BUY_PANEL_HEIGHT;
        for (i, hero) in Hero::list().iter().enumerate() {
            let (horizontal_offset, vertical_offset) = Self::get_buy_panel_offset(i);
            let panel_color = if i % 2 == 0 { CLEAN_COLOR } else { DIRTY_COLOR };
            let panel_rect = Rect::new(
                width * (BUY_PANEL_HORIZONTAL_PAD + horizontal_offset),
                height * (start_height + vertical_offset),
                button_width,
                button_height,
            );
            let (mouse_x, mouse_y) = mouse_position();

            // draw tooltip
            if panel_rect.contains(Vec2::new(mouse_x, mouse_y)) {
                let (horizontal_offset, vertical_offset) = Self::get_tooltip_offset(i);

                draw_rectangle(
                    width * (BUY_PANEL_HORIZONTAL_PAD + BUY_PANEL_WIDTH + 0.01 + horizontal_offset),
                    height * (start_height + vertical_offset),
                    width * TOOLTIP_WIDTH,
                    button_height,
                    panel_color,
                );
                draw_text(
                    &hero.short_description(),
                    (width
                        * (BUY_PANEL_HORIZONTAL_PAD
                            + BUY_PANEL_WIDTH
                            + 0.01
                            + 0.01
                            + horizontal_offset))
                        .round(),
                    (height * (start_height + 0.01 + vertical_offset) + font_size).round(),
                    font_size,
                    BLACK,
                );

                let (production, kind) = if i % 2 == 0 {
                    (
                        hero.production_clean() * world.heroes_count[hero],
                        "limpiezas",
                    )
                } else {
                    (
                        hero.production_dirty() * world.heroes_count[hero],
                        "suciedades",
                    )
                };
                draw_text(
                    &format!(
                        "Has contratado {} invirtiendo {} €",
                        world.heroes_count[hero],
                        accumulate_price(world.heroes_count[hero]) * hero.base_price() as f32
                    ),
                    (width
                        * (BUY_PANEL_HORIZONTAL_PAD
                            + BUY_PANEL_WIDTH
                            + 0.01
                            + 0.01
                            + horizontal_offset))
                        .round(),
                    (height * (start_height + 0.01 + vertical_offset) + font_size * 2.2).round(),
                    font_size,
                    BLACK,
                );
                draw_text(
                    &format!("Produciendo {} {} por segundo", production, kind),
                    (width
                        * (BUY_PANEL_HORIZONTAL_PAD
                            + BUY_PANEL_WIDTH
                            + 0.01
                            + 0.01
                            + horizontal_offset))
                        .round(),
                    (height * (start_height + 0.01 + vertical_offset) + font_size * 3.4).round(),
                    font_size,
                    BLACK,
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
            // draw_line(
            //     width * (0.05 + horizontal_offset),
            //     height * (start_height + vertical_offset) + font_size * 1.2,
            //     width * (0.05 + horizontal_offset) + button_width,
            //     height * (start_height + vertical_offset) + font_size * 1.2,
            //     1.0,
            //     BLACK,
            // );
            let character_texture = self.textures[Texture::Hero1 as usize + i];
            let texture_size = Vec2::new(
                panel_rect.h * character_texture.width() / character_texture.height(),
                panel_rect.h,
            );
            let text_pos_x = (width * (BUY_PANEL_HORIZONTAL_PAD + 0.01 + horizontal_offset)
                + if i % 2 == 0 { 0.0 } else { texture_size.x })
            .round();

            draw_text(
                &hero.name(),
                text_pos_x,
                (height * (start_height + 0.01 + vertical_offset) + font_size).round(),
                font_size,
                BLACK,
            );
            draw_text(
                &format!("Precio: {} €", world.price(hero)),
                text_pos_x,
                (height * (start_height + 0.01 + vertical_offset) + font_size * 2.2).round(),
                font_size,
                BLACK,
            );
            draw_text(
                &format!(
                    "{}: {} x {}",
                    if i % 2 == 0 {
                        "Limpiando"
                    } else {
                        "Ensuciando"
                    },
                    if i % 2 == 0 {
                        hero.production_clean()
                    } else {
                        hero.production_dirty()
                    },
                    world.heroes_count[&hero],
                ),
                text_pos_x,
                (height * (start_height + 0.01 + vertical_offset) + font_size * 3.4).round(),
                font_size,
                BLACK,
            );
            let texture_x = if i % 2 == 0 {
                panel_rect.x + panel_rect.w - texture_size.x
            } else {
                panel_rect.x
            };
            let texture_rect = Rect::new(texture_x, panel_rect.y, texture_size.x, texture_size.y);
            draw::is_texture_clicked(texture_rect, character_texture, Some(character_texture));
        }
        for button in &self.buy_buttons {
            button.render();
        }
        self.buy_buttons.clear();
    }

    /// Returns coefficients [0, 1] that you have to multiply by screen_width and screen_height.
    fn get_buy_panel_offset(hero_index: usize) -> (f32, f32) {
        let horizontal_offset = if hero_index % 2 == 0 {
            0.0
        } else {
            1.0 - 2.0 * BAR_HORIZONTAL_PAD - BUY_PANEL_WIDTH
        };
        let vertical_offset = (hero_index / 2) as f32 * (BUY_PANEL_HEIGHT + BUY_PANEL_VERTICAL_PAD);
        (horizontal_offset, vertical_offset)
    }
    fn get_buy_text_offset(&self, hero_index: usize, width: f32, height: f32) -> f32 {
        let texture_offset = if hero_index % 2 == 0 {
            0.0
        } else {
            let character_texture = self.textures[Texture::Villain1 as usize];
            BUY_PANEL_HEIGHT * height * character_texture.width()
                / character_texture.height()
                / width
        };
        texture_offset
    }

    /// Returns coefficients [0, 1] that you have to multiply by screen_width and screen_height.
    fn get_tooltip_offset(hero_index: usize) -> (f32, f32) {
        let (horizontal_button_offset, vertical_offset) = Self::get_buy_panel_offset(hero_index);
        let horizontal_offset = if hero_index % 2 == 0 {
            0.0
        } else {
            horizontal_button_offset - BUY_PANEL_WIDTH - TOOLTIP_WIDTH - 0.02
        };
        (horizontal_offset, vertical_offset)
    }

    fn clean_texture(&self) -> Texture {
        use Texture::*;
        [CleanFgBroom, CleanFgSpray, CleanFgSponge][self.clean_index]
    }
    fn dirty_texture(&self) -> Texture {
        use Texture::*;
        [DirtyFgFish, DirtyFgBanana, DirtyFgCigar][self.dirty_index]
    }

    fn draw_game_over(&mut self, world: &mut World, width: f32, height: f32, font_size: f32) {
        if world.game_over {
            let text_rect = Rect::new(
                (width * 0.35).round(),
                (height * 0.5).round(),
                (width * 0.3).round(),
                (height * 0.25).round(),
            );
            draw_rectangle(
                text_rect.x,
                text_rect.y,
                text_rect.w,
                text_rect.h,
                Color::new(0.7, 0.7, 0.7, 1.00),
            );
            draw_rectangle_lines(
                text_rect.x,
                text_rect.y,
                text_rect.w,
                text_rect.h,
                2.0,
                BLACK,
            );
            draw_text_centered("GAME OVER", Vec2::new(0.5, 0.57), width, height, font_size);
            draw_text_centered(
                "Te has pasado de avaricioso.",
                Vec2::new(0.5, 0.64),
                width,
                height,
                font_size,
            );
            draw_text_centered(
                "La suciedad se ha apoderado de ti.",
                Vec2::new(0.5, 0.67),
                width,
                height,
                font_size,
            );
            let mut button = draw::Button::from_center_pos(
                "Reiniciar",
                Vec2::new(width * 0.5, height * 0.7),
                font_size,
            );
            if button.interact().is_clicked() {
                self.restart();
                world.restart(); // TODO: move this somehow to basic_input
            }
            button.render();
        }
    }

fn draw_game_won(&self, world: &mut World, width: f32, height: f32, font_size: f32) {
    if world.won() && !world.continued() {
        let text_rect = Rect::new(
            (width * 0.35).round(),
            (height * 0.5).round(),
            (width * 0.3).round(),
            (height * 0.25).round(),
        );
        draw_rectangle(
            text_rect.x,
            text_rect.y,
            text_rect.w,
            text_rect.h,
            Color::new(0.7, 0.7, 0.7, 1.00),
        );
        draw_rectangle_lines(
            text_rect.x,
            text_rect.y,
            text_rect.w,
            text_rect.h,
            2.0,
            BLACK,
        );
        draw_text_centered(
            "Has ganado!",
            Vec2::new(0.5, 0.57),
            width,
            height,
            font_size,
        );
        draw_text_centered(
            "Tienes bastante dinero para jubilarte.",
            Vec2::new(0.5, 0.64),
            width,
            height,
            font_size,
        );
        draw_text_centered(
            "Puedes seguir jugando si quieres.",
            Vec2::new(0.5, 0.67),
            width,
            height,
            font_size,
        );
        if let Some(button) = self.continue_button.as_ref() {
            button.render();
        }
    }
}

}

fn draw_bar(world: &World, width: f32, height: f32, overlapping: bool) {
    let bar_width = 1.0 - BAR_HORIZONTAL_PAD * 2.0;
    let bar_height = if overlapping {
        0.1 + BAR_VERTICAL_PAD
    } else {
        BAR_VERTICAL_PAD
    };

    draw_rectangle(
        width * BAR_HORIZONTAL_PAD,
        height * BAR_VERTICAL_PAD,
        width * bar_width,
        height * bar_height,
        CLEAN_COLOR,
    );
    let dirtiness_coef = world.dirtiness_units() as f32 / world.max_dirtiness_units() as f32;
    draw_rectangle(
        width * (1.0 - BAR_HORIZONTAL_PAD - bar_width * dirtiness_coef),
        height * BAR_VERTICAL_PAD,
        width * bar_width * dirtiness_coef,
        height * bar_height,
        DIRTY_COLOR,
    );
    draw_rectangle_lines(
        width * BAR_HORIZONTAL_PAD,
        height * BAR_VERTICAL_PAD,
        width * bar_width,
        height * bar_height,
        2.0,
        BLACK,
    );
}

fn draw_savings(world: &World, width: f32, height: f32, overlapping: bool, font_size: f32) {
    let vertical_offset = if overlapping { 0.0 } else { 0.05 };
    let font_size = font_size * 2.0;
    let money_text = format!("{} €", world.money_euros());
    let money_size = measure_text(&money_text, None, font_size as u16, 1.0);
    let text_rect = Rect::new(
        width * 0.5 - (money_size.width * 0.5).round(),
        (height * (SAVINGS_HEIGHT + vertical_offset)).round(),
        money_size.width,
        money_size.height,
    );
    // root_ui().label(Some(Vec2::new(width * 0.5 - money_size.width * 0.5, height * 0.1 - money_size.height)), &money_text);
    // draw_text(
    //     &money_text,
    //     width * 0.5 - (money_size.width * 0.5).round() + 1.0,
    //     (height * (0.15 + vertical_offset)).round() + 1.0,
    //     font_size,
    //     WHITE,
    // );
    // draw_text(
    //     &money_text,
    //     width * 0.5 - (money_size.width * 0.5).round() + 1.0,
    //     (height * (0.15 + vertical_offset)).round() - 1.0,
    //     font_size,
    //     WHITE,
    // );
    // draw_text(
    //     &money_text,
    //     width * 0.5 - (money_size.width * 0.5).round() - 1.0,
    //     (height * (0.15 + vertical_offset)).round() + 1.0,
    //     font_size,
    //     WHITE,
    // );
    // draw_text(
    //     &money_text,
    //     width * 0.5 - (money_size.width * 0.5).round() - 1.0,
    //     (height * (0.15 + vertical_offset)).round() - 1.0,
    //     font_size,
    //     WHITE,
    // );
    draw_text(
        &money_text,
        text_rect.x - 1.0,
        text_rect.y - 1.0,
        font_size,
        WHITE,
    );
    draw_text(
        &money_text,
        text_rect.x + 1.0,
        text_rect.y + 1.0,
        font_size,
        WHITE,
    );
    draw_text(&money_text, text_rect.x, text_rect.y, font_size, BLACK);

    let text_top_left = Rect {
        y: text_rect.y - text_rect.h,
        ..text_rect
    };
    let (mouse_x, mouse_y) = mouse_position();
    if text_top_left.contains(Vec2::new(mouse_x, mouse_y)) {
        let pad = font_size * 0.5;
        let tooltip_text = "Ahorros";
        let tooltip_dimensions = measure_text(tooltip_text, None, font_size as u16, 1.0);
        draw_rectangle(
            mouse_x,
            mouse_y - tooltip_dimensions.height - pad * 2.0,
            tooltip_dimensions.width + pad * 2.0,
            tooltip_dimensions.height + pad * 2.0,
            LIGHTGRAY,
        );
        draw_rectangle_lines(
            mouse_x,
            mouse_y - tooltip_dimensions.height - pad * 2.0,
            tooltip_dimensions.width + pad * 2.0,
            tooltip_dimensions.height + pad * 2.0,
            2.0,
            BLACK,
        );
        draw_text(
            tooltip_text,
            (mouse_x + pad).round(),
            (mouse_y - pad).round(),
            font_size,
            BLACK,
        );
    }
}
fn draw_speeds(world: &World, width: f32, height: f32, overlapping: bool, font_size: f32) {
    let vertical_offset = if overlapping { 0.0 } else { 0.05 };
    let mut speed = 0;
    for hero in [Hero::Hero1, Hero::Hero2, Hero::Hero3] {
        speed += hero.production_clean() * world.heroes_count[&hero];
    }
    let cleaning_text = format!("Velocidad de limpieza: {}", speed);
    let text_pos = Vec2::new(
        (width * BAR_HORIZONTAL_PAD + font_size).round(),
        (height * (SAVINGS_HEIGHT + vertical_offset)).round(),
    );
    draw_text(&cleaning_text, text_pos.x, text_pos.y, font_size, BLACK);

    let mut speed = 0;
    for hero in [Hero::Villain1, Hero::Villain2, Hero::Villain3] {
        speed += hero.production_dirty() * world.heroes_count[&hero];
    }
    let dirtiying_text = format!("Velocidad de ensuciamiento: {}", speed);
    let text_size = measure_text(&dirtiying_text, None, font_size as u16, 1.0);

    let text_pos = Vec2::new(
        (width * (1.0 - BAR_HORIZONTAL_PAD) - text_size.width - font_size).round(),
        (height * (SAVINGS_HEIGHT + vertical_offset)).round(),
    );
    draw_text(&dirtiying_text, text_pos.x, text_pos.y, font_size, BLACK);
}

fn draw_dirtiness(world: &World, width: f32, height: f32, overlapping: bool, font_size: f32) {
    let vertical_offset = if overlapping { 0.0 } else { 0.05 };
    let dirtied_str = format!(
        "Suciedades: {}/{}",
        world.dirtiness_units(),
        world.max_dirtiness_units()
    );
    let text_size = measure_text(&dirtied_str, None, font_size as u16, 1.0);
    draw_text(
        &dirtied_str,
        (width * (1.0 - BAR_HORIZONTAL_PAD) - text_size.width - font_size).round(),
        (height * (SAVINGS_HEIGHT - 0.03 + vertical_offset)).round(),
        font_size,
        BLACK,
    );
}

fn draw_text_bar(_world: &World, width: f32, height: f32, font_size: f32) {
    let bar_height = BUY_PANEL_START_HEIGHT + 3.0 * (BUY_PANEL_HEIGHT + BUY_PANEL_VERTICAL_PAD);
    draw_line(
        width * 0.0,
        height * bar_height + 2.0,
        width * 1.0,
        height * bar_height + 2.0,
        2.0,
        BLACK,
    );
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    let dimensions = measure_text(text, None, font_size as u16, 1.0);
    draw_text(
        text,
        (width * 0.5 - dimensions.width * 0.5).round(),
        (height * (bar_height + 0.01) + dimensions.offset_y).round(),
        font_size,
        BLACK,
    );
}

fn draw_version(_width: f32, height: f32, font_size: f32) {
    root_ui().label(
        Vec2::new(0.0, height - font_size),
        &format!("v{}", GIT_VERSION),
    );
}

fn draw_alerts(world: &World, width: f32, height: f32, font_size: f32) {
    for (i, (_, alert)) in world.alerts.iter().enumerate() {
        draw_tooltip_centered(
            &alert.to_string(),
            Vec2::new(0.5, 0.5 + (i as f32 * 2.0 * font_size) / height),
            width,
            height,
            font_size,
        );
    }
}
fn draw_tooltip_centered(text: &str, position: Vec2, width: f32, height: f32, font_size: f32) {
    let pad = font_size * 0.5;
    let tooltip_size = measure_text(&text, None, font_size as u16, 1.0);
    let text_rect = Rect::new(
        (width * position.x - tooltip_size.width * 0.5 - pad).round(),
        (height * position.y - tooltip_size.height - pad * 2.0).round(),
        tooltip_size.width + pad * 2.0,
        tooltip_size.height + pad * 2.0,
    );
    draw_rectangle(
        text_rect.x,
        text_rect.y,
        text_rect.w,
        text_rect.h,
        LIGHTGRAY,
    );
    draw_rectangle_lines(
        text_rect.x,
        text_rect.y,
        text_rect.w,
        text_rect.h,
        2.0,
        BLACK,
    );
    draw_text(
        &text,
        text_rect.x + pad,
        text_rect.y + pad + tooltip_size.offset_y,
        font_size,
        BLACK,
    );
}

fn draw_text_centered(text: &str, position: Vec2, width: f32, height: f32, font_size: f32) {
    let pad = font_size * 0.5;
    let tooltip_size = measure_text(&text, None, font_size as u16, 1.0);
    let text_rect = Rect::new(
        (width * position.x - tooltip_size.width * 0.5 - pad).round(),
        (height * position.y - tooltip_size.height - pad * 2.0).round(),
        tooltip_size.width + pad * 2.0,
        tooltip_size.height + pad * 2.0,
    );
    draw_text(
        &text,
        text_rect.x + pad,
        text_rect.y + pad + tooltip_size.offset_y,
        font_size,
        BLACK,
    );
}
