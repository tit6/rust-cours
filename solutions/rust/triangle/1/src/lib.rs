#[derive(Debug)]
pub struct Triangle {
    x: Vec<u64>,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        for i in sides{
            if i == 0 {
                println!("{}", i);
                return None;
            }
        }
        if sides[0] + sides[1] > sides[2] && sides[0] + sides[2] > sides[1] && sides[1] + sides[2] > sides[0] {
            return Some(Triangle { x : sides.to_vec() })
        }
        
        return None
    }

    pub fn is_equilateral(&self) -> bool {
        self.x[0] == self.x[1] && self.x[1] == self.x[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.x[0] != self.x[1] && self.x[1] != self.x[2] && self.x[0] != self.x[2]
    }

    pub fn is_isosceles(&self) -> bool {
        (self.x[0] == self.x[1] ) || (self.x[1] == self.x[2]) || (self.x[0] == self.x[2])
    }
}
