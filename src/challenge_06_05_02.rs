use std::f64::consts;

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        consts::PI * self.radius * self.radius
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
}

#[cfg(test)]
mod tests {
    use super::{Circle, Rect, Shape};
    use std::f64::consts;

    #[test]
    fn test_shape() {
        approx::assert_ulps_eq!(Circle { radius: 2.0 }.area(), consts::PI * 4.0);

        approx::assert_ulps_eq!(
            Rect {
                width: 2.0,
                height: 3.0
            }
            .area(),
            6.0,
        );
    }
}
