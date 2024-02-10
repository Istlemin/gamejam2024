use super::utils::{cross, EPS};
use super::{Croppable, Line, Point, Reflectable};

#[derive(Debug)]
pub struct Polygon {
    vertices: Vec<Point>,
}

fn signed_area(vertices: &Vec<Point>) -> f32 {
    let mut area = 0.0;

    for i in 0..vertices.len() {
        area += cross(
            vertices[i],
            vertices[if i + 1 >= vertices.len() { 0 } else { i + 1 }],
        ) / 2.0;
    }

    area
}

impl Polygon {
    pub fn new(vertices: Vec<Point>) -> Polygon {
        Polygon {
            vertices: if signed_area(&vertices) >= 0.0 {
                vertices
            } else {
                vertices.into_iter().rev().collect()
            },
        }
    }

    pub fn vertices(&self) -> &Vec<Point> {
        &self.vertices
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn area(&self) -> f32 {
        signed_area(&self.vertices)
    }
}

impl Reflectable for Polygon {
    fn reflect_over_line(&self, line: super::Line) -> Self {
        Polygon {
            vertices: self
                .vertices
                .iter()
                .map(|x: &Point| x.reflect_over_line(line))
                .collect(),
        }
    }

    fn reflect_over_point(&self, origin: Point) -> Self {
        Polygon {
            vertices: self
                .vertices
                .iter()
                .map(|x: &Point| x.reflect_over_point(origin))
                .collect(),
        }
    }
}

impl Croppable for Polygon {
    fn crop_to_halfplane(&self, line: super::Line, side: f32) -> Option<Self> {
        let start = (0..self.vertices.len())
            .find(|index: &usize| line.is_on_side(self.vertices[*index], side))?;

        let mut new_vertices = vec![self.vertices[start]];
        let n = self.vertices.len();

        for i in start + 1..start + n + 1 {
            let last = (i - 1) % n;
            let nx = i % n;

            if line.is_on_side(self.vertices[nx], side) {
                if !line.is_on_side(self.vertices[last], side) {
                    let intersection = line
                        .intersect(Line::new_through(self.vertices[last], self.vertices[nx]))
                        .expect("Expected intersection, but no intersection has been found");

                    if (intersection - *new_vertices.last().unwrap()).length() > EPS
                        && (intersection - self.vertices[nx]).length() > EPS
                    {
                        new_vertices.push(intersection);
                    }
                }
                new_vertices.push(self.vertices[nx]);
            } else {
                if line.is_on_side(self.vertices[last], side) {
                    let intersection = line
                        .intersect(Line::new_through(self.vertices[last], self.vertices[nx]))
                        .expect("Expected intersection, but no intersection has been found");

                    if (intersection - *new_vertices.last().unwrap()).length() > EPS {
                        new_vertices.push(intersection);
                    }
                }
            }
        }
        new_vertices.pop();

        Some(Polygon {
            vertices: new_vertices,
        })
    }
}
