use std::collections::HashMap;

use crate::external::backends::{now, Seconds};
use crate::screen::GuiActions;
use crate::world::heores::Hero;

pub mod heores;


pub const CLEANING_REWARD: i64 = 10;

pub struct World {
    previous_trigger_time: Seconds,
    pub frame: i64,
    pub previous_frame_timestamp: Seconds,
    pub time_since_last_frame: Seconds,
    pub dirtiness: i64,
    pub max_dirtiness: i64,
    pub money: i64,
    pub heroes_count: HashMap<Hero, usize>,
}

impl World {
    pub fn new() -> Self {
        Self {
            previous_trigger_time: now(),
            previous_frame_timestamp: now(),
            frame: 0,
            time_since_last_frame: 0.0,
            dirtiness: 50,
            max_dirtiness: 1000,
            money: 0,
            heroes_count: HashMap::from_iter(Hero::list().iter().map(|h| (*h, 0))),
        }
    }

    pub fn update(&mut self, gui_actions: GuiActions) -> bool {
        self.frame += 1;
        self.max_dirtiness = 1000 + self.money / 100;
        if gui_actions.dirty_pressed {
            self.dirtiness += 10;
        }
        if gui_actions.clean_pressed && self.dirtiness > 0 {
            self.dirtiness -= 10;
            self.money += 10;
        }

        let now_time = now();
        self.time_since_last_frame = now_time - self.previous_frame_timestamp;
        self.previous_frame_timestamp = now_time;

        for villain in [Hero::Villain1, Hero::Villain2, Hero::Villain3] {
            let count = self.heroes_count[&villain] as i64;
            self.dirtiness += count * villain.production_dirty();
        }
        let mut cleaned = 0;
        for hero in [Hero::Hero1, Hero::Hero2, Hero::Hero3] {
            let count = self.heroes_count[&hero] as i64;
            cleaned += count * hero.production_clean();
        }
        cleaned = cleaned.min(self.dirtiness);
        self.money += cleaned;
        self.dirtiness -= cleaned;
        self.dirtiness = self.max_dirtiness.min(self.dirtiness);

        for (hero, bought) in &gui_actions.heroes_bought {
            if *bought && self.money >= hero.price() * 10 {
                *self.heroes_count.get_mut(&hero).unwrap() += 1;
                self.money -= hero.price() * 10;
            }
        }
        for (hero, sold) in &gui_actions.heroes_sold {
            let count = self.heroes_count.get_mut(&hero).unwrap();
            if *count > 0 && *sold {
                *count -= 1;
                self.money += hero.price() * 10;
            }
        }
        if gui_actions.restart {
            *self = Self::new();
        }
        gui_actions.should_continue()
    }


    pub fn min_valid_percentage(&self) -> i64 {
        0
    }

    pub fn max_valid_percentage(&self) -> i64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
