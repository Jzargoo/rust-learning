#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point{
    pub fn distance(&self, another: &Self) -> f32 {
        
        let sum  = (self.y - another.y).pow(2) + (self.x - another.x).pow(2);
        (sum as f32).sqrt()
    }
}