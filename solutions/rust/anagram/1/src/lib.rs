use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str>{
    println!("{:?}, {:?}", word, possible_anagrams);
    let mut set: HashSet<&'a str> = HashSet::new();    

    
    for &i in possible_anagrams{
        if i.len() == word.len() &&  i.to_lowercase() != word.to_lowercase() {
            let mut possible_anagrams_vec: Vec<&str> = i.split("").collect();
            possible_anagrams_vec.pop();
            possible_anagrams_vec.remove(0);
            let mut word_vec: Vec<&str> = word.split("").collect();
            word_vec.pop();
            word_vec.remove(0);
            
            let mut temp_word_vec = word_vec.clone();
            for &valeur_anagrams in possible_anagrams_vec.iter(){
                 if let Some(index) = temp_word_vec.iter().position(|&x| x.to_lowercase().to_string() == valeur_anagrams.to_lowercase().to_string()) {
                    temp_word_vec.remove(index);
                        
                }
            }
            println!("{:?}", temp_word_vec);
            if temp_word_vec.is_empty() {
                set.insert(i);
            }
        }
    }
    
    
    return set
    
}
