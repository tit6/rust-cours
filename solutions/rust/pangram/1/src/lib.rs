/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let a : &str = "abcdefghijklmnopqrstuvwxyz";
    let vecteur_ref: Vec<&str> = a.split("").collect();
    let mut vecteur_vide: Vec<&str> = Vec::new();

    for i in &vecteur_ref {
        if sentence.contains(i) || sentence.contains(i.to_uppercase().as_str()){
            vecteur_vide.push(i);
        }
    }

    if vecteur_vide == vecteur_ref {
        return true
    }

    return false
}
