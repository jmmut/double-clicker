use macroquad::prelude::{load_texture, trace, FilterMode, Texture2D};

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
    trace!("before loading");
    let mut textures = Vec::new();
    for path in [
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
    ] {
        let t = load_texture(path).await.unwrap();
        t.set_filter(FilterMode::Linear);
        textures.push(t);
    }
    trace!("after loading");
    textures
}
