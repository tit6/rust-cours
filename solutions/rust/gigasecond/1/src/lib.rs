use time::PrimitiveDateTime as DateTime;
use time::{Duration, PrimitiveDateTime, Date, Time};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    println!("{start}"); 
    let gigasecond = Duration::seconds(1_000_000_000);
    

    println!("{gigasecond}");
    let result = start + gigasecond;

    println!("Un gigaseconde après : {}", result);
        
    result
}
