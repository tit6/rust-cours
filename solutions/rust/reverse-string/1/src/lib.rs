pub fn reverse(input: &str) -> String {
    println!("{input}");
    let reverse_input = input.chars().rev().collect::<String>();
    println!("{}", reverse_input);
    return reverse_input;
    
}
