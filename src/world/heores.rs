use crate::screen::textures::Texture;
use crate::screen::translations::Translation;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Hero {
    Hero1,
    Villain1,
    Hero2,
    Villain2,
    Hero3,
    Villain3,
}

const HEROES_LIST: [Hero; 6] = [
    Hero::Hero1,
    Hero::Villain1,
    Hero::Hero2,
    Hero::Villain2,
    Hero::Hero3,
    Hero::Villain3,
];

impl Hero {
    pub fn list() -> &'static [Hero] {
        &HEROES_LIST
    }
    pub fn index(&self) -> usize {
        match self {
            Hero::Hero1 => 0,
            Hero::Villain1 => 1,
            Hero::Hero2 => 2,
            Hero::Villain2 => 3,
            Hero::Hero3 => 4,
            Hero::Villain3 => 5,
        }
    }
    pub fn texture_index(&self) -> Texture {
        match self {
            Hero::Hero1 => Texture::Hero1,
            Hero::Villain1 => Texture::Villain1,
            Hero::Hero2 => Texture::Hero2,
            Hero::Villain2 => Texture::Villain2,
            Hero::Hero3 => Texture::Hero3,
            Hero::Villain3 => Texture::Villain3,
        }
    }
    pub fn name(&self, translation: &Translation) -> &'static str {
        match self {
            Hero::Hero1 => translation.name.hero_1,
            Hero::Villain1 => translation.name.villain_1,
            Hero::Hero2 => translation.name.hero_2,
            Hero::Villain2 => translation.name.villain_2,
            Hero::Hero3 => translation.name.hero_3,
            Hero::Villain3 => translation.name.villain_3,
        }
    }
    pub fn short_description(&self, translation: &Translation) -> &'static str {
        match self {
            Hero::Hero1 => translation.description.hero_1,
            Hero::Villain1 => translation.description.villain_1,
            Hero::Hero2 => translation.description.hero_2,
            Hero::Villain2 => translation.description.villain_2,
            Hero::Hero3 => translation.description.hero_3,
            Hero::Villain3 => translation.description.villain_3,
        }
    }
    pub fn long_description(&self, translation: &Translation) -> &'static str {
        match self {
            Hero::Hero1 => translation.long_description.hero_1,
            Hero::Villain1 => translation.long_description.villain_1,
            Hero::Hero2 => translation.long_description.hero_2,
            Hero::Villain2 => translation.long_description.villain_2,
            Hero::Hero3 => translation.long_description.hero_3,
            Hero::Villain3 => translation.long_description.villain_3,
        }
    }
    pub fn base_price(&self) -> i64 {
        match self {
            Hero::Hero1 => 5,
            Hero::Villain1 => 12,
            Hero::Hero2 => 500,
            // Hero::Hero2 => 5,
            Hero::Villain2 => 1000,
            // Hero::Villain2 => 5,
            Hero::Hero3 => 50000,
            // Hero::Hero3 => 5,
            Hero::Villain3 => 80000,
            // Hero::Villain3 => 5,
        }
    }
    pub fn production_clean(&self) -> i64 {
        match self {
            Hero::Hero1 => 1,
            Hero::Villain1 => 0,
            Hero::Hero2 => 10,
            Hero::Villain2 => 0,
            Hero::Hero3 => 100,
            Hero::Villain3 => 0,
        }
    }
    pub fn production_dirty(&self) -> i64 {
        match self {
            Hero::Hero1 => 0,
            Hero::Villain1 => 2,
            Hero::Hero2 => 0,
            Hero::Villain2 => 21,
            Hero::Hero3 => 0,
            Hero::Villain3 => 221,
        }
    }
}
