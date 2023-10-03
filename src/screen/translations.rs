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
    pub change_style: &'static str,
    pub cleanings: &'static str,
    pub cleaning: &'static str,
    pub dirtyings: &'static str,
    pub dirtying: &'static str,
    pub you_hired: &'static str,
    pub investing: &'static str,
    pub producing: &'static str,
    pub per_second: &'static str,
    pub price: &'static str,
    pub over_greedy: &'static str,
    pub owned_by_dirt: &'static str,
    pub you_won: &'static str,
    pub retire: &'static str,
    pub you_can_continue_playing: &'static str,
    pub savings: &'static str,
    pub cleaning_speed: &'static str,
    pub dirtying_speed: &'static str,
    pub dirts: &'static str,
    pub description: CharacterText,
    pub name: CharacterText,
    pub lore: Lore,
    pub alerts: AlertMessages,
}

pub struct Lore {
    pub act_1: &'static [&'static str],
    pub act_2: &'static [&'static str],
    pub act_3: &'static [&'static str],
    pub game_over: &'static [&'static str],
    pub game_won: &'static [&'static str],
}

pub struct CharacterText {
    pub hero_1: & 'static str,
    pub villain_1: & 'static str,
    pub hero_2: & 'static str,
    pub villain_2: & 'static str,
    pub hero_3: & 'static str,
    pub villain_3: & 'static str,
}
pub struct AlertMessages {
    pub inefficient_cleaners: & 'static str,
    pub cannot_clean: & 'static str,
    pub insufficient_money: & 'static str,
    pub cannot_sell: & 'static str,
}

const SPANISH: Translation = Translation {
    restart: "Reiniciar",
    continue_playing: "Continuar jugando",
    buy: "Comprar",
    sell: "Vender",
    change_style: "Cambiar estilo",
    cleanings: "limpiezas",
    cleaning: "Limpiando",
    dirtyings: "suciedades",
    dirtying: "Ensuciando",
    you_hired: "Has contratado",
    investing: "invirtiendo",
    producing: "Produciendo",
    per_second: "por segundo",
    price: "Precio",
    over_greedy: "Te has pasado de avaricioso.",
    owned_by_dirt: "La suciedad se ha apoderado de ti.",
    you_won: "Has ganado!",
    retire: "Tienes bastante dinero para jubilarte.",
    you_can_continue_playing: "Puedes seguir jugando si quieres.",
    savings: "Ahorros",
    cleaning_speed: "Velocidad de limpieza",
    dirtying_speed: "Velocidad de ensuciamiento",
    dirts: "Suciedades",
    description : CharacterText {
        hero_1: "Hace 1 tarea de limpieza por segundo",
        villain_1: "Hace 2 tareas de ensuciar por segundo",
        hero_2: "Hace 10 tareas de limpieza por segundo",
        villain_2: "Hace 21 tareas de ensuciar por segundo",
        hero_3: "Hace 100 tareas de limpieza por segundo",
        villain_3: "Hace 221 tareas de ensuciar por segundo",
    },
    name: CharacterText {
        hero_1: "Técnico Operario de Cepillo",
        villain_1: "Sucio Steve",
        hero_2: "Larry el Limpio",
        villain_2: "Caos Adora",
        hero_3: "Aspiradora Autónoma Andy",
        villain_3: "Lord de la Mugre",
    },
    lore: Lore {
        act_1: &[
            "Tu jefe quiere hablar contigo, está contento. Demasiado contento.",
            "\"¿Cómo que despedido?\" - Malik",
            "Tu departamento va a desaparecer, debes hacer algo.",
            "\"Debo encontrar 'Los Trapos Sucios'\" - Malik",
        ],
        act_2: &[
            "\"¿Me dejas chupar ese moho?\" - Caos Adora",
            "\"Jefe, voy a necesitar tres cepillos\" - Técnico Operario de Cepillo",
            "Frase acto 2 - 1",
            "Frase acto 2 - 2",
            "Frase acto 2 - 3",
        ],
        act_3: &[
            "Frase acto 3 - 1",
            "Frase acto 3 - 2",
            "Frase acto 3 - 3",
            "Frase acto 3 - 4",
            "Frase acto 3 - 5",
            "Frase acto 3 - 6",
            "Frase acto 3 - 7",
        ],
        game_over: &["Todo se acaba, excepto la suciedad."],
        game_won: &["Contra todo pronóstico, te has salido con la tuya."],
    },
    alerts: AlertMessages {
        inefficient_cleaners: "Tienes limpiadores sin suficiente suciedad que limpiar",
        cannot_clean: "No se puede limpiar si no hay nada sucio",
        insufficient_money: "No tienes suficiente dinero para comprar esto",
        cannot_sell: "No puedes vender porque tienes 0 unidades",
    }
};

