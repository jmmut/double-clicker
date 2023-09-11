use crate::external::backends::{now, Seconds};
use crate::screen::GuiActions;

pub const MONEY_PERIOD: f64 = 5.0;
pub const SALARY: i64 = 100;

pub struct World {
    previous_trigger_time: Seconds,
    pub remaining_until_next_trigger: Seconds,
    pub frame: i64,
    pub previous_frame_timestamp: Seconds,
    pub time_since_last_frame: Seconds,
    pub cleaned: i64,
    pub dirtied: i64,
    pub money: i64,
}

impl World {
    pub fn new() -> Self {
        Self {
            previous_trigger_time: now(),
            previous_frame_timestamp: now(),
            remaining_until_next_trigger: MONEY_PERIOD,
            frame: 0,
            time_since_last_frame: 0.0,
            cleaned: 0,
            dirtied: 0,
            money: 0,
        }
    }
    pub fn update(&mut self, gui_actions: GuiActions) -> bool {
        self.frame += 1;

        if gui_actions.dirty_pressed {
            self.dirtied += 1;
        }
        if gui_actions.clean_pressed {
            self.cleaned += 1;
        }

        let now_time = now();
        self.time_since_last_frame = now_time - self.previous_frame_timestamp;
        self.previous_frame_timestamp = now_time;

        let trigger_time = try_trigger_timer(self.previous_trigger_time, now_time, MONEY_PERIOD);
        self.remaining_until_next_trigger = trigger_time.remaining;
        if trigger_time.triggered {
            self.previous_trigger_time = trigger_time.new_time;
            self.money += self.expected_payment();
            self.dirtied = monotonically_decrease(self.dirtied);
            self.cleaned = monotonically_decrease(self.cleaned);
        }
        gui_actions.should_continue()
    }

    pub fn expected_payment(&self) -> i64 {
        if should_receive_payment(self.dirtied, self.cleaned) {
            SALARY
        } else {
            0
        }
    }
}

pub fn should_receive_payment(dirtied: i64, cleaned: i64) -> bool {
    if dirtied + cleaned == 0 {
        false
    } else {
        let percentage = dirtied * 100 / (dirtied + cleaned);
        let min_valid_percentage = 40;
        let max_valid_percentage = 60;
        if min_valid_percentage <= percentage && percentage <= max_valid_percentage {
            true
        } else {
            false
        }
    }
}

fn monotonically_decrease(x: i64) -> i64 {
    let decreased = x - (x * 10 + 100) / 100;
    decreased.max(0)
}
struct TriggerTime {
    triggered: bool,
    new_time: Seconds,
    remaining: Seconds,
}
fn try_trigger_timer(
    previous_trigger_time: Seconds,
    now_time: Seconds,
    period: Seconds,
) -> TriggerTime {
    let diff = now_time - previous_trigger_time;
    let remaining = period - diff % period;
    if diff >= period {
        TriggerTime {
            triggered: true,
            new_time: now_time,
            remaining,
        }
    } else {
        TriggerTime {
            triggered: false,
            new_time: previous_trigger_time,
            remaining,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_timer_triggered() {
        let previous_trigger_time: Seconds = now();
        let period: Seconds = 5.0;
        let extra_time: Seconds = 1.0;
        let now_time: Seconds = previous_trigger_time + period + extra_time;
        let TriggerTime {
            triggered,
            new_time,
            remaining,
        } = try_trigger_timer(previous_trigger_time, now_time, period);
        assert_eq!(triggered, true);
        assert_eq!(new_time, now_time);
        assert_eq!(remaining, period - extra_time)
    }

    #[test]
    fn test_payment_timer() {
        let previous_trigger_time: Seconds = now();
        let period: Seconds = 5.0;
        let extra_time: Seconds = 1.0;
        let now_time: Seconds = previous_trigger_time + extra_time;
        let TriggerTime {
            triggered,
            new_time,
            remaining,
        } = try_trigger_timer(previous_trigger_time, now_time, period);
        assert_eq!(triggered, false);
        assert_eq!(new_time, previous_trigger_time);
        assert_eq!(remaining, period - extra_time)
    }

    #[test]
    fn test_decrease() {
        assert_eq!(monotonically_decrease(0), 0);
        assert_eq!(monotonically_decrease(1), 0);
        assert_eq!(monotonically_decrease(2), 1);
        assert_eq!(monotonically_decrease(3), 2);
        assert_eq!(monotonically_decrease(9), 8);
        assert_eq!(monotonically_decrease(10), 8);
        assert_eq!(monotonically_decrease(100), 89);
    }
}
