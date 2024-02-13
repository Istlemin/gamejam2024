use super::reflectable::Reflectable;
use super::utils::EPS;
use super::Point;

impl Reflectable for Point {
    type InvertOutType = Option<Point>;

    fn reflect_over_point(&self, origin: Point) -> Self {
        2.0 * origin - *self
    }

    fn reflect_over_line(&self, line: super::Line) -> Self {
        let dist = self.dot(line.normal()) + line.get_offset();
        *self - 2.0 * dist * line.normal()
    }

    fn invert_over_circle(&self, circle: super::Circle) -> Self::InvertOutType {
        let diff = *self - circle.center();

        if diff.length() < EPS {
            None
        } else {
            Some(circle.center() + diff * circle.radius() * circle.radius() / (diff.dot(diff)))
        }
    }
}
