pub fn square(s: u32) -> u64 {
    if s <= 0 || s >= 65 {
         panic!("panic");
    }
    let mut t : u64 = 1;
        
    for i in 1..s {
        t = t * 2;
    }
    println!("{}", t);
    return(t)
}

pub fn total() -> u64 {
    return(18_446_744_073_709_551_615)
}
