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
            Hero::Hero1 => "Técnico Operario de Cepillo",
            Hero::Villain1 => "Saboteador",
            Hero::Hero2 => "Encerador de Calles",
            Hero::Villain2 => "Maestro del Desorden",
            Hero::Hero3 => "Bot Limpium 2000",
            Hero::Villain3 => "Lord de la Mugre",
        }
    }
    pub fn short_description(&self) -> &'static str {
        match self {
            Hero::Hero1 => "Hace 10 tareas de limpieza por cada salario",
            Hero::Villain1 => "Hace 12 tareas de ensuciar por cada salario",
            Hero::Hero2 => "Hace 1000 tareas de limpieza por cada salario",
            Hero::Villain2 => "Hace 1100 tareas de ensuciar por cada salario",
            Hero::Hero3 => "???",
            Hero::Villain3 => "???",
        }
    }

    pub fn price(&self) -> i64 {
        match self {
            Hero::Hero1 => 5,
            Hero::Villain1 => 5,
            Hero::Hero2 => 5,
            Hero::Villain2 => 5,
            Hero::Hero3 => 5,
            Hero::Villain3 => 5,
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
            Hero::Villain2 => 13,
            Hero::Hero3 => 0,
            Hero::Villain3 => 111,
        }
    }
}
