mod enums;

use crate::enums::Direction;
use crate::enums::Shape;

fn main() {
    direction();
    shape()
}

fn shape () {
    let triangle = Shape::Triangle;
    let rectangle = Shape::Rectangle((2), (2));
    let square = Shape::Square((2));
    let circle = Shape::Circle { rad: (5) };

    println!("Triangle: {:?}", triangle);
    println!("Rectangle: {:?}", rectangle);
    println!("Square: {:?}", square);
    println!("Cicrle: {:?}", circle);
}
fn direction () {
    let north = Direction::North;
    let south = Direction::South;
    let east = Direction::East;
    let west = Direction::West;

    // Print the values of the variables representing directions
    println!("North: {:?}", north);
    println!("South: {:?}", south);
    println!("East: {:?}", east);
    println!("West: {:?}", west);
}
