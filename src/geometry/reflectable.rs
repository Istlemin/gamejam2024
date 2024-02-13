use super::Circle;
use super::Line;
use super::Point;

pub trait Reflectable {
    type InvertOutType;

    fn reflect_over_point(&self, origin: Point) -> Self;
    fn reflect_over_line(&self, line: Line) -> Self;
    fn invert_over_circle(&self, circle: Circle) -> Self::InvertOutType;
}
