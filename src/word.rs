use rand::Rng;

pub fn select_word() -> String {
    let random_number: usize = rand::thread_rng().gen_range(0..=WORDS.len());
    let word: &str = WORDS[random_number];
    return String::from(word);
}

pub const WORDS: [&str; 29] = [
    "bogge",
    "briar",
    "zelda",
    "cross",
    "berra",
    "aelin",
    "magic",
    "maeve",
    "rifth",
    "peeta",
    "arena",
    "rebel",
    "daunt",
    "harry",
    "magic",
    "wands",
    "draco",
    "ocean",
    "music",
    "nessa",
    "viral",
    "lover",
    "speak",
    "blank",
    "bella",
    "jacob",
    "forks",
    "coven",
    "meyer"
];