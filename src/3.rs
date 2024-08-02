#[derive(Debug)]
enum Shape {
    Circle(f64),         // radius
    Triangle(f64, f64),  // base, height
    Square(f64),         // side length
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Triangle(base, height) => 0.5 * base * height,
            Shape::Square(side) => side * side,
        }
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle(2.0),
        Shape::Triangle(3.0, 4.0),
        Shape::Square(5.0),
    ];

    for shape in shapes {
        println!("{:?} has an area of {:.2}", shape, shape.area());
    }
}