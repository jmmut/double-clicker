use crate::external::backends::factory;
use crate::external::widgets::anchor::Anchor;
use crate::external::widgets::text::TextRect;
use crate::screen::Screen;
use crate::world::World;
use macroquad::prelude::{
    clear_background, load_texture, next_frame, screen_height, screen_width, trace, Color,
    FileError, Texture2D, BLACK, WHITE,
};
use std::future::Future;
use std::pin::Pin;
use std::task::{Poll, RawWaker, RawWakerVTable, Waker};

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
pub struct Loader {
    textures: Vec<Texture2D>,
    in_progress: Option<Pin<Box<dyn Future<Output = Result<Texture2D, FileError>>>>>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
            in_progress: None,
        }
    }

    pub async fn frame(&mut self) -> Result<bool, FileError> {
        if self.textures.len() < TEXTURE_PATHS.len() {
            // more textures to load
            let next_unloaded_index = self.textures.len();
            if let Some(in_progress) = &mut self.in_progress {
                // the loading of some texture was started
                match resume(in_progress) {
                    Some(texture_res) => {
                        // the texture finished loading
                        let texture = texture_res?;
                        self.textures.push(texture);
                        self.in_progress = None;
                    }
                    None => {
                        // the texture is still loading
                    }
                }
            } else {
                // no texture is loading
                let texture_fut = load_texture(TEXTURE_PATHS[next_unloaded_index]);
                let texture2 = Box::pin(texture_fut);
                self.in_progress = Some(texture2);
            }

            clear_background(BLACK);
            Self::draw_loading(next_unloaded_index);
            next_frame().await;
            Ok(true)
        } else {
            trace!("finished loading textures");
            Ok(false)
        }
    }

    fn draw_loading(next_unloaded_index: usize) {
        Self::draw_loading_text(&next_unloaded_index, WHITE);
        trace!("painted frame of loading screen");
    }

    fn draw_loading_text(next_unloaded_index: &usize, color: Color) {
        TextRect::new(
            &format!(
                "Loading... ({}/{})",
                next_unloaded_index,
                TEXTURE_PATHS.len()
            ),
            Anchor::center(screen_width() * 0.5, screen_height() * 0.5),
            32.0,
        )
        .render_text(color);
    }

    pub fn next_stage(self) -> (Screen, World) {
        if self.textures.len() != TEXTURE_PATHS.len() {
            panic!("Called next_stage() too soon. Call this only after frame() returns false")
        } else {
            factory(self.textures)
        }
    }
}

// taken from macroquad::exec ---------------------------

/// returns Some(T) if future is done, None if it would block
fn resume<T>(future: &mut Pin<Box<dyn Future<Output = T>>>) -> Option<T> {
    let waker = waker();
    let mut futures_context = std::task::Context::from_waker(&waker);
    match future.as_mut().poll(&mut futures_context) {
        Poll::Ready(v) => Some(v),
        Poll::Pending => None,
    }
}
fn waker() -> Waker {
    unsafe fn clone(data: *const ()) -> RawWaker {
        RawWaker::new(data, &VTABLE)
    }
    unsafe fn wake(_data: *const ()) {
        panic!(
            "macroquad does not support waking futures, please use coroutines, \
            otherwise your pending future will block until the next frame"
        )
    }
    unsafe fn wake_by_ref(data: *const ()) {
        wake(data)
    }
    unsafe fn drop(_data: *const ()) {
        // Nothing to do
    }
    const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    let raw_waker = RawWaker::new(std::ptr::null(), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}
