// Exercise: Rectangle struct with area, perimeter, and is_square methods.
// Define a Rectangle struct and implement methods on it.

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
}

fn main() {
    let rect = Rectangle::new(10.0, 20.0);
    assert!((rect.area() - 200.0).abs() < f64::EPSILON);
    assert!((rect.perimeter() - 60.0).abs() < f64::EPSILON);
    assert!(!rect.is_square());

    let square = Rectangle::new(15.0, 15.0);
    assert!((square.area() - 225.0).abs() < f64::EPSILON);
    assert!((square.perimeter() - 60.0).abs() < f64::EPSILON);
    assert!(square.is_square());

    println!("All rectangle tests passed!");
}
