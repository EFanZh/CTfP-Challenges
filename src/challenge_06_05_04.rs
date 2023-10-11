use std::f64::consts;

pub trait Shape {
    fn area(&self) -> f64;
    fn circ(&self) -> f64;
}

pub struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        consts::PI * self.radius * self.radius
    }

    fn circ(&self) -> f64 {
        consts::TAU * self.radius
    }
}

pub struct Rect {
    width: f64,
    height: f64,
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn circ(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

pub struct Square {
    size: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.size * self.size
    }

    fn circ(&self) -> f64 {
        4.0 * self.size
    }
}

#[cfg(test)]
mod tests {
    use super::{Circle, Rect, Shape, Square};
    use std::f64::consts;

    #[test]
    fn test_shape() {
        approx::assert_ulps_eq!(Circle { radius: 7.0 }.area(), consts::PI * 49.0);
        approx::assert_ulps_eq!(Circle { radius: 7.0 }.circ(), consts::PI * 14.0);

        approx::assert_ulps_eq!(
            Rect {
                width: 2.0,
                height: 3.0
            }
            .area(),
            6.0,
        );

        approx::assert_ulps_eq!(
            Rect {
                width: 2.0,
                height: 3.0
            }
            .circ(),
            10.0,
        );

        approx::assert_ulps_eq!(Square { size: 7.0 }.area(), 49.0);
        approx::assert_ulps_eq!(Square { size: 7.0 }.circ(), 28.0);
    }
}
