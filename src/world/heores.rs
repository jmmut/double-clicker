#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Hero {
    Hero1,
    Hero2,
    Hero3,
    Hero4,
    Hero5,
    Hero6,
}

const HEROES_LIST: [Hero; 6] = [
    Hero::Hero1,
    Hero::Hero2,
    Hero::Hero3,
    Hero::Hero4,
    Hero::Hero5,
    Hero::Hero6,
];

impl Hero {
    pub fn list() -> &'static [Hero] {
        &HEROES_LIST
    }
    pub fn index(&self) -> usize {
        match self {
            Hero::Hero1 => 0,
            Hero::Hero2 => 1,
            Hero::Hero3 => 2,
            Hero::Hero4 => 3,
            Hero::Hero5 => 4,
            Hero::Hero6 => 5,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Hero::Hero1 => "Técnico Operario de Cepillo",
            Hero::Hero2 => "Saboteador",
            Hero::Hero3 => "Encerador de Calles",
            Hero::Hero4 => "Maestro del Desorden",
            Hero::Hero5 => "Bot Limpium 2000",
            Hero::Hero6 => "Lord de la Mugre",
        }
    }
    pub fn short_description(&self) -> &'static str {
        match self {
            Hero::Hero1 => "Hace 10 tareas de limpieza por cada salario",
            Hero::Hero2 => "Hace 12 tareas de ensuciar por cada salario",
            Hero::Hero3 => "Hace 1000 tareas de limpieza por cada salario",
            Hero::Hero4 => "Hace 1100 tareas de ensuciar por cada salario",
            Hero::Hero5 => "???",
            Hero::Hero6 => "???",
        }
    }

    pub fn price(&self) -> i64 {
        match self {
            Hero::Hero1 => 5,
            Hero::Hero2 => 5,
            Hero::Hero3 => 5,
            Hero::Hero4 => 5,
            Hero::Hero5 => 5,
            Hero::Hero6 => 5,
        }
    }

    pub fn production_clean(&self) -> i64 {
        match self {
            Hero::Hero1 => 10,
            Hero::Hero2 => 0,
            Hero::Hero3 => 1000,
            Hero::Hero4 => 0,
            Hero::Hero5 => 0,
            Hero::Hero6 => 0,
        }
    }
    pub fn production_dirty(&self) -> i64 {
        match self {
            Hero::Hero1 => 0,
            Hero::Hero2 => 12,
            Hero::Hero3 => 0,
            Hero::Hero4 => 1100,
            Hero::Hero5 => 0,
            Hero::Hero6 => 0,
        }
    }
}
