use std::collections::HashMap;

use crate::external::backends::{now, Seconds};
use crate::screen::GuiActions;
use crate::world::acts::Act;
use crate::world::alerts::Alert;
use crate::world::heores::Hero;

pub mod acts;
mod alerts;
pub mod heores;

type Cents = i64;
type Units = i64;

const ALERT_PERSISTENCE: Seconds = 5.0;
// pub const TARGET_SAVINGS: Units = 1_000_000;
pub const TARGET_SAVINGS: Units = 10;
// pub const TARGET_SAVINGS: Units = 1_000_000;

pub struct World {
    pub frame: i64,
    pub previous_frame_timestamp: Seconds,
    pub time_since_last_frame: Seconds,
    dirtiness: Cents,
    max_dirtiness: Units,
    money: Cents,
    total_money: Cents,
    target_savings: Units,
    pub heroes_count: HashMap<Hero, i64>,
    pub alerts: Vec<(Seconds, Alert)>,
    inefficient_cleaning_warning: bool,
    act: Act,
}

impl World {
    pub fn new() -> Self {
        Self {
            previous_frame_timestamp: now(),
            frame: 0,
            time_since_last_frame: 0.0,
            dirtiness: to_cents(5),
            max_dirtiness: 100,
            money: 0,
            total_money: 0,
            target_savings: TARGET_SAVINGS,
            heroes_count: HashMap::from_iter(Hero::list().iter().map(|h| (*h, 0))),
            alerts: Vec::new(),
            inefficient_cleaning_warning: false,
            act: Act::Act1,
        }
    }

    pub fn update(&mut self, gui_actions: GuiActions) -> bool {
        if gui_actions.restart {
            self.restart();
        }
        if self.act == Act::GameWon {
            if gui_actions.continue_playing {
                self.act = Act::ContinuePlayingAfterWinning;
            }
        } else if self.act != Act::GameOver {
            self.frame += 1;
            let now_time = now();
            self.time_since_last_frame = now_time - self.previous_frame_timestamp;
            self.previous_frame_timestamp = now_time;
            self.remove_old_alerts(now_time);

            // self.max_dirtiness = 100 + self.total_money_euros();
            if gui_actions.dirty_pressed {
                self.dirtiness += to_cents(1);
            }
            if gui_actions.clean_pressed {
                if self.dirtiness >= to_cents(1) {
                    self.dirtiness -= to_cents(1);
                    self.money += to_cents(1);
                    self.total_money += 10;
                } else {
                    self.alerts.push((now_time, Alert::CannotClean));
                }
            }

            for villain in [Hero::Villain1, Hero::Villain2, Hero::Villain3] {
                let count = self.heroes_count[&villain];
                self.dirtiness += count * villain.production_dirty();
            }
            let mut cleaned = 0;
            for hero in [Hero::Hero1, Hero::Hero2, Hero::Hero3] {
                let count = self.heroes_count[&hero];
                cleaned += count * hero.production_clean();
            }
            if cleaned > self.dirtiness {
                self.inefficient_cleaning_warning = true;
                self.alerts.push((now_time, Alert::InefficientCleaners))
            }
            cleaned = cleaned.min(self.dirtiness);
            self.money += cleaned;
            self.total_money += cleaned / 10;
            self.dirtiness -= cleaned;
            self.dirtiness = to_cents(self.max_dirtiness).min(self.dirtiness);

            for (hero, bought) in &gui_actions.heroes_bought {
                if *bought {
                    if self.money_euros() >= self.price(hero) {
                        self.money -= to_cents(self.price(hero));
                        *self.heroes_count.get_mut(&hero).unwrap() += 1;
                    } else {
                        self.alerts.push((now_time, Alert::InsufficientMoney))
                    }
                }
            }
            for (hero, sold) in &gui_actions.heroes_sold {
                let count = self.heroes_count.get_mut(&hero).unwrap();
                if *sold {
                    if *count > 0 {
                        *count -= 1;
                        self.money += to_cents(self.price(hero));
                    } else {
                        self.alerts.push((now_time, Alert::CannotSell))
                    }
                }
            }
            if self.money_euros() >= self.target_savings
                && self.act != Act::ContinuePlayingAfterWinning
            {
                self.act = Act::GameWon;
            }
            if self.dirtiness_units() >= self.max_dirtiness_units() && self.act != Act::GameWon {
                self.act = Act::GameOver;
            }
        }
        gui_actions.should_continue()
    }
    fn remove_old_alerts(&mut self, now_time: Seconds) {
        self.alerts.retain(|(time_alert_was_raised, alert)| {
            time_alert_was_raised + ALERT_PERSISTENCE >= now_time
                && *alert != Alert::InefficientCleaners
        });
    }

