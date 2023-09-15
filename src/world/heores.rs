#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Hero {
    Hero1,
    Hero2,
    Hero3,
    Hero4,
    Hero5,
    Hero6,
}

pub const HEROES_LIST: [Hero; 6] = [
    Hero::Hero1,
    Hero::Hero2,
    Hero::Hero3,
    Hero::Hero4,
    Hero::Hero5,
    Hero::Hero6,
];

pub fn get_name(hero: Hero) -> &'static str {
    match hero {
        Hero::Hero1 => "Heroe 1",
        Hero::Hero2 => "Heroe 2",
        Hero::Hero3 => "Heroe 3",
        Hero::Hero4 => "Heroe 4",
        Hero::Hero5 => "Heroe 5",
        Hero::Hero6 => "Heroe 6",
    }
}
