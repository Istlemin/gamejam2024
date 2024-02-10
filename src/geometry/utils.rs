use super::Point;

pub const EPS: f32 = 1e-6;

pub fn cross(p1: Point, p2: Point) -> f32 {
    p1.x * p2.y - p1.y * p2.x
}
