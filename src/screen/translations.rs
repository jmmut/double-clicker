use std::collections::HashMap;
use std::slice::SliceIndex;

#[derive(Copy, Clone)]
pub enum Language {
    Spanish,
    English,
}

pub struct Translation {
    pub restart: &'static str,
    pub continue_playing: &'static str,
    pub buy: &'static str,
    pub sell: &'static str,
    pub placeholder3: &'static str,
    pub placeholder4: &'static str,
    pub placeholder5: &'static str,
    pub placeholder6: &'static str,
    pub placeholder7: &'static str,
    pub placeholder8: &'static str,
}

const SPANISH: Translation = Translation {
    restart: "Reiniciar",
    continue_playing: "Continuar jugando",
    buy: "Comprar",
    sell: "Vender",
    placeholder3: "",
    placeholder4: "",
    placeholder5: "",
    placeholder6: "",
    placeholder7: "",
    placeholder8: "",
};
const ENGLISH: Translation = Translation {
    restart: "Restart",
    continue_playing: "Continue playing",
    buy: "Buy",
    sell: "Sell",
    placeholder3: "",
    placeholder4: "",
    placeholder5: "",
    placeholder6: "",
    placeholder7: "",
    placeholder8: "",
};

pub fn text(language: Language) -> &'static Translation {
    match language {
        Language::Spanish => &SPANISH,
        Language::English => &ENGLISH,
    }
    // "asdf"
}
