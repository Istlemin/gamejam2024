use super::{LineSegment, Point};

pub const EPS: f32 = 1e-6;

pub fn cross(p1: Point, p2: Point) -> f32 {
    p1.x * p2.y - p1.y * p2.x
}

const FAR: f32 = 300.0;

pub fn split_weight_function(p: Point, center: Point) -> f32 {
    let r = (p - center).length().recip();

    if r < FAR {
        r
    } else {
        (FAR - r).exp()
    }
}

pub fn num_integrate(seg: LineSegment, n: usize, f: &dyn Fn(Point) -> f32) -> Vec<f32> {
    let vals: Vec<f32> = (0..n)
        .map(|step| f(seg.interpolate_position(step as f32, (n - step) as f32)).abs())
        .collect();

    let mut intermediate = vec![0.0];
    for j in vals {
        intermediate.push(intermediate.last().unwrap() + j);
    }

    intermediate
}

pub fn signed_area(vertices: &Vec<Point>) -> f32 {
    let mut area = 0.0;

    for i in 0..vertices.len() {
        area += cross(
            vertices[i],
            vertices[if i + 1 >= vertices.len() { 0 } else { i + 1 }],
        ) / 2.0;
    }

    area
}
