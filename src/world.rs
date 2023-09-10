use crate::external::backends::{now, Seconds};
use crate::screen::GuiActions;

pub const MONEY_PERIOD: f64 = 5.0;

pub struct World {
    previous_trigger_time: Seconds,
    remaining_until_next_trigger: Seconds,
}

impl World {
    pub fn new() -> Self {
        Self {
            previous_trigger_time: now(),
            remaining_until_next_trigger: MONEY_PERIOD,
        }
    }
    pub fn update(&mut self, gui_actions: GuiActions) -> bool {
        let trigger_time = try_trigger_timer(self.previous_trigger_time, now(), MONEY_PERIOD);
        if trigger_time.triggered {
            self.previous_trigger_time = trigger_time.new_time;
            self.remaining_until_next_trigger = trigger_time.remaining;
        }
        gui_actions.should_continue()
    }
}

struct TriggerTime {
    triggered: bool,
    new_time: Seconds,
    remaining: Seconds,
}
fn try_trigger_timer(previous_trigger_time: Seconds, now_time: Seconds, period: Seconds) -> TriggerTime {
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
    fn test_payment_timer() {
        let previous_trigger_time: Seconds = macroquad::miniquad::date::now();
        let period : Seconds = 5.0;
        let extra_time: Seconds = 1.0;
        let now_time: Seconds = previous_trigger_time + period + extra_time;
        let TriggerTime { triggered, new_time, remaining } = try_trigger_timer(previous_trigger_time, now_time, period);
        assert_eq!(triggered, true);
        assert_eq!(new_time, now_time);
        assert_eq!(remaining, period - extra_time)
    }
}
