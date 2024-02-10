use super::Line;
use super::Point;

pub trait Reflectable {
    fn reflect_over_point(self, origin: Point) -> Self;
    fn reflect_over_line(self, line: Line) -> Self;
}
