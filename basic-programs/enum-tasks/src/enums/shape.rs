#[derive(Debug)]
pub enum Shape {
    Circle{rad: u32},
    Triangle,
    Rectangle(u32,u32),
    Square(u32)
}