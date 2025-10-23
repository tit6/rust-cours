pub fn egg_count(mut display_value: u32) -> usize {
    let mut count : usize = 0;
    
    loop {
        if display_value <= 0 {
            break;
        }

        if display_value % 2 != 0{
            count = &count + 1;
            println!("{}", display_value)
        }
        display_value = display_value / 2;


    }


    return count;
}
