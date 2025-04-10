fn main() {
    let numerateur  : u128 = 1;
    let denumerateur : u128 = 100;
    let numerateur = numerateur as f64;
    let denumerateur = denumerateur as f64;
    
    match return_division(numerateur, denumerateur) {
        Divise::Error{er} => println!("error : {}", er),
        Divise::Good{valide} => println!("resultat : {}", valide),
    }
}

enum Divise {
    Error {er : &'static str},
    Good {valide : f64},
}

fn return_division(a:f64, b:f64) -> Divise {
    if b == 0.0 {
        Divise::Error {er : "impossible de divise pas 0"}
    } else {
        Divise::Good{valide : a / b}
    }
}
