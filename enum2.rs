fn main() {
    let numerateur  : u64 = 1000;
    let denumerateur : u64 = 100;
    let numerateur = numerateur as f64;
    let denumerateur = denumerateur as f64;
    
    match return_division(numerateur, denumerateur) {
        Ok(valide) => println!("resultat : {}", valide),
        Err(er) => println!("resultat : {}", er),
    }
}


fn return_division(a:f64, b:f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err ("impossible de divise pas 0")
    } else {
        Ok(a / b)
    }
}

