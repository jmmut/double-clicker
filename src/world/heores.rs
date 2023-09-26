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
    pub fn name(&self) -> &'static str {
        match self {
            Hero::Hero1 => "Larry el Limpio",
            Hero::Villain1 => "Sucio Steve",
            Hero::Hero2 => "Técnico Operario de Cepillo",
            Hero::Villain2 => "Caos Adora",
            Hero::Hero3 => "Aspiradora Autónoma Andy",
            Hero::Villain3 => "Lord de la Mugre",
        }
    }
    pub fn short_description(&self) -> &'static str {
        match self {
            Hero::Hero1 => "Hace 1 tarea de limpieza por segundo",
            Hero::Villain1 => "Hace 2 tareas de ensuciar por segundo",
            Hero::Hero2 => "Hace 10 tareas de limpieza por segundo",
            Hero::Villain2 => "Hace 21 tareas de ensuciar por segundo",
            Hero::Hero3 => "Hace 100 tareas de limpieza por segundo",
            Hero::Villain3 => "Hace 221 tareas de ensuciar por segundo",
        }
    }
    pub fn base_price(&self) -> i64 {
        match self {
            Hero::Hero1 => 5,
            Hero::Villain1 => 12,
            Hero::Hero2 => 500,
            Hero::Villain2 => 1000,
            Hero::Hero3 => 50000,
            Hero::Villain3 => 80000,
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
