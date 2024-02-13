use super::Point;

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    center: Point,
    radius: f32,
}

impl Circle {
    pub fn new(center: Point, radius: f32) -> Circle {
        assert!(radius >= 0.0);
        Circle {
            center: center,
            radius: radius,
        }
    }

    pub fn center(&self) -> Point {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}
