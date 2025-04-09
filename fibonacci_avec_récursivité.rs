fn main() {
    let nb1 : u128 = 0;
    let nb2 : u128 = 1;
    let max : u128 = u64::MAX as u128;
    suivant(nb1, nb2, max);
}

fn suivant(nb1 : u128, nb2 : u128, max : u128) {
    let fiba : u128 = nb1 + nb2;
    if fiba >= max {
        println!("arriver en bout des entier de 64 byte");
    } else{
        println!("{}", fiba);
        suivant(nb2, fiba, max);
    }
}
