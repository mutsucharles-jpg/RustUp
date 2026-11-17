//! Lesson 2 exercises: structs, enums, `match` and methods.
//!
//! The exercise tests are marked `#[ignore]` so normal `cargo test` and CI
//! stay green. Run them while you work with:
//!
//!     cargo test lesson2 -- --ignored
//!
//! Replace each `todo!()` with a real implementation until its test passes.
//! Once an exercise passes, delete its `#[ignore]` line.
#![allow(dead_code, unused_variables)]

#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    /// Exercise 2: return true when width and height are equal.
    pub fn is_square(&self) -> bool {
        todo!("compare width and height")
    }
}

#[derive(Debug)]
pub enum Shape {
    Circle(f64),
    Square(f64),
    Rect { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

/// Exercise 1: the `Triangle` arm is missing. Delete the `todo!()` arm and
/// compute the area of a triangle (half of base times height).
pub fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Square(s) => s * s,
        Shape::Rect { width, height } => width * height,
        Shape::Triangle { base, height } => todo!("half of base times height"),
    }
}

/// Exercise 3: return the shape with the largest area, or `None` when the
/// slice is empty.
///
/// Hint: loop over the shapes and keep the best one so far in
/// `let mut best: Option<&Shape> = None;`
pub fn largest(shapes: &[Shape]) -> Option<&Shape> {
    todo!("find the shape with the biggest area")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Worked examples from the lesson. These already pass.

    #[test]
    fn rectangle_area_and_scale() {
        let mut r = Rectangle::new(3.0, 4.0);
        assert_eq!(r.area(), 12.0);
        r.scale(2.0);
        assert_eq!(r.area(), 48.0);
    }

    #[test]
    fn area_of_square_and_rect() {
        assert_eq!(area(&Shape::Square(2.0)), 4.0);
        assert_eq!(
            area(&Shape::Rect {
                width: 2.0,
                height: 5.0
            }),
            10.0
        );
    }

    // Exercises. Remove `#[ignore]` once each one passes.

    #[test]
    #[ignore = "exercise 1"]
    fn exercise1_triangle_area() {
        let t = Shape::Triangle {
            base: 4.0,
            height: 3.0,
        };
        assert_eq!(area(&t), 6.0);
    }

    #[test]
    #[ignore = "exercise 2"]
    fn exercise2_is_square() {
        assert!(Rectangle::new(2.0, 2.0).is_square());
        assert!(!Rectangle::new(2.0, 3.0).is_square());
    }

    #[test]
    #[ignore = "exercise 3"]
    fn exercise3_largest_of_empty_is_none() {
        assert!(largest(&[]).is_none());
    }

    #[test]
    #[ignore = "exercise 3"]
    fn exercise3_largest_picks_biggest_area() {
        let shapes = vec![
            Shape::Square(2.0), // 4
            Shape::Rect {
                width: 2.0,
                height: 5.0,
            }, // 10
            Shape::Triangle {
                base: 4.0,
                height: 3.0,
            }, // 6
        ];
        assert!(matches!(largest(&shapes), Some(Shape::Rect { .. })));
    }
}
