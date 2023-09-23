use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Alert {
    InefficientCleaners = 0,
    CannotClean = 1,
    InsufficientMoney = 2,
    CannotSell = 3,
}

const MESSAGES: [&'static str; 4] = [
    "Tienes limpiadores sin suficiente suciedad que limpiar",
    "No se puede limpiar si no hay nada sucio",
    "No tienes suficiente dinero para comprar esto",
    "No puedes vender porque tienes 0 unidades",
];

impl Alert {
    pub fn to_string(&self) -> &str {
        MESSAGES[*self as usize]
    }
}
impl Display for Alert {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
