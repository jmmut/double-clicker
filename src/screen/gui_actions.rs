use std::collections::HashMap;

use crate::world::heores::Hero;

pub struct GuiActions {
    pub quit: bool,
    pub clean_pressed: bool,
    pub dirty_pressed: bool,
    pub next_arrangement: bool,
    pub restart: bool,
    pub continue_playing: bool,
    pub heroes_bought: HashMap<Hero, bool>,
    pub heroes_sold: HashMap<Hero, bool>,
}

impl GuiActions {
    pub fn should_continue(&self) -> bool {
        !self.quit
    }
}
