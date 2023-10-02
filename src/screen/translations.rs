
pub enum Text {
    Restart,
}

#[derive(Copy, Clone)]
pub enum Language {
    Spanish,
}

const SPANISH: [&str; 1] = [
    "Reiniciar",
];

// I considered making language a `static mut` or a `static Mutex` but decided against it because
// tests are run in parallel and different tests might want different languages but would share
// the same single variable.
pub fn text(which_sentence: Text, language: Language) -> &'static str {
    match language {
        Language::Spanish => { SPANISH[which_sentence as usize] }
    }
}
