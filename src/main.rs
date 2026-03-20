use std::fmt;

// Enums with data attached to each variant
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(a, b, c) => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(r) => 2.0 * std::f64::consts::PI * r,
            Shape::Rectangle(w, h) => 2.0 * (w + h),
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}

// Implement Display so we can println! a Shape directly
impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shape::Circle(r) => write!(f, "Circle(r={})", r),
            Shape::Rectangle(w, h) => write!(f, "Rectangle({}x{})", w, h),
            Shape::Triangle(a, b, c) => write!(f, "Triangle({}, {}, {})", a, b, c),
        }
    }
}

fn main() {
    let shapes: Vec<Shape> = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 7.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    println!("{:-<40}", "");
    println!("{:<20} {:>8} {:>8}", "Shape", "Area", "Perimeter");
    println!("{:-<40}", "");

    // Iterators + closures
    for shape in &shapes {
        println!(
            "{:<20} {:>8.2} {:>8.2}",
            shape.to_string(),
            shape.area(),
            shape.perimeter()
        );
    }

    println!("{:-<40}", "");

    // Find the largest shape by area using iterators
    let largest = shapes
        .iter()
        .max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap());

    if let Some(shape) = largest {
        println!("Largest by area: {} ({:.2})", shape, shape.area());
    }

    // Collect areas into a Vec and compute the total
    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("Total area:      {:.2}", total);
}
