use super::reflectable::Reflectable;
use super::Point;

impl Reflectable for Point {
    fn reflect_over_point(self, origin: Point) -> Self {
        2.0 * origin - self
    }

    fn reflect_over_line(self, line: super::Line) -> Self {
        let dist = self.dot(line.normal()) + line.get_offset();
        self - 2.0 * dist * line.normal()
    }
}
