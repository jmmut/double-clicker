use macroquad::prelude::*;

use crate::external::backends::{now, Seconds};
use crate::external::texture_drawer::buttons::Buttons;
use crate::external::texture_drawer::draw::draw_panel_border;
use crate::external::widgets::anchor::Anchor;
use crate::external::widgets::button::Interaction;
use crate::external::widgets::text::{
    draw_text_centered, draw_text_lines, draw_tooltip_centered, wrap_or_hide_text, TextRect,
};
use crate::screen::drawer_trait::{Button, DrawerTrait};
use crate::screen::textures::{Texture, Textures};
use crate::screen::translations::{get_translation, Language, Translation};
use crate::screen::GuiActions;
use crate::world::acts::Act;
use crate::world::heores::Hero;
use crate::world::{accumulate_price, World};
use crate::GIT_VERSION;

mod buttons;
pub mod draw;

pub const CLEAN_COLOR: Color = SKYBLUE;
pub const DIRTY_COLOR: Color = PURPLE;

const CLEAN_BACKGROUND_COLOR: Color = Color::new(0.75, 0.85, 1.0, 1.0);
const DIRTY_BACKGROUND_COLOR: Color = Color::new(0.85, 0.75, 1.0, 1.0);

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
    pub textures: Textures,
    arrangement_index: usize,
    clean_index: usize,
    dirty_index: usize,
    dirtiness: f32,
    buttons: Buttons,
    font_size: f32,
    stage: Act,
    width: f32,
    height: f32,
    translation: &'static Translation,
    extra_controls: bool,
    show_debug_fps: bool,
}

#[derive(Copy, Clone, Debug)]
struct Arrangement {
    overlapping: bool,
    transparency: bool,
}

#[rustfmt::skip]
const AVAILABLE_ARRANGEMENTS: &[Arrangement] = &[
    Arrangement { overlapping: true, transparency: false},
    Arrangement { overlapping: false, transparency: false },
    Arrangement { overlapping: true, transparency: true},
    Arrangement { overlapping: false, transparency: true },
];

impl TextureDrawer {
    pub fn new(textures: Textures, translation: &'static Translation) -> Self {
        Self::new_from_mocked(textures, screen_width(), screen_height(), translation)
    }
    pub fn new_from_mocked(
        textures: Textures,
        width: f32,
        height: f32,
        translation: &'static Translation,
    ) -> Self {
        let font_size = Self::choose_font_size(width, height);
        let buttons = buttons::create_buttons(font_size, width, height, &textures, translation);
        Self {
            frame: 0,
            previous_time: now(),
            textures,
            arrangement_index: 0,
            clean_index: 0,
            dirty_index: 0,
            dirtiness: 0.0,
            buttons,
            font_size,
            stage: Act::Act1,
            width,
            height,
            translation,
            extra_controls: false,
            show_debug_fps: false,
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
    fn recreate_buttons(&mut self) {
        self.buttons = buttons::create_buttons(
            self.font_size,
            self.width,
            self.height,
            &self.textures,
            self.translation,
        );
    }

    fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.font_size = Self::choose_font_size(width, height);
        self.recreate_buttons();
    }

    fn dirtiness_from_world(world: &World) -> f32 {
        world.dirtiness_units() as f32 / world.max_dirtiness_units() as f32
    }
}

impl DrawerTrait for TextureDrawer {
    fn draw(&mut self, world: &mut World) {
        self.frame += 1;
        self.stage = world.stage();
        self.dirtiness = Self::dirtiness_from_world(world);
        let width = screen_width();
        let height = screen_height();
        if width != self.width || height != self.height {
            self.resize(width, height);
            self.font_size = Self::choose_font_size(width, height);
        }

        let background_color = Self::get_background_color(self.dirtiness());
        clear_background(background_color);
        self.draw_background_pattern(width, height);
        self.draw_bar_and_money(world, width, height, self.font_size);
        self.draw_clean_and_dirty();
        self.draw_buy_heroes(world, width, height, self.font_size);
        draw_text_bar(
            world,
            width,
            height,
            self.font_size,
            now(),
            self.translation,
        );
        draw_alerts(world, width, height, self.font_size, self.translation);
        self.draw_game_over(world, width, height, self.font_size);
        self.draw_game_won(world, width, height, self.font_size);
        self.buttons
            .change_language_to_spanish
            .set_color(background_color)
            .render();
        self.buttons
            .change_language_to_english
            .set_color(background_color)
            .render();
        if self.show_debug_fps {
            self.debug_fps(&world, width, height)
        }
        let extra = &mut self.buttons.extra;
        if self.extra_controls {
            extra.show_debug_fps.set_color(background_color).render();
            extra.restart.set_color(background_color).render();
            extra
                .change_arrangement
                .set_color(background_color)
                .render();
            draw_version(width, height, self.font_size);
        }
        extra
            .show_extra_controls
            .set_color(background_color)
            .render();
    }