    pub fn restart(&mut self) {
        *self = Self::new();
    }
    pub fn price(&self, hero: &Hero) -> Units {
        (self.heroes_count[&hero] + 1) * hero.base_price()
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

    pub fn stage(&self) -> Act {
        self.act
    }
    pub fn min_valid_percentage(&self) -> i64 {
        0
    }

    pub fn max_valid_percentage(&self) -> i64 {
        0
    }

    #[cfg(test)]
    fn set_target_savings(&mut self, new_target_savings: Units) {
        self.target_savings = new_target_savings;
    }
}
pub fn to_cents(unit: Units) -> Cents {
    unit * 100
}

pub fn accumulate_price(n: i64) -> f32 {
    ((1 + n) * n) as f32 / 2.0
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::acts::Act::{Act1, ContinuePlayingAfterWinning, GameOver, GameWon};

    #[test]
    fn test_invested() {
        let actual = accumulate_price(5);
        let expected = 1 + 2 + 3 + 4 + 5;
        assert_eq!(actual, expected as f32);
    }

    #[test]
    fn test_restart_game_over() {
        let mut world = World::new();
        assert_eq!(world.stage(), Act1);
        for _ in 0..world.max_dirtiness_units() {
            world.update(GuiActions {
                dirty_pressed: true,
                ..GuiActions::default()
            });
        }
        assert_eq!(world.stage(), GameOver);

        world.update(GuiActions {
            restart: true,
            ..GuiActions::default()
        });
        assert_eq!(world.stage(), Act1);
    }

    #[test]
    fn test_continue_after_winning() {
        let mut world = World::new();
        let target_savings = 10;
        world.set_target_savings(target_savings);
        assert_eq!(world.stage(), Act1);
        for _ in 0..target_savings {
            world.update(GuiActions {
                dirty_pressed: true,
                clean_pressed: true,
                ..GuiActions::default()
            });
        }

        assert_eq!(world.stage(), GameWon);

        world.update(GuiActions {
            continue_playing: true,
            ..GuiActions::default()
        });

        assert_eq!(world.stage(), ContinuePlayingAfterWinning);
    }

    #[test]
    fn test_win_and_lose_and_restart() {
        let mut world = World::new();
        assert_eq!(world.stage(), Act1);

        let target_savings = 10;
        world.set_target_savings(target_savings);
        for _ in 0..target_savings {
            world.update(GuiActions {
                dirty_pressed: true,
                clean_pressed: true,
                ..GuiActions::default()
            });
        }
        assert_eq!(world.stage(), GameWon);

        world.update(GuiActions {
            continue_playing: true,
            ..GuiActions::default()
        });
        assert_eq!(world.stage(), ContinuePlayingAfterWinning);

        for _ in 0..world.max_dirtiness_units() {
            world.update(GuiActions {
                dirty_pressed: true,
                ..GuiActions::default()
            });
        }
        assert_eq!(world.stage(), GameOver);

        world.update(GuiActions {
            restart: true,
            ..GuiActions::default()
        });
        assert_eq!(world.stage(), Act1);
    }
}
