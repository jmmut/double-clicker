use macroquad::prelude::{load_texture, trace, FilterMode, Texture2D};

pub enum Texture {
    CleanBackground = 0,
    CleanBackgroundOff = 1,
    DirtyBackground = 2,
    DirtyBackgroundOff = 3,
    CleanFgBroom = 4,
    DirtyFgFish = 5,
    Hero1 = 6,
    Villain1 = 7,
}

pub async fn load_textures() -> Vec<Texture2D> {
    trace!("before loading");
    let mut textures = Vec::new();
    for path in [
        "assets/images/buttons/buttonBLU-256-yes.png",
        "assets/images/buttons/buttonBLU-256-no.png",
        "assets/images/buttons/buttonPUR-256-yes.png",
        "assets/images/buttons/buttonPUR-256-no.png",
        "assets/images/buttons/buttonBLU-1.png",
        "assets/images/buttons/buttonPUR-1.png",
        "assets/images/characters/heroe1.png",
        "assets/images/characters/villano1.png",
    ] {
        let t = load_texture(path).await.unwrap();
        t.set_filter(FilterMode::Linear);
        textures.push(t);
    }
    trace!("after loading");
    textures
}
