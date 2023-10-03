use std::fmt::{Display, Formatter};
use crate::screen::translations::Translation;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Alert {
    InefficientCleaners = 0,
    CannotClean = 1,
    InsufficientMoney = 2,
    CannotSell = 3,
}


impl Alert {
    pub fn to_string(&self, translation : &Translation) -> &str {

        match self {
            Alert::InefficientCleaners => translation.alerts.inefficient_cleaners,
            Alert::CannotClean => translation.alerts.cannot_clean,
            Alert::InsufficientMoney => translation.alerts.insufficient_money,
            Alert::CannotSell => translation.alerts.cannot_sell,
        }
    }
}
