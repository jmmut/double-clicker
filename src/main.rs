use macroquad::prelude::*;

use double_clicker::external::backends::{now, Seconds};
use double_clicker::external::loader_stage::LoaderStage;
use double_clicker::frame;

const DEFAULT_WINDOW_WIDTH: i32 = 1200;
const DEFAULT_WINDOW_HEIGHT: i32 = 675;
const DEFAULT_WINDOW_TITLE: &str = "Double Clicker";

#[macroquad::main(window_conf)]
async fn main() -> Result<(), FileError> {
    let (mut screen, mut world) = LoaderStage::setup().await?;
    let mut previous_time = now();
    while frame(&mut screen, &mut world) {
        sleep_until_next_frame(&mut previous_time).await
    }
    Ok(())
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

async fn sleep_until_next_frame(previous_time: &mut Seconds) {
    #[cfg(not(target_family = "wasm"))]
    {
        const MAX_FPS: f64 = 80.0;
        const FRAME_PERIOD: f64 = 1.0 / MAX_FPS;
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
            // This may not work in web.
            std::thread::sleep(std::time::Duration::from_secs_f64(sleep_secs));
        }
    }
    next_frame().await;
    *previous_time = now();
}
