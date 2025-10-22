pub fn abbreviate(phrase: &str) -> String {
    let vec : Vec<&str> = phrase.split("").collect();
    let mut Nom: String = "".to_string();
    for (count, val) in vec.iter().enumerate(){
        if count == 0 {
            continue
        }
        if vec[count - 1] == " " && vec[count] != "_" && vec[count] != "-" {
            Nom.push_str(&vec[count].to_uppercase());
            println!("1 : {}", &vec[count]);
        } else if val.chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or(false) {
            println!("2_bis : {}", &vec[count]);
            if !vec[count - 1].chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or(false){
                Nom.push_str(&vec[count].to_uppercase());
                println!("2 : {}", &vec[count]);
            }
        } else if vec[count - 1] == "-" && vec[count] != " " {
            Nom.push_str(&vec[count].to_uppercase());
             println!("3 : {}", &vec[count]);
        }else if vec[count - 2] == "-" && vec[count - 1] == " " {
            Nom.push_str(&vec[count].to_uppercase());
             println!("4 : {}", &vec[count]);
        }
    }
    

    return Nom.to_string()
}
