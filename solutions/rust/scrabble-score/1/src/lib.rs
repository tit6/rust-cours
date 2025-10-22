use std::collections::HashMap;


pub fn score(word: &str) -> u64 {

    if word == "" {
        return 0
    }
    let list: Vec<String> = word
    .chars()
    .map(|c| c.to_uppercase().to_string())
    .collect();

    let mut score: u64 = 0;

    let map = HashMap::from([
        (vec!["A", "E", "I", "O", "U", "L", "N", "R", "S", "T"], 1),
        (vec!["D", "G"], 2),
        (vec!["B", "C", "M", "P"], 3),
        (vec!["F", "H", "V", "W", "Y"], 4),
        (vec!["K"], 5),
        (vec!["J", "X"], 8),
        (vec!["Q","Z"], 10)
    ]);
    
    for t in list{
        for (key, num) in &map{
            for i in key {
                if *i == t {
                    score = &score + num;
                }
            }
        }
    }

    println!("{}", score);
    return score
}
