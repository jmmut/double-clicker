use macroquad::prelude::{
    clear_background, draw_rectangle, draw_rectangle_lines, next_frame, screen_height,
    screen_width, trace, FileError, Rect, Vec2, BLACK, WHITE,
};

use crate::external::backends::factory;
use crate::external::texture_drawer::{CLEAN_COLOR, DIRTY_COLOR};
use crate::external::texture_loader::{Progress, TextureLoader};
use crate::external::widgets::anchor::Anchor;
use crate::external::widgets::text::TextRect;
use crate::screen::Screen;
use crate::world::World;

const TEXTURE_PATHS: &[&str] = &[
    "assets/images/buttons/buttonBLU-256-yes.png",
    "assets/images/buttons/buttonBLU-256-no.png",
    "assets/images/buttons/buttonPUR-256-yes.png",
    "assets/images/buttons/buttonPUR-256-no.png",
    "assets/images/characters/heroe1.png",
    "assets/images/characters/villano1.png",
    "assets/images/characters/heroe2.png",
    "assets/images/characters/villano2.png",
    "assets/images/characters/heroe3.png",
    "assets/images/characters/villano3.png",
    "assets/images/buttons/buttonBLU256-1.png",
    "assets/images/buttons/buttonBLU256-2.png",
    "assets/images/buttons/buttonBLU256-3.png",
    "assets/images/buttons/buttonPUR256-1.png",
    "assets/images/buttons/buttonPUR256-2.png",
    "assets/images/buttons/buttonPUR256-3.png",
    "assets/images/background/BGpatronsmol.png",
    "assets/images/background/BGmargin.png",
];
pub struct LoaderStage;

impl LoaderStage {
    pub async fn setup() -> Result<(Screen, World), FileError> {
        let mut loader = TextureLoader::new(TEXTURE_PATHS);
        loop {
            if let Some(textures) = loader.get_textures()? {
                return Ok(factory(textures));
            }
            Self::draw_loading(loader.get_progress());
            next_frame().await;
        }
    }

    fn draw_loading(progress: Progress) {
        clear_background(BLACK);
        let font_size = 32.0;
        let width = screen_width();
        let height = screen_height();
        let text_rect = TextRect::new(
            &format!(
                "Loading... ({}/{})",
                progress.loaded, progress.total_to_load
            ),
            Anchor::center(width * 0.5, height * 0.5),
            font_size,
        );
        text_rect.render_text(DIRTY_COLOR);

        let bar_width = width / 8.0;
        let rect = Rect::new(
            width * 0.5 - bar_width * 0.5,
            text_rect.rect.y + text_rect.rect.h + font_size * 0.5,
            bar_width * progress.loaded as f32 / progress.total_to_load as f32,
            font_size,
        );

        let thickness = 2.0;
        let pad = 3.0 + thickness;
        let line_rect = Rect::new(
            rect.x - pad,
            rect.y - pad,
            bar_width + 2.0 * pad,
            rect.h + 2.0 * pad,
        );

        draw_rectangle_lines(
            line_rect.x,
            line_rect.y,
            line_rect.w,
            line_rect.h,
            thickness,
            DIRTY_COLOR,
        );
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, CLEAN_COLOR);
    }
}
