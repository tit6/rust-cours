pub fn nth(n: u32) -> u32 {
    let mut liste: Vec<u32> = Vec::new();
    liste = list_np(104_800);

    
    return liste[n as usize]
}

pub fn list_np(s : u32) -> Vec<u32> {
    let mut list: Vec<u32> = Vec::new();
    for i in 2..s+1{
        list.push(i);
    }
    let mut itteration : u32 = 0;
    let mut candidat : u32;
    let mut temp_list : Vec<u32>;
    temp_list = list.clone();
    while itteration < list.len().try_into().unwrap() {
    
        candidat = list[itteration as usize];
        
        //println!("candidat {:?}", candidat);
        //println!("temp_list : {:?}", temp_list);
        //println!("list : {:?}", list);
        //println!("itteration : {:?}", itteration);
        
        
        let mut s = temp_list.len();
        while s > 0 {
            s -= 1;
            if temp_list[s] % candidat == 0 && candidat < temp_list[s] {
                temp_list.remove(s);
                
            }
        }
        
        itteration = itteration + 1;
        list = temp_list.clone();
        

    }
    
    return list
}
