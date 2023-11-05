use crate::external::backends::now;
use macroquad::prelude::*;
use macroquad::prelude::{load_texture, trace, FilterMode, Texture2D};
use macroquad::ui::root_ui;

#[derive(Copy, Clone)]
pub enum Texture {
    CleanBackground = 0,
    CleanBackgroundOff = 1,
    DirtyBackground = 2,
    DirtyBackgroundOff = 3,
    Hero1 = 4,
    Villain1 = 5,
    Hero2 = 6,
    Villain2 = 7,
    Hero3 = 8,
    Villain3 = 9,
    CleanFgBroom = 10,
    CleanFgSpray = 11,
    CleanFgSponge = 12,
    DirtyFgFish = 13,
    DirtyFgBanana = 14,
    DirtyFgCigar = 15,
    BackgroundPattern = 16,
    BackgroundMargin = 17,
}

pub struct Textures {
    inner: Vec<Texture2D>,
}
impl Textures {
    pub fn new(textures: Vec<Texture2D>) -> Self {
        Self { inner: textures }
    }
    pub fn get(&self, texture: Texture) -> Texture2D {
        self.inner[texture as usize]
    }
}
impl Default for Textures {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

pub async fn load_textures() -> Vec<Texture2D> {
    let start = now();
    trace!("before loading at {}", start);
    let mut textures = Vec::new();
    let texture_paths = [
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
    for (i, path) in texture_paths.iter().enumerate() {
        let t = load_texture(path).await.unwrap();
        t.set_filter(FilterMode::Linear);
        textures.push(t);

        clear_background(LIGHTGRAY);
        root_ui().label(None, &format!("Loading... ({}/{})", i, texture_paths.len()));
        trace!("painted frame of loading screen");
        next_frame().await;
    }

    // TODO: remove this after testing the loading screen
    textures.clear();
    for (i, path) in texture_paths.iter().enumerate() {
        let t = load_texture(path).await.unwrap();
        t.set_filter(FilterMode::Linear);
        textures.push(t);

        clear_background(LIGHTGRAY);
        root_ui().label(None, &format!("Loading... ({}/{})", i, texture_paths.len()));
        trace!("painted frame of loading screen");
        next_frame().await;
    }
    textures.clear();
    for (i, path) in texture_paths.iter().enumerate() {
        let t = load_texture(path).await.unwrap();
        t.set_filter(FilterMode::Linear);
        textures.push(t);

        clear_background(LIGHTGRAY);
        root_ui().label(None, &format!("Loading... ({}/{})", i, texture_paths.len()));
        trace!("painted frame of loading screen");
        next_frame().await;
    }
    //TODO: until here

    let end = now();
    trace!("after loading at {}, took {:.3}s", end, end - start);
    textures
}
