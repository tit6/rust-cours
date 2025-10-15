use std::collections::HashMap;
use regex::Regex;

pub fn replace_isolated_apostrophes(s: &str) -> String {
    let re_left = Regex::new(r"(?i)(^|[^a-z])['’]").unwrap();
    let step1 = re_left.replace_all(s, "$1 ").into_owned();

    let re_right = Regex::new(r"(?i)['’]([^a-z]|$)").unwrap();
    re_right.replace_all(&step1, " $1").into_owned()
}

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut HashMap = HashMap::new();
    
    // ← ne peut pas prendre &str sur un String temporaire : on garde le String
    let word: String = words
        .replace(":", " ")
        .replace("!", " ")
        .replace("?", " ")
        .replace(",", " ")
        .replace("&", " ")
        .replace("@", " ")
        .replace("$", " ")
        .replace("^", " ")
        .replace("%", " ")
        .replace("\t", " ")
        .replace(".", " ")
        .replace("\n", " ");

    // ← la fonction renvoie String, donc pas de &
    let temp_word: String = replace_isolated_apostrophes(&word);

    // ← split retourne des &str ; on collecte en Vec<&str>
    let word_parts: Vec<&str> = temp_word.split(" ").collect();
    
    for i in &word_parts {
        if *i == "" || *i == " " {
            continue
        }
        if let Some(val) = HashMap.get(&*i.to_lowercase()) {
            HashMap.insert(i.to_string().to_lowercase(), val + 1);
        } else {
            HashMap.insert(i.to_string().to_lowercase(), 1);
        }
    }

    println!("{:?}", HashMap);
    return HashMap
}