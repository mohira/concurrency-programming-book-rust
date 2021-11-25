struct Vec2 {
    x: f64,
    y: f64,
}

struct MyU32 {
    val: u32,
}
// type MyInt u32

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    let mut v = Vec2::new(10.0, 5.0);

    println!("v.norm = {}", v.norm());

    v.set(3.8, 9.1);
    println!("v.norm = {}", v.norm());
}