    fn button(&mut self, button: Button) -> bool {
        match button {
            Button::Clean => self.buttons.clean.interact().is_clicked(),
            Button::Dirty => self.buttons.dirty.interact().is_clicked(),
            Button::Arrangement => self
                .buttons
                .extra
                .change_arrangement
                .interact()
                .is_clicked(),
            Button::Restart => {
                if self.extra_controls {
                    let button = &mut self.buttons.extra.restart;
                    if button.interact().is_clicked() {
                        self.restart();
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Button::ContinuePlaying => {
                if self.stage == Act::GameWon {
                    self.buttons.continue_playing.interact().is_clicked()
                } else {
                    false
                }
            }
            Button::ContinueAfterGameOver => {
                if self.stage == Act::GameOver {
                    let interaction = &mut self.buttons.continue_after_game_over.interact();
                    if interaction.is_clicked() {
                        self.restart();
                    }
                    interaction.is_clicked()
                } else {
                    false
                }
            }
            Button::Buy(hero) => {
                let button = self.buttons.buy.get_mut(&hero).unwrap();
                button.interact().is_clicked()
            }
            Button::Sell(hero) => {
                let button = self.buttons.sell.get_mut(&hero).unwrap();
                button.interact().is_clicked()
            }
            Button::ChangeLanguageToSpanish => {
                let button = &mut self.buttons.change_language_to_spanish;
                let is_clicked = button.interact().is_clicked();
                if is_clicked {
                    self.translation = get_translation(Language::Spanish);
                    self.recreate_buttons();
                }
                is_clicked
            }
            Button::ChangeLanguageToEnglish => {
                let button = &mut self.buttons.change_language_to_english;
                let is_clicked = button.interact().is_clicked();
                if is_clicked {
                    self.translation = get_translation(Language::English);
                    self.recreate_buttons();
                }
                is_clicked
            }
            Button::DebugFps => {
                if self.extra_controls {
                    let button = &mut self.buttons.extra.show_debug_fps;
                    let is_clicked = button.interact().is_clicked();
                    if is_clicked {
                        self.show_debug_fps = !self.show_debug_fps;
                    }
                    is_clicked
                } else {
                    false
                }
            }
            Button::ExtraControls => {
                let button = &mut self.buttons.extra.show_extra_controls;
                let is_clicked = button.interact().is_clicked();
                if is_clicked {
                    self.extra_controls = !self.extra_controls;
                }
                is_clicked
            }
        }
    }

    fn apply_gui_actions(&mut self, gui_actions: &GuiActions) {
        if gui_actions.next_arrangement {
            self.next_arrangement();
        }
        if gui_actions.clean_pressed {
            self.next_clean();
        }
        if gui_actions.dirty_pressed {
            self.next_dirty();
        }
    }

    fn dirtiness(&self) -> f32 {
        self.dirtiness
    }
}

impl TextureDrawer {
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

    fn debug_fps(&mut self, world: &World, width: f32, _height: f32) {
        let new_time = now();
        let lines = vec![
            format!("now: {}", new_time),
            format!("drawing frame: {}", self.frame),
            format!("physics frame: {}", world.frame),
            format!("drawing fps: {:.2}", 1.0 / (new_time - self.previous_time)),
            format!("physics fps: {:.2}", 1.0 / (world.time_since_last_frame)),
        ];
        draw_rectangle(
            0.0,
            0.0,
            0.25 * width,
            self.font_size * (lines.len() as f32 + 0.5),
            Color::new(0.0, 0.0, 0.0, 0.5),
        );
        draw_text_lines(
            lines,
            Anchor::top_left(0.0, 0.0),
            self.font_size,
            self.font_size,
            WHITE,
        );
        self.previous_time = new_time;
    }

    fn restart(&mut self) {
        self.restart_mocked(screen_width(), screen_height())
    }

    fn restart_mocked(&mut self, width: f32, height: f32) {
        // apparently, rust is not clever enough to reuse the textures doing this:
        // *self = Self::new(self.textures);
        // my guess is that it's because the assignment to *self happens after taking self.textures,
        // during which self is incomplete/invalid. Workaround:
        let textures = std::mem::take(&mut self.textures);
        *self = Self::new_from_mocked(textures, width, height, self.translation);
    }

    pub fn get_background_color(dirtiness: f32) -> Color {
        // clear_background(Color::from_rgba(0x01, 0x00, 0x30, 255));
        // clear_background(Color::new(0.85, 0.75, 1.0, 1.0));
        // clear_background(Color::new(0x30, 0x00, 0x2f));

        let clean_color = CLEAN_BACKGROUND_COLOR.clone();
        let dirty_color = DIRTY_BACKGROUND_COLOR.clone();
        let bg_color = Color::new(
            (1.0 - dirtiness) * clean_color.r + dirtiness * dirty_color.r,
            (1.0 - dirtiness) * clean_color.g + dirtiness * dirty_color.g,
            (1.0 - dirtiness) * clean_color.b + dirtiness * dirty_color.b,
            (1.0 - dirtiness) * clean_color.a + dirtiness * dirty_color.a,
        );
        bg_color
    }

    fn draw_background_pattern(&mut self, width: f32, height: f32) {
        let target_ratio = 0.4 * height;
        let pattern_texture = self.textures.get(Texture::BackgroundPattern);
        let texture_size = Vec2::new(
            target_ratio * pattern_texture.width() / pattern_texture.height(),
            target_ratio,
        );
        let offset_x = 0.0;
        let period = 16.0;
        let time_dt = now() % period;
        let offset_y = -texture_size.y + (time_dt / period) as f32 * texture_size.y;

        for i_height in 0..=((height / texture_size.y).ceil() as i32) {
            for i_width in 0..((width / texture_size.x).ceil() as i32) {
                let x = texture_size.x * i_width as f32 + offset_x;
                let y = texture_size.y * i_height as f32 + offset_y;
                draw_texture_ex(
                    pattern_texture,
                    x,
                    y,
                    Color::new(1.0, 1.0, 1.0, 0.25),
                    DrawTextureParams {
                        dest_size: Some(texture_size),
                        source: None,
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    },
                );
            }
        }

        draw_texture_ex(
            self.textures.get(Texture::BackgroundMargin),
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(width, height)),
                ..Default::default()
            },
        );
    }

    fn draw_bar_and_money(&self, world: &World, width: f32, height: f32, font_size: f32) {
        let Arrangement {
            overlapping,
            transparency,
        } = AVAILABLE_ARRANGEMENTS[self.arrangement_index];

        draw_bar(world, width, height, overlapping);
        // draw_salary(world, width, height, overlapping);
        draw_savings(
            world,
            width,
            height,
            overlapping,
            transparency,
            font_size,
            self.translation,
        );
        draw_speeds(
            world,
            width,
            height,
            overlapping,
            transparency,
            font_size,
            self.translation,
        );
        // draw_dirtiness(
        //     world,
        //     width,
        //     height,
        //     overlapping,
        //     font_size,
        //     self.translation,
        // );
    }

    fn draw_clean_and_dirty(&self) {
        use Texture::*;
        self.buttons.clean.render(
            vec![
                self.textures.get(CleanBackground),
                self.textures.get(self.clean_texture()),
            ],
            Some(vec![
                self.textures.get(CleanBackgroundOff),
                self.textures.get(self.clean_texture()),
            ]),
        );
        self.buttons.dirty.render(
            vec![
                self.textures.get(DirtyBackground),
                self.textures.get(self.dirty_texture()),
            ],
            Some(vec![
                self.textures.get(DirtyBackgroundOff),
                self.textures.get(self.dirty_texture()),
            ]),
        );
    }

    fn draw_buy_heroes(&mut self, world: &World, width: f32, height: f32, font_size: f32) {
        let start_height = BUY_PANEL_START_HEIGHT;
        let panel_width = width * BUY_PANEL_WIDTH;
        let panel_height = height * BUY_PANEL_HEIGHT;
        for (i, hero) in Hero::list().iter().enumerate() {
            let (horizontal_offset, vertical_offset) = Self::get_buy_panel_offset(i);
            let panel_color = if i % 2 == 0 { CLEAN_COLOR } else { DIRTY_COLOR };
            let panel_rect = Rect::new(
                width * (BUY_PANEL_HORIZONTAL_PAD + horizontal_offset),
                height * (start_height + vertical_offset),
                panel_width,
                panel_height,
            );
            let (mouse_x, mouse_y) = mouse_position();

            // draw tooltip
            if panel_rect.contains(Vec2::new(mouse_x, mouse_y)) {
                let (horizontal_offset, vertical_offset) = Self::get_tooltip_offset(i);
                let pad_coef = 0.01;
                let tooltip_x_coef =
                    BUY_PANEL_HORIZONTAL_PAD + BUY_PANEL_WIDTH + pad_coef + horizontal_offset;
                let tooltip_y_coef = start_height + vertical_offset;
                draw_rectangle(
                    width * tooltip_x_coef,
                    height * tooltip_y_coef,
                    width * TOOLTIP_WIDTH,
                    panel_height,
                    panel_color,
                );
                let x = (width * (tooltip_x_coef + pad_coef)).round();
                let y = height * (tooltip_y_coef + pad_coef); // rounded later

                let (production, kind) = if i % 2 == 0 {
                    (
                        hero.production_clean() * world.heroes_count[hero],
                        self.translation.cleanings,
                    )
                } else {
                    (
                        hero.production_dirty() * world.heroes_count[hero],
                        self.translation.dirtyings,
                    )
                };

                let invested = format!(
                    "{} {} {} {} €",
                    self.translation.you_hired,
                    world.heroes_count[hero],
                    self.translation.investing,
                    accumulate_price(world.heroes_count[hero]) * hero.base_price() as f32
                );
                let speed = format!(
                    "{} {} {} {}",
                    self.translation.producing, production, kind, self.translation.per_second
                );
                let lines = vec![hero.short_description(self.translation), &invested, &speed];

                let line_height_coef = 1.1;
                for (i, line) in lines.iter().enumerate() {
                    draw_text(
                        line,
                        x,
                        (y + font_size * (1.0 + line_height_coef * i as f32)).round(),
                        font_size,
                        BLACK,
                    );
                }

                let line_y_in_panel = font_size * (0.5 + line_height_coef * lines.len() as f32);
                let line_y = y + line_y_in_panel;
                draw_line(
                    x,
                    line_y,
                    (width * (tooltip_x_coef + TOOLTIP_WIDTH - pad_coef)).round(),
                    line_y,
                    1.0,
                    BLACK,
                );
                let description = wrap_or_hide_text(
                    hero.long_description(self.translation),
                    font_size,
                    font_size * line_height_coef,
                    width * (TOOLTIP_WIDTH - 2.0 * pad_coef),
                    height * (BUY_PANEL_HEIGHT - 2.0 * pad_coef) - line_y_in_panel,
                );
                if is_mouse_button_pressed(MouseButton::Left) {
                    println!("{:?}", mouse_position());
                }
                // lines.append(&mut description.iter().map(|s| s.as_str()).collect());
                for (i, line) in description.iter().enumerate() {
                    draw_text(
                        line,
                        x,
                        (y + font_size
                            * (lines.len() as f32 * line_height_coef
                                + 1.5
                                + line_height_coef * i as f32))
                            .round(),
                        font_size,
                        BLACK,
                    );
                }
            }
            draw_rectangle(
                panel_rect.x,
                panel_rect.y,
                panel_rect.w,
                panel_rect.h,
                panel_color,
            );
            // draw_rectangle_lines(
            //     panel_rect.x,
            //     panel_rect.y,
            //     panel_rect.w,
            //     panel_rect.h,
            //     2.0,
            //     BLACK,
            // );
            draw_panel_border(panel_rect, Interaction::None);

            // draw_line(
            //     width * (0.05 + horizontal_offset),
            //     height * (start_height + vertical_offset) + font_size * 1.2,
            //     width * (0.05 + horizontal_offset) + button_width,
            //     height * (start_height + vertical_offset) + font_size * 1.2,
            //     1.0,
            //     BLACK,
            // );
            let character_texture = self.textures.get(hero.texture_index());
            let texture_size = Vec2::new(
                panel_rect.h * character_texture.width() / character_texture.height(),
                panel_rect.h,
            );
            let text_pos_x = (width * (BUY_PANEL_HORIZONTAL_PAD + 0.01 + horizontal_offset)
                + if i % 2 == 0 { 0.0 } else { texture_size.x })
            .round();

            let title_size = font_size * 1.25;
            draw_text(
                &hero.name(self.translation),
                (text_pos_x).round(),
                (height * (start_height + 0.01 + vertical_offset) + title_size).round(),
                title_size,
                BLACK,
            );
            draw_text(
                &hero.name(self.translation),
                (text_pos_x + 0.5).round(),
                (height * (start_height + 0.01 + vertical_offset) + title_size + 0.5).round(),
                title_size,
                with_alpha(BLACK, 0.3),
            );
            draw_text(
                &format!("{}: {} €", self.translation.price, world.price(hero)),
                text_pos_x,
                (height * (start_height + 0.01 + vertical_offset) + font_size * 3.0).round(),
                font_size,
                BLACK,
            );
            draw_text(
                &format!(
                    "{}: {} x {}",
                    if i % 2 == 0 {
                        self.translation.cleaning
                    } else {
                        self.translation.dirtying
                    },
                    if i % 2 == 0 {
                        hero.production_clean()
                    } else {
                        hero.production_dirty()
                    },
                    world.heroes_count[&hero],
                ),
                text_pos_x,
                (height * (start_height + 0.01 + vertical_offset) + font_size * 4.2).round(),
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
        for (_, button) in &self.buttons.buy {
            button.render();
        }
        for (_, button) in &self.buttons.sell {
            button.render();
        }
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
    fn get_buy_text_offset_from_texture(
        hero_index: usize,
        width: f32,
        height: f32,
        character_texture: Texture2D,
    ) -> f32 {
        let texture_offset = if hero_index % 2 == 0 {
            0.0
        } else {
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
        if world.stage() == Act::GameOver {
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
            draw_text_centered("GAME OVER", Vec2::new(0.5, 0.55), width, height, font_size);
            draw_text_centered(
                self.translation.over_greedy,
                Vec2::new(0.5, 0.61),
                width,
                height,
                font_size,
            );
            draw_text_centered(
                self.translation.owned_by_dirt,
                Vec2::new(0.5, 0.64),
                width,
                height,
                font_size,
            );
            self.buttons.continue_after_game_over.render();
        }
    }

    fn draw_game_won(&self, world: &mut World, width: f32, height: f32, font_size: f32) {
        if world.stage() == Act::GameWon {
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
                self.translation.you_won,
                Vec2::new(0.5, 0.55),
                width,
                height,
                font_size,
            );
            draw_text_centered(
                self.translation.retire,
                Vec2::new(0.5, 0.61),
                width,
                height,
                font_size,
            );
            draw_text_centered(
                self.translation.you_can_continue_playing,
                Vec2::new(0.5, 0.64),
                width,
                height,
                font_size,
            );
            self.buttons.continue_playing.render()
        }
    }
}

fn draw_bar(world: &World, width: f32, height: f32, overlapping: bool) {
    let bar_width = 1.0 - BAR_HORIZONTAL_PAD * 2.0;
    let bar_height = get_bar_height(overlapping);

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
    draw_panel_border(
        Rect::new(
            width * BAR_HORIZONTAL_PAD,
            height * BAR_VERTICAL_PAD,
            width * bar_width,
            height * bar_height,
        ),
        Interaction::None,
    );
}

fn get_bar_height(overlapping: bool) -> f32 {
    if overlapping {
        0.05 + BAR_VERTICAL_PAD
    } else {
        BAR_VERTICAL_PAD
    }
}

fn draw_savings(
    world: &World,
    width: f32,
    height: f32,
    overlapping: bool,
    transparency: bool,
    font_size: f32,
    translation: &Translation,
) {
    let vertical_offset = if overlapping {
        height * (BAR_VERTICAL_PAD * 2.0)
    } else {
        height * (BAR_VERTICAL_PAD * 3.0)
    };
    let savings_font_size = font_size * 2.0;
    let money_text = format!("{} €", world.money_euros());
    let money_rect = TextRect::new(
        &money_text,
        Anchor::center((width * 0.5).round(), vertical_offset.round()),
        savings_font_size,
    );

    if transparency {
        let bg_color =
            TextureDrawer::get_background_color(TextureDrawer::dirtiness_from_world(&world));
        draw_rectangle(
            money_rect.rect.x,
            money_rect.rect.y,
            money_rect.rect.w,
            money_rect.rect.h,
            with_alpha(bg_color, 0.5),
        )
    }

    money_rect.render_text(BLACK);
    let mut shadow = money_rect.clone();
    shadow.rect = shadow.rect.offset(Vec2::new(1.0, 1.0));
    shadow.render_text(with_alpha(BLACK, 0.5));

    let (mouse_x, mouse_y) = mouse_position();
    if money_rect.rect.contains(Vec2::new(mouse_x, mouse_y)) {
        let tooltip_text = translation.savings;
        let tooltip = TextRect::new(
            tooltip_text,
            Anchor::bottom_left(mouse_x, mouse_y),
            font_size,
        );
        draw_rectangle(
            tooltip.rect.x,
            tooltip.rect.y,
            tooltip.rect.w,
            tooltip.rect.h,
            LIGHTGRAY,
        );
        draw_rectangle_lines(
            tooltip.rect.x,
            tooltip.rect.y,
            tooltip.rect.w,
            tooltip.rect.h,
            2.0,
            BLACK,
        );
        tooltip.render_text(BLACK)
    }
}
fn draw_speeds(
    world: &World,
    width: f32,
    height: f32,
    overlapping: bool,
    transparency: bool,
    font_size: f32,
    translation: &Translation,
) {
    let vertical_offset = if overlapping {
        height * BAR_VERTICAL_PAD + font_size * 0.5
    } else {
        height * BAR_VERTICAL_PAD * 2.0 + font_size * 0.5
    };
    // let text_color = if overlapping { BLACK } else { WHITE };
    let text_color = BLACK;

    let mut speed = 0;
    for hero in [Hero::Hero1, Hero::Hero2, Hero::Hero3] {
        speed += hero.production_clean() * world.heroes_count[&hero];
    }
    let cleaning_text = format!("{}: {}", translation.cleaning_speed, speed);
    let text_pos = Anchor::top_left(
        (width * (BAR_HORIZONTAL_PAD)).round(),
        (vertical_offset).round(),
    );
    let text_rect = TextRect::new(&cleaning_text, text_pos, font_size);
    if !overlapping || transparency {
        draw_rectangle(
            text_rect.rect.x,
            text_rect.rect.y,
            text_rect.rect.w,
            text_rect.rect.h,
            if transparency {
                with_alpha(CLEAN_BACKGROUND_COLOR, 0.75)
            } else {
                CLEAN_BACKGROUND_COLOR
            },
        );
    }
    text_rect.render_text(text_color);

    let mut speed = 0;
    for hero in [Hero::Villain1, Hero::Villain2, Hero::Villain3] {
        speed += hero.production_dirty() * world.heroes_count[&hero];
    }
    let dirtiying_text = format!("{}: {}", translation.dirtying_speed, speed);
    let text_pos = Anchor::top_right(
        (width * (1.0 - BAR_HORIZONTAL_PAD)).round(),
        (vertical_offset).round(),
    );
    let text_rect = TextRect::new(&dirtiying_text, text_pos, font_size);
    if !overlapping || transparency {
        draw_rectangle(
            text_rect.rect.x,
            text_rect.rect.y,
            text_rect.rect.w,
            text_rect.rect.h,
            if transparency {
                with_alpha(DIRTY_BACKGROUND_COLOR, 0.75)
            } else {
                DIRTY_BACKGROUND_COLOR
            },
        );
    }
    text_rect.render_text(text_color);
}

fn with_alpha(mut color: Color, alpha: f32) -> Color {
    color.a = alpha;
    color
}

#[allow(unused)]
fn draw_dirtiness(
    world: &World,
    width: f32,
    height: f32,
    overlapping: bool,
    font_size: f32,
    translation: &Translation,
) {
    let vertical_offset = if overlapping { -0.025 } else { 0.05 };
    let dirtied_str = format!(
        "{}: {}/{}",
        translation.dirts,
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

fn draw_text_bar(
    world: &World,
    width: f32,
    height: f32,
    font_size: f32,
    now: Seconds,
    translation: &Translation,
) {
    let bar_height = BUY_PANEL_START_HEIGHT + 3.0 * (BUY_PANEL_HEIGHT + BUY_PANEL_VERTICAL_PAD);
    let dirtiness_coef = world.dirtiness_units() as f32 / world.max_dirtiness_units() as f32;
    draw_rectangle(
        width * 0.0,
        height * bar_height + 2.0,
        width,
        height - bar_height,
        TextureDrawer::get_background_color(dirtiness_coef),
    );
    draw_line(
        width * 0.0,
        height * bar_height + 2.0,
        width * 1.0,
        height * bar_height + 2.0,
        2.0,
        BLACK,
    );
    let text = choose_text_lore(world.stage(), now, translation);
    let wrapped_text = wrap_or_hide_text(text, font_size, font_size, width, height - bar_height);
    draw_text_lines(
        wrapped_text,
        Anchor::center(
            (width * 0.5).round(),
            (height * (bar_height + 0.01) + font_size).round(),
        ),
        font_size,
        font_size,
        BLACK,
    );
}

fn choose_text_lore(stage: Act, now: Seconds, translation: &Translation) -> &str {
    let lore_sentences = match stage {
        Act::Act1 => translation.lore.act_1,
        Act::Act2 => translation.lore.act_2,
        Act::Act3 => translation.lore.act_3,
        Act::GameOver => translation.lore.game_over,
        Act::GameWon => translation.lore.game_won,
        Act::ContinuePlayingAfterWinning => translation.lore.act_3,
    };
    *choose_pseudo_random(lore_sentences, now)
}

fn choose_pseudo_random<T>(collection: &[T], now: Seconds) -> &T {
    let persistence: Seconds = 15.0;
    let block = (now / persistence) as i64;
    let hash = block % 5 + 6 - block * 2 % 3 + block / 5;
    let index = hash as usize % collection.len();
    collection.get(index).unwrap()
}

fn draw_version(_width: f32, height: f32, font_size: f32) {
    let text_rect = TextRect::new(
        &format!("v{}", GIT_VERSION),
        Anchor::BottomLeft {
            x: -font_size,
            y: height - 1.5 * font_size,
        },
        font_size,
    );
    text_rect.render_text(BLACK);
}

fn draw_alerts(world: &World, width: f32, height: f32, font_size: f32, translation: &Translation) {
    for (i, (_, alert)) in world.alerts.iter().enumerate() {
        draw_tooltip_centered(
            &alert.to_string(translation),
            Vec2::new(width * 0.5, height * 0.5 + (i as f32 * 2.0 * font_size)),
            font_size,
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restart() {
        let mut textures = Vec::new();
        for _ in 0..=(Texture::Villain3 as usize) {
            let mut texture = miniquad::Texture::empty();
            texture.width = 100;
            texture.height = 200;
            textures.push(Texture2D::from_miniquad_texture(texture))
        }
        let mut drawer = TextureDrawer::new_from_mocked(
            Textures::new(textures.clone()),
            2000.0,
            1000.0,
            get_translation(Language::Spanish),
        );
        drawer.restart_mocked(2000.0, 1000.0);
        drawer.restart_mocked(2000.0, 1000.0);
    }
}
