use crate::screen::translations::Translation;

pub fn act_1_lore(translation: &Translation) -> &[&str] {
    &translation.lore.act_1
}

pub fn act_2_lore(translation: &Translation) -> &[&str] {
    &translation.lore.act_2
}

pub fn act_3_lore(translation: &Translation) -> &[&str] {
    &translation.lore.act_3
}

pub fn game_over_lore() -> &'static [&'static str] {
    &["Todo se acaba, excepto la suciedad."]
}

pub fn game_won_lore() -> &'static [&'static str] {
    &["Contra todo pronóstico, te has salido con la tuya."]
}
