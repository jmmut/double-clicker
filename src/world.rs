use std::collections::HashMap;

use crate::external::backends::{now, Seconds};
use crate::screen::GuiActions;
use crate::world::heores::Hero;

pub mod heores;

type Cents = i64;
type Units = i64;

pub const CLEANING_REWARD: i64 = 10;

pub struct World {
    previous_trigger_time: Seconds,
    pub frame: i64,
    pub previous_frame_timestamp: Seconds,
    pub time_since_last_frame: Seconds,
    dirtiness: Cents,
    max_dirtiness: Units,
    money: Cents,
    total_money: Cents,
    pub heroes_count: HashMap<Hero, i64>,
}

impl World {
    pub fn new() -> Self {
        Self {
            previous_trigger_time: now(),
            previous_frame_timestamp: now(),
            frame: 0,
            time_since_last_frame: 0.0,
            dirtiness: to_cents(5),
            max_dirtiness: 100,
            money: 0,
            total_money: 0,
            heroes_count: HashMap::from_iter(Hero::list().iter().map(|h| (*h, 0))),
        }
    }

    pub fn update(&mut self, gui_actions: GuiActions) -> bool {
        self.frame += 1;
        self.max_dirtiness = 100 + self.total_money_euros();
        if gui_actions.dirty_pressed {
            self.dirtiness += to_cents(1);
        }
        if gui_actions.clean_pressed && self.dirtiness >= to_cents(1) {
            self.dirtiness -= to_cents(1);
            self.money += to_cents(1);
            self.total_money += to_cents(1);
        }

        let now_time = now();
        self.time_since_last_frame = now_time - self.previous_frame_timestamp;
        self.previous_frame_timestamp = now_time;

        for villain in [Hero::Villain1, Hero::Villain2, Hero::Villain3] {
            let count = self.heroes_count[&villain];
            self.dirtiness += count * villain.production_dirty();
        }
        let mut cleaned = 0;
        for hero in [Hero::Hero1, Hero::Hero2, Hero::Hero3] {
            let count = self.heroes_count[&hero];
            cleaned += count * hero.production_clean();
        }
        cleaned = cleaned.min(self.dirtiness);
        self.money += cleaned;
        self.dirtiness -= cleaned;
        self.dirtiness = to_cents(self.max_dirtiness).min(self.dirtiness);

        for (hero, bought) in &gui_actions.heroes_bought {
            if *bought && self.money_euros() >= self.price(hero) {
                self.money -= to_cents(self.price(hero));
                *self.heroes_count.get_mut(&hero).unwrap() += 1;
            }
        }
        for (hero, sold) in &gui_actions.heroes_sold {
            let count = self.heroes_count.get_mut(&hero).unwrap();
            if *count > 0 && *sold {
                *count -= 1;
                self.money += to_cents(self.price(hero));
            }
        }
        if gui_actions.restart {
            *self = Self::new();
        }
        gui_actions.should_continue()
    }

    pub fn price(&self, hero: &Hero) -> Units {
        (self.heroes_count[&hero] + 1)
            * match hero {
                Hero::Hero1 => 5,
                Hero::Villain1 => 7,
                Hero::Hero2 => 2000,
                Hero::Villain2 => 5000,
                Hero::Hero3 => 1000000,
                Hero::Villain3 => 1500000,
            }
    }

    pub fn money_euros(&self) -> Units {
        self.money / 100
    }
    pub fn total_money_euros(&self) -> Units {
        self.total_money / 100
    }
    pub fn dirtiness_units(&self) -> Units {
        self.dirtiness / 100
    }
    pub fn max_dirtiness_units(&self) -> Units {
        self.max_dirtiness
    }
    pub fn min_valid_percentage(&self) -> i64 {
        0
    }

    pub fn max_valid_percentage(&self) -> i64 {
        0
    }
}
pub fn to_cents(unit: Units) -> Cents {
    unit * 100
}

#[cfg(test)]
mod tests {
    use super::*;
}
