use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set: HashSet<[u32; 3]> = HashSet::new();
    let mut c = 0;
    println!("{:?}", sum);
    for a in 1..= sum/3 {
        for b in a+1..= sum / 2 {
            c = sum - a - b;
            if (c * c == a * a + b * b){
                 set.insert([a, b, c]);

            }
            
        }
    }
    return set;
    
    todo!(
        "Given the sum {sum}, return all possible Pythagorean triplets, which produce the said sum, or an empty HashSet if there are no such triplets. Note that you are expected to return triplets in [a, b, c] order, where a < b < c"
    );
}
  