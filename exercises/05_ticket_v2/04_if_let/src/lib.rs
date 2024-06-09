enum Shape {
    Circle { radius: f64 },
    Square { border: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    // TODO: Implement the `radius` method using
    //  either an `if let` or a `let/else`.
    // Using `if let`
    pub fn radius(&self) -> f64 {
        // Try to match `self` with the `Shape::Circle { radius }` pattern
        if let Shape::Circle { radius } = self {
            // If the pattern matches, extract the `radius` field
            *radius
        } else {
            // If the pattern does not match, panic with an error message
            panic!("This shape does not have a radius");
        }
    }

    // Using `let/else`
    pub fn radius2(&self) -> f64 {
        // Try to match `self` with the `Shape::Circle { radius }` pattern
        // If the pattern does not match, execute the code in the `else` block
        let Shape::Circle { radius } = self else {
            // Panic with an error message if the pattern does not match
            panic!("This shape does not have a radius");
        };
        // If the pattern matches, extract the `radius` field
        *radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let _ = Shape::Circle { radius: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_square() {
        let _ = Shape::Square { border: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_rectangle() {
        let _ = Shape::Rectangle {
            width: 1.0,
            height: 2.0,
        }
            .radius();
    }
}
