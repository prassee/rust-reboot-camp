use std::fmt::Display;

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub enum Shape {
    Circle {
        center: Point,
        radius: u32,
    },
    Rect {
        top_left: Point,
        width: u32,
        height: u32,
    },
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Circle { center, radius } => {
                write!(f, "circle at {} r={}", center, radius)
            }
            Shape::Rect {
                top_left,
                width,
                height,
            } => {
                write!(f, "rect at {} {}x{}", top_left, width, height)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_point() {
        assert_eq!(Point::new(23, 45).to_string(), "(23, 45)");
        assert_eq!(Point::new(23, -45).to_string(), "(23, -45)");
    }

    #[test]
    fn test_shape() {
        let circle = Shape::Circle {
            center: Point::new(34, 45),
            radius: 3,
        };
        assert_eq!(circle.to_string(), "circle at (34, 45) r=3");

        let circle = Shape::Circle {
            center: Point::new(34, -45),
            radius: 3,
        };
        assert_eq!(circle.to_string(), "circle at (34, -45) r=3");

        let rect = Shape::Rect {
            top_left: Point::new(34, 45),
            width: 20,
            height: 10,
        };
        assert_eq!(rect.to_string(), "rect at (34, 45) 20x10");

        let rect = Shape::Rect {
            top_left: Point::new(-34, 45),
            width: 20,
            height: 10,
        };
        assert_eq!(rect.to_string(), "rect at (-34, 45) 20x10");
    }
}
