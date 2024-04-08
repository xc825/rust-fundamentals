enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64, f64),
}

fn largest_shape(shapes: Vec<Shape>) -> Option<Shape> {
    let mut largest_area = 0.0;
    let mut largest_shape = Option::None;
    for shape in shapes {
        let area = match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(a, b, c,) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        };
        if area > largest_area {
            largest_area = area;
            largest_shape = Some(shape);
        }
    }
    largest_shape
}

fn shape_details(shape: Shape) -> String{
    match shape {
        Shape::Circle(radius) => format!("Circle with radius: {}", radius),
        Shape::Square(length) => format!("Square with side length: {}", length),
        Shape::Triangle(a, b, c) => format!("Triangle with sides: {}, {}, {}", a, b, c),
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle(1.0),
        Shape::Square(4.0),
        Shape::Triangle(5.0, 7.0, 5.0)
    ];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(a, b, c,) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        })
        .sum();

    println!("Total area: {:.2} sq. units", total_area);

    let largest_shape = largest_shape(shapes);
    // let largest_shape_details = if let Some(shape) = largest_shape {
    //     shape_details(shape)
    // }
    let largest_shape_details = match largest_shape {
        Some(shape) => shape_details(shape),
        None => format!("No shapes found")
    };

    println!("Largest shape: {:?}", largest_shape_details);
}
