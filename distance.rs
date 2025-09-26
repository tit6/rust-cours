#[derive(Debug)]
struct Point{
    x: f64,
    y: f64,
}

fn translate(p: &mut Point, dx: f64, dy: f64) {
    (*p).x += dx;
    (*p).y += dy;
    
}

fn distance(p1: Point, p2: Point) -> f64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    return (dx * dx + dy * dy).sqrt();
    
}
fn main() {
    let i = Point{x:0.0, y:0.0};
    let mut u = Point{x:1.0, y:2.0};
    
    translate(&mut u, 1.0, 2.0);
    
    println!("{:#?}", u);
    
    println!("{}", distance(i, u));
}
