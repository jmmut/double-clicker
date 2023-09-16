use macroquad::prelude::{load_texture, trace, FilterMode, Texture2D};
use std::slice::SliceIndex;

pub enum Texture {
    CleanBackground = 0,
    DirtyFgFish = 1,
}

pub async fn load_textures() -> Vec<Texture2D> {
    trace!("before loading");
    let mut textures = Vec::new();
    for path in [
        "assets/images/buttons/buttonBLU-256-yes.png",
        "assets/images/buttons/buttonPUR-1.png",
    ] {
        let mut t = load_texture(path).await.unwrap();
        t.set_filter(FilterMode::Nearest);
        textures.push(t);
    }
    trace!("after loading");
    textures
}
