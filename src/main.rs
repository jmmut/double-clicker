use std::sync::{Arc, Mutex};
use std::time::Duration;

use macroquad::prelude::coroutines::start_coroutine;
use macroquad::prelude::*;
use macroquad::ui::root_ui;

use double_clicker::external::backends::{factory, now, Seconds};
use double_clicker::frame;
use double_clicker::screen::textures::load_textures;
use double_clicker::screen::Screen;
use double_clicker::world::World;

const DEFAULT_WINDOW_WIDTH: i32 = 1200;
const DEFAULT_WINDOW_HEIGHT: i32 = 675;
const DEFAULT_WINDOW_TITLE: &str = "Double Clicker";
const MAX_FPS: f64 = 80.0;
const FRAME_PERIOD: f64 = 1.0 / MAX_FPS;

#[macroquad::main(window_conf)]
async fn main() {
    let (mut screen, mut world) = load().await;
    let mut previous_time = now();
    while frame(&mut screen, &mut world) {
        sleep_until_next_frame(&mut previous_time).await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        high_dpi: true,
        ..Default::default()
    }
}


async fn load() -> (Screen, World) {
    #[cfg(not(target_family = "wasm"))]
    {
        let assets = Arc::new(Mutex::new(None));
        start_coroutine(save_texture(assets.clone()));
        let mut frames = 0;
        while assets.as_ref().lock().unwrap().is_none() {
            frames += 1;
            clear_background(LIGHTGRAY);
            root_ui().label(None, "Loading...");
            trace!("painted frame of loading screen");
            next_frame().await;
        }
        info!("loading took {} frames", frames);
        return factory(assets.as_ref().lock().unwrap().clone().unwrap());
    }

    #[cfg(target_family = "wasm")]
    {
        clear_background(LIGHTGRAY);
        root_ui().label(None, "Loading...");
        next_frame().await;
        trace!("before loading");
        let t = load_textures().await;
        trace!("after loading");
        // trace!("before sleeping");
        // wait_seconds(4.0).await;
        // trace!("after sleeping");
        trace!("moving to regular game loop");
        return factory(t);
    }
}

async fn save_texture(assets: Arc<Mutex<Option<Vec<Texture2D>>>>) {
    let textures = load_textures().await;
    // info!("before sleeping");
    // wait_seconds(4.0).await;
    // info!("after sleeping");
    *assets.as_ref().lock().unwrap() = Some(textures);
}

async fn sleep_until_next_frame(previous_time: &mut Seconds) {
    #[cfg(not(target_family = "wasm"))]
    {
        let new_time = now();
        // dbg!(new_time);
        // dbg!(*previous_time);
        let frame_duration = new_time - *previous_time;
        if frame_duration < FRAME_PERIOD {
            let sleep_secs = FRAME_PERIOD - frame_duration;
            // info!("sleeping for {}", sleep_secs);

            // this is a blocking sleep on purpose. My current understanding is that macroquad
            // relies on OS or GPU drivers to limit the FPS to ~60 on non-wasm, which doesn't always
            // work. I was experiencing ~8000 FPS and this is the only way I know to limit them.
            std::thread::sleep(Duration::from_secs_f64(sleep_secs));
        }
    }
    next_frame().await;
    *previous_time = now();
}
