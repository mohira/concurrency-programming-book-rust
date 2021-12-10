use std::ops::Add;

struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let v1 = Vec2 { x: 10.0, y: 5.0 };
    let v2 = Vec2 { x: 3.1, y: 8.7 };
    let v = v1 + v2;

    println!("v.x = {}, v.y = {}", v.x, v.y);
}
