use rand::Rng;
use std::io;

const MAX_TOURS: i32 = 10;

fn random_number() -> i32 {
    rand::rng().random_range(1..101)
}

fn choice_user() -> Option<i32> {
    loop {
        let mut input = String::new();
        println!("Entrez un nombre entre 0-100 (ou 'q' pour quitter) :");

        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "q" {
            break None;
        }
        

        match input.parse::<i32>() {
            Ok(n) if (0..=100).contains(&n) => return Some(n),
            Ok(_) => println!("Le nombre doit être entre 0 et 100."),
            Err(_) => println!("Veuillez entrer un nombre correct."),
        }
    }
}


fn game() -> io::Result<bool> {
    let secret_number = random_number();
    let mut user_number: i32;
    let mut user_tour:i32 = MAX_TOURS;
    loop {
        match choice_user() {
            Some(n) => {
                user_number = n;
                println!("nombre : {}", n)
            }
            None => {
            println!("au revoir");
            return Ok(false)
            },
        }

        if secret_number == user_number {
            return Ok(true);
        } else if secret_number < user_number {
            println!("trop grand");
            user_tour -=1;
        } else if secret_number > user_number {
            println!("trop petit");
            user_tour -=1;
        }
        
        if user_tour == 0 {
            println!("Vous avez utilisé toutes vos tentatives. Le nombre était {}.", secret_number);
            return Ok(false);
        }
    }
}

fn main() {
    match game() {
        Ok(true) => println!("Gagné"),
        Ok(false) => println!("Perdu"),
        Err(e) => eprintln!("Erreur E/S : {e}"),
    }
}
