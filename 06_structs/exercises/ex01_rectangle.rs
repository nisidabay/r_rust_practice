/*
 * ex01_rectangle.rs — Exercise 1
 *
 * Task: Define a Rectangle struct with width and height (f64).
 *       Implement methods: area(), perimeter(), can_hold(other).
 *
 * Run: ./ex01_rectangle
 * Expected: area=50, perimeter=30, can hold smaller rect
 */

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let big = Rectangle::new(10.0, 5.0);
    let small = Rectangle::new(4.0, 3.0);
    let too_wide = Rectangle::new(12.0, 2.0);

    println!("Big rect: {:.1} x {:.1}", big.width, big.height);
    println!("Area: {:.1}", big.area());
    println!("Perimeter: {:.1}", big.perimeter());

    assert!((big.area() - 50.0).abs() < 0.001);
    assert!((big.perimeter() - 30.0).abs() < 0.001);

    println!("Can hold small: {}", big.can_hold(&small)); // true
    println!("Can hold too_wide: {}", big.can_hold(&too_wide)); // false

    assert!(big.can_hold(&small));
    assert!(!big.can_hold(&too_wide));

    println!("All tests passed!");
}
