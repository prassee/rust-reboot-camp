#[derive(Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle { center: Point, radius: u32 },
    Line(Point, Point),
}
// recommended to keep these functions in this file to access private fields of Point and Shape
pub fn starts_at_origin(shape: &Shape) -> bool {
    matches!(
        shape,
        Shape::Circle {
            center: Point { x: 0, y: 0 },
            ..
        } | Shape::Line(Point { x: 0, y: 0 }, ..)
    )
}

pub fn vertical_x(shape: &Shape) -> Option<i32> {
    match shape {
        Shape::Line(Point { x: x1, .. }, Point { x: x2, .. }) if (x1 == x2) => Some(*x1),
        _ => None,
    }
}

pub fn mirror(Point { x, y }: Point) -> Point {
    Point { x: y, y: x }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_for_origin() {
        assert!(starts_at_origin(&Shape::Circle {
            center: Point::new(0, 0),
            radius: 2,
        }));
        assert!(!starts_at_origin(&Shape::Circle {
            center: Point::new(0, 1),
            radius: 2,
        }));
        assert!(!starts_at_origin(&Shape::Circle {
            center: Point::new(0, -11),
            radius: 2,
        }));
        assert!(!starts_at_origin(&Shape::Line(
            Point::new(0, 1),
            Point::new(0, 2),
        )));
        assert!(!starts_at_origin(&Shape::Line(
            Point::new(10, 1),
            Point::new(0, 0),
        )));
        assert!(starts_at_origin(&Shape::Line(
            Point::new(0, 0),
            Point::new(0, 2),
        )));
        assert!(starts_at_origin(&Shape::Line(
            Point::new(0, 0),
            Point::new(0, -12),
        )));
    }

    #[test]
    fn test_for_vertical_x() {
        assert_eq!(
            vertical_x(&Shape::Line(Point::new(10, 3), Point::new(10, 2))),
            Some(10)
        );

        assert_eq!(
            vertical_x(&Shape::Line(Point::new(-10, 10), Point::new(-10, 30))),
            Some(-10)
        );

        assert_eq!(
            vertical_x(&Shape::Line(Point::new(-10, 0), Point::new(0, 0))),
            None
        );

        assert_eq!(
            vertical_x(&Shape::Circle {
                center: Point::new(-10, 0),
                radius: 3
            }),
            None
        );
        assert_eq!(
            vertical_x(&Shape::Circle {
                center: Point::new(10, 23),
                radius: 3
            }),
            None
        );
    }

    #[test]
    fn test_for_mirror() {
        assert_eq!(mirror(Point::new(0, 3)), Point::new(3, 0));
        assert_eq!(mirror(Point::new(-10, 3)), Point::new(3, -10));
    }
}
