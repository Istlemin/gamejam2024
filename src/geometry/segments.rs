use std::cmp;

use super::utils::{cross, num_integrate, split_weight_function, EPS};
use super::{Circle, Croppable, Point, Reflectable};

#[derive(Clone, Copy, Debug)]
pub struct Line {
    coeffs: Point,
    offset: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct LineSegment {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(normal: Point, offset: f32) -> Line {
        let norm = normal.length();
        Line {
            coeffs: normal / norm,
            offset: offset / norm,
        }
    }

    pub fn new_through(a: Point, b: Point) -> Line {
        LineSegment::new(a, b).get_line()
    }

    pub fn intersect(self, other: Line) -> Option<Point> {
        let det = cross(self.coeffs, other.coeffs);

        if det.abs() < EPS {
            None
        } else {
            Some(
                Point::new(
                    -other.coeffs.y * self.offset + self.coeffs.y * other.offset,
                    other.coeffs.x * self.offset - self.coeffs.x * other.offset,
                ) / det,
            )
        }
    }

    pub fn normal(self) -> Point {
        self.coeffs
    }

    pub fn get_offset(self) -> f32 {
        self.offset
    }

    pub fn point_on_line(self) -> Point {
        -self.offset * self.coeffs
    }

    pub fn direction(self) -> Point {
        Point::new(self.coeffs.y, -self.coeffs.x)
    }

    pub fn perpendicular_through(self, p: Point) -> Line {
        Line::new_through(p, p + self.normal())
    }

    pub fn contains(self, p: Point) -> bool {
        self.distance(p) < EPS
    }

    pub fn side(self, p: Point) -> f32 {
        p.dot(self.normal()) + self.offset
    }

    pub fn is_on_side(self, p: Point, side: f32) -> bool {
        self.side(p).abs() < EPS || self.side(p) * side > 0.0
    }

    pub fn centered_line(self) -> Line {
        Line {
            coeffs: self.coeffs,
            offset: 0.0,
        }
    }

    pub fn distance(self, p: Point) -> f32 {
        self.side(p).abs()
    }

    pub fn closest_point(self, p: Point) -> Point {
        -self.offset * self.coeffs + self.direction() * self.direction().dot(p)
    }
}

impl LineSegment {
    pub fn new(start: Point, end: Point) -> LineSegment {
        LineSegment { start, end }
    }

    pub fn length(&self) -> f32 {
        (self.end - self.start).length()
    }

    pub fn mid_point(&self) -> Point {
        (self.start + self.end) / 2.
    }

    pub fn get_line(self) -> Line {
        let cx = self.start.y - self.end.y;
        let cy = self.end.x - self.start.x;
        let cr = -(cx * self.start.x + cy * self.start.y);

        Line::new(Point::new(cx, cy), cr)
    }

    pub fn split(self, line: Line) -> Vec<LineSegment> {
        match self.get_line().intersect(line) {
            None => vec![self],
            Some(intersection) => {
                if self.contains(intersection) {
                    vec![
                        LineSegment::new(self.start, intersection),
                        LineSegment::new(intersection, self.end),
                    ]
                } else {
                    vec![self]
                }
            }
        }
    }

    pub fn contains(self, p: Point) -> bool {
        if !self.get_line().contains(p) {
            return false;
        }

        let p_displacement = p - self.start;
        let direction = self.end - self.start;
        let length = p_displacement.dot(direction);

        return 0.0 <= length && length <= direction.dot(direction);
    }

    pub fn endpoints(self) -> (Point, Point) {
        (self.start, self.end)
    }

    pub fn strip_boundaries(self) -> (Line, Line) {
        let line = self.get_line();
        (
            line.perpendicular_through(self.start),
            line.perpendicular_through(self.end),
        )
    }

    pub fn on_strip(self, p: Point) -> bool {
        let (b_a, b_b) = self.strip_boundaries();
        b_a.is_on_side(p, b_a.side(self.end)) & b_b.is_on_side(p, b_b.side(self.start))
    }

    pub fn interpolate_position(&self, w1: f32, w2: f32) -> Point {
        (w2 * self.start + w1 * self.end) / (w1 + w2)
    }

    pub fn weighted_split(self, f: &dyn Fn(Point) -> f32, num_splits: usize) -> Vec<Point> {
        let n = cmp::min(200, (self.length() / 0.05).ceil() as usize);

        let intermediate = num_integrate(self, n, f);

        let m = num_splits + 1;

        let total = intermediate.last().unwrap();
        let step_sizes = total / (m as f32);

        let mut p1: usize = 0;

        let mut res = vec![];

        for p2 in 0..n {
            let a = self.interpolate_position(p2 as f32, (n - p2) as f32);
            let b = self.interpolate_position((p2 + 1) as f32, (n - p2 - 1) as f32);
            let seg = LineSegment::new(a, b);
            while p1 <= m && step_sizes * (p1 as f32) <= EPS * total + intermediate[p2 + 1] {
                let curr_value = step_sizes * (p1 as f32);
                res.push(seg.interpolate_position(
                    curr_value - intermediate[p2],
                    intermediate[p2 + 1] - curr_value,
                ));
                p1 += 1;
            }
        }

        res
    }
}

const LINESEG_RES: usize = 30;

impl Reflectable for LineSegment {
    type InvertOutType = Vec<LineSegment>;

    fn reflect_over_line(&self, line: Line) -> Self {
        LineSegment::new(
            self.start.reflect_over_line(line),
            self.end.reflect_over_line(line),
        )
    }

    fn reflect_over_point(&self, origin: Point) -> Self {
        LineSegment::new(
            self.start.reflect_over_point(origin),
            self.end.reflect_over_point(origin),
        )
    }

    fn invert_over_circle(&self, circle: Circle) -> Self::InvertOutType {
        let points =
            self.weighted_split(&|x| split_weight_function(x, circle.center()), LINESEG_RES);

        points
            .windows(2)
            .map(|segs| LineSegment::new(segs[0], segs[1]))
            .collect()
    }
}

impl Croppable for LineSegment {
    fn crop_to_halfplane(&self, line: Line, side: f32) -> Option<Self> {
        let segments = self.split(line);
        for seg in segments {
            if line.is_on_side(seg.start, side) && line.is_on_side(seg.end, side) {
                return Some(seg);
            }
        }

        None
    }
}
