use std::{
    collections::{HashMap, HashSet},
    env::args,
    fs::read_to_string,
};

fn main() {
    let base_char = 0xf1900;
    let words = [
        "a",
        "akesi",
        "ala",
        "alasa",
        "ale",
        "anpa",
        "ante",
        "anu",
        "awen",
        "e",
        "en",
        "esun",
        "ijo",
        "ike",
        "ilo",
        "insa",
        "jaki",
        "jan",
        "jelo",
        "jo",
        "kala",
        "kalama",
        "kama",
        "kasi",
        "ken",
        "kepeken",
        "kili",
        "kiwen",
        "ko",
        "kon",
        "kule",
        "kulupu",
        "kute",
        "la",
        "lape",
        "laso",
        "lawa",
        "len",
        "lete",
        "li",
        "lili",
        "linja",
        "lipu",
        "loje",
        "lon",
        "luka",
        "lukin",
        "lupa",
        "ma",
        "mama",
        "mani",
        "meli",
        "mi",
        "mije",
        "moku",
        "moli",
        "monsi",
        "mu",
        "mun",
        "musi",
        "mute",
        "nanpa",
        "nasa",
        "nasin",
        "nena",
        "ni",
        "nimi",
        "noka",
        "o",
        "alin",
        "ona",
        "open",
        "pakala",
        "pali",
        "palisa",
        "pan",
        "pana",
        "pi",
        "pilin",
        "pimeja",
        "pini",
        "pipi",
        "poka",
        "poki",
        "pona",
        "pu",
        "sama",
        "seli",
        "selo",
        "seme",
        "sewi",
        "sijelo",
        "sike",
        "sin",
        "sina",
        "sinpin",
        "sitelen",
        "sona",
        "soweli",
        "suli",
        "suno",
        "supa",
        "suwi",
        "tan",
        "taso",
        "tawa",
        "telo",
        "tenpo",
        "toki",
        "tomo",
        "tu",
        "unpa",
        "uta",
        "utala",
        "walo",
        "wan",
        "waso",
        "wawa",
        "weka",
        "wile",
        "epiku",
        "jasima",
        "kijetesantakalu",
        "kin",
        "kipisi",
        "leko",
        "monsuta",
        "n",
        "namako",
        "oko",
        "soko",
        "tonsi",
    ];

    let in_path = args()
        .nth(1)
        .expect("expected one argument: path to default tok.json");

    let mut lang: HashMap<String, String> =
        serde_json::from_str(&read_to_string(in_path).unwrap()).unwrap();

    let mut unrecognized = HashSet::new();

    for (key, text) in lang.iter_mut() {
        let mut translated = String::new();
        let mut word = String::new();
        let mut translate = |translated: &mut String, word: &str| {
            let lower = word.to_ascii_lowercase();
            if let Some(index) = words.iter().position(|&w| w == lower) {
                translated.push(char::from_u32(base_char + index as u32).unwrap());
                true
            } else {
                if unrecognized.insert(word.to_owned()) && !word.is_empty() {
                    println!("Unrecognized word in {key}: {word}");
                }
                translated.push_str(word);
                false
            }
        };
        for char in text.chars() {
            if char.is_ascii_alphabetic() {
                word.push(char);
            } else {
                let prev_is_sitelen = translate(&mut translated, &word);
                word.clear();
                if (char != ' ') || !prev_is_sitelen {
                    translated.push(char);
                }
            }
        }
        translate(&mut translated, &word);
        *text = translated;
    }

    let out = serde_json::to_string_pretty(&lang).unwrap();
    std::fs::write("assets/minecraft/lang/tok.json", out).unwrap();
}
