use std::collections::HashMap;
use std::slice::SliceIndex;

#[derive(Copy, Clone)]
pub enum Language {
    Spanish,
    English,
}

pub struct Translation {
    pub restart: &'static str,
}

const SPANISH: Translation = Translation {
    restart: "Reiniciar",
};
const ENGLISH: Translation = Translation { restart: "Restart" };

pub fn text(language: Language) -> &'static Translation {
    match language {
        Language::Spanish => &SPANISH,
        Language::English => &ENGLISH,
    }
    // "asdf"
}
