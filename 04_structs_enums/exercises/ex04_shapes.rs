// BONUS: Enum Shape (Circle, Rect, Triangle) with area() and perimeter().
// Use f64 for all calculations. π ≈ 3.14159.

enum Shape {
    Circle { radius: f64 },
    Rect { width: f64, height: f64 },
    Triangle { a: f64, b: f64, c: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rect { width, height } => width * height,
            Shape::Triangle { a, b, c } => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 2.0 * std::f64::consts::PI * radius,
            Shape::Rect { width, height } => 2.0 * (width + height),
            Shape::Triangle { a, b, c } => a + b + c,
        }
    }
}

fn main() {
    use std::f64::consts::PI;

    let circle = Shape::Circle { radius: 5.0 };
    assert!((circle.area() - PI * 25.0).abs() < 1e-10);
    assert!((circle.perimeter() - 2.0 * PI * 5.0).abs() < 1e-10);

    let rect = Shape::Rect { width: 4.0, height: 7.0 };
    assert!((rect.area() - 28.0).abs() < 1e-10);
    assert!((rect.perimeter() - 22.0).abs() < 1e-10);

    // 3-4-5 triangle
    let tri = Shape::Triangle { a: 3.0, b: 4.0, c: 5.0 };
    assert!((tri.area() - 6.0).abs() < 1e-10);
    assert!((tri.perimeter() - 12.0).abs() < 1e-10);

    println!("All shape tests passed!");
}
