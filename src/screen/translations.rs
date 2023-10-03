use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
pub enum Text {
    Restart,
}

const TEXT_COUNT: usize = 1;
#[derive(Copy, Clone)]
pub enum Language {
    Spanish,
}

// const SPANISH: HashMap<Text, &str> = HashMap::from([
//     (Text::Restart, "Reiniciar"),
// ]);

const SPANISH: EnumMap<&str, TEXT_COUNT> = EnumMap::new::<Text>(&[
    (Text::Restart as usize, "Reiniciar"),
], &"dummy value");
struct EnumMap<'a, T, const COUNT: usize> {
    array: [&'a T; COUNT],
}

impl<'a, T: Default, const COUNT: usize> EnumMap<'a, T, COUNT> {
    pub const fn new<E>(array: &'a [(usize, T); COUNT], dummy_value: &'a T) -> Self {
        let mut i = 0;
        let mut sorted_array: [&'a T; COUNT] = [dummy_value; COUNT];
        while i < COUNT {
            let key = *&array[i].0;
            sorted_array[key] = &array[i].1;
            i += 1;
        }
        Self {
            array: sorted_array,
        }
    }
    pub fn get(&self, enum_key: usize) -> &T {
        &self.array[enum_key]
    }
}


// I considered making language a `static mut` or a `static Mutex` but decided against it because
// tests are run in parallel and different tests might want different languages but would share
// the same single variable.
pub fn text(which_sentence: Text, language: Language) -> &'static str {
    match language {
        Language::Spanish => { SPANISH.get(which_sentence as usize) }
    }
    // "asdf"
}

// const fn build_spanish() -> [&'static str; TEXT_COUNT] {
//     let mut text_array : [&str; TEXT_COUNT];
//     while let Some((code, sentence)) = [(Text::Restart, "Reiniciar")].iter() {
//         text_array[*code as usize] = *sentence;
//     }
//     text_array
// }