const ENGLISH: Translation = Translation {
    restart: "Restart",
    continue_playing: "Continue playing",
    buy: "Buy",
    sell: "Sell",
    change_style: "Change Style",
    cleanings: "cleanings",
    cleaning: "Cleaning",
    dirtyings: "dirtyings",
    dirtying: "Dirtying",
    you_hired: "You hired",
    investing: "investing",
    producing: "Producing",
    per_second: "per second",
    price: "Price",
    over_greedy: "You were too greedy.",
    owned_by_dirt: "The dirt owns you now.",
    you_won: "Has ganado!",
    retire: "You earned enough money to retire.",
    you_can_continue_playing: "You can continue playing if you want.",
    savings: "Savings",
    cleaning_speed: "Cleaning speed",
    dirtying_speed: "Dirtying speed",
    dirts: "Dirtiness",
    description : CharacterText {
        hero_1: "Completes 1 cleaning task per second",
        villain_1: "Completes 2 dirtying tasks per second",
        hero_2: "Completes 10 cleaning tasks per second",
        villain_2: "Completes 21 dirtying tasks per second",
        hero_3: "Completes 100 cleaning tasks per second",
        villain_3: "Completes 221 dirtying tasks per second",
    },
    name: CharacterText {
        hero_1: "Clean Carl",
        villain_1: "Dirty Derek",
        hero_2: "Operator of Cleaning Devices",
        villain_2: "Chaos Lover",
        hero_3: "Autonomous Absterging Andy",
        villain_3: "Dirt Lord",
    },
    lore: Lore {
        act_1: &[
            "Your boss wants to talk to you. He's happy. Too happy.",
            "\"Fired? Me?\" - Malik",
            "They are making your department redundant. You must do something.",
            "\"I must find 'The Dirty Rags'\" - Malik",
        ],
        act_2: &[
            "\"¿Can I lick that mold?\" - Chaos Lover",
            "\"Boss, I'll need three brushes\" - Operator of Cleaning Devices",
            "Frase acto 2 - 1",
            "Frase acto 2 - 2",
            "Frase acto 2 - 3",
        ],
        act_3: &[
            "Frase acto 3 - 1",
            "Frase acto 3 - 2",
            "Frase acto 3 - 3",
            "Frase acto 3 - 4",
            "Frase acto 3 - 5",
            "Frase acto 3 - 6",
            "Frase acto 3 - 7",
        ],
        game_over: &["Everything is finite, except dirtiness."],
        game_won: &["Against all odds, you got away with it."],
    },
    alerts: AlertMessages {
        inefficient_cleaners: "Your cleaners don't have enough dirt to clean",
        cannot_clean: "You can not clean is there is nothing dirty",
        insufficient_money: "You don't have enough money to buy this",
        cannot_sell: "You can not sell this because you have 0 units",
    },
};

pub fn get_translation(language: Language) -> &'static Translation {
    match language {
        Language::Spanish => &SPANISH,
        Language::English => &ENGLISH,
    }
}
