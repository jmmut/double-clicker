use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use double_clicker::external::backends::{factory, now, Seconds};
use double_clicker::frame;
use macroquad::prelude::*;
use std::time::Duration;
use macroquad::prelude::coroutines::{start_coroutine, TimerDelayFuture, wait_seconds};
use macroquad::ui::root_ui;
use double_clicker::screen::Screen;
use double_clicker::world::World;

const DEFAULT_WINDOW_WIDTH: i32 = 800;
const DEFAULT_WINDOW_HEIGHT: i32 = 450;
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

// #[macroquad::main("Texture")]
// async fn main() {
//     clear_background(LIGHTGRAY);
//     root_ui().label(None, "Loading...");
//     next_frame().await;
//
//     let texture: Texture2D = load_texture("examples/ferris.png").await.unwrap();
//     loop {
//         clear_background(LIGHTGRAY);
//         draw_texture(texture, 0., 0., WHITE);
//         next_frame().await
//     }
// }

fn window_conf() -> Conf {
    Conf {
        // high_dpi: true,
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}

async fn save_texture(assets: Arc<Mutex<Option<Texture2D>>>) {
    trace!("before loading");
    let texture = load_texture("assets/images/ferris.png").await.unwrap();
    trace!("after loading");
    // info!("before sleeping");
    // wait_seconds(4.0).await;
    // info!("after sleeping");
    *assets.as_ref().lock().unwrap() = Some(texture);
}

async fn load() -> (Screen, World)  {

    #[cfg(not(target_family = "wasm"))]
    {
        let assets = Arc::new(Mutex::new(None));
        start_coroutine(save_texture(assets.clone()));
        let mut frames = 0;
        while assets.as_ref().lock().unwrap().is_none() {
            // info!("frame of loading screen");
            frames += 1;
            clear_background(LIGHTGRAY);
            root_ui().label(None, "Loading...");
            next_frame().await;
        }
        info!("loading took {} frames", frames);
        let x = factory(assets.as_ref().lock().unwrap().unwrap());
        return x;
    }

    #[cfg(target_family = "wasm")]
    {
        clear_background(LIGHTGRAY);
        root_ui().label(None, "Loading...");
        next_frame().await;
        trace!("before loading");
        let t = load_texture("assets/images/ferris.png").await.unwrap();
        trace!("after loading");
        // trace!("before sleeping");
        // wait_seconds(4.0).await;
        // trace!("after sleeping");
        trace!("moving to regular game loop");
        return factory(t);
    }
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
