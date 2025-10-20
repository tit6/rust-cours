pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    let t: Vec<String> = digits.chars().map(|c| c.into()).collect();
    let mut temp: String = "TEST".to_string();

    if digits.is_empty() || digits.len() < len {
        return v
    }

    for (count, val) in t.iter().enumerate() {
        temp = "".to_string();
        if len + count > digits.len(){
            return v
        }
        for i in 0..len {
            temp.push_str(&t[i+count].to_string());
            println!("{:?}", temp);
        }
        v.push(temp);
    }
    println!("{:?}", v);
    return v
}
