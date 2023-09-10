use std::time::Duration;
use double_clicker::external::backends::{factory, now, Seconds};
use double_clicker::frame;
use macroquad::prelude::*;

const DEFAULT_WINDOW_WIDTH: i32 = 800;
const DEFAULT_WINDOW_HEIGHT: i32 = 450;
const DEFAULT_WINDOW_TITLE: &str = "Double Clicker";
const MAX_FPS: f64 = 80.0;
const FRAME_PERIOD: f64 = 1.0 / MAX_FPS;

#[macroquad::main(window_conf)]
async fn main() {
    let (mut screen, mut world) = factory().await;

    let mut previous_time = now();
    while frame(&mut screen, &mut world) {
        sleep_until_next_frame(&mut previous_time).await
    }
}

fn window_conf() -> Conf {
    Conf {
        // high_dpi: true,
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}

async fn sleep_until_next_frame(previous_time: &mut Seconds) {
    #[cfg(not(target_family = "wasm"))] {
        let new_time = now();
        dbg!(new_time);
        dbg!(*previous_time);
        let frame_duration = dbg!(new_time - *previous_time);
        if dbg!(frame_duration < dbg!(FRAME_PERIOD)) {
            let sleep_secs = FRAME_PERIOD - frame_duration;
            eprintln!("sleeping for {}", sleep_secs);
            std::thread::sleep(Duration::from_secs_f64(sleep_secs));
        }
    }
    next_frame().await;
    *previous_time = now();
}
