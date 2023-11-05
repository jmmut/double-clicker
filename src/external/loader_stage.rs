use macroquad::prelude::{
    clear_background, next_frame, screen_height, screen_width, trace, Color, FileError, BLACK,
    WHITE,
};

use crate::external::backends::factory;
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
        Self::draw_loading_text(progress, WHITE);
        trace!("painted frame of loading screen");
    }

    fn draw_loading_text(progress: Progress, color: Color) {
        TextRect::new(
            &format!(
                "Loading... ({}/{})",
                progress.loaded, progress.total_to_load
            ),
            Anchor::center(screen_width() * 0.5, screen_height() * 0.5),
            32.0,
        )
        .render_text(color);
    }
}
