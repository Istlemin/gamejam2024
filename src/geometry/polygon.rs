use super::utils::{cross, EPS};
use super::Point;
use super::{Croppable, Line, LineSegment, Reflectable};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_rapier2d::geometry::Collider;

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

impl Polygon {
    pub fn reflect_over_line_segment(&self, seg: LineSegment) -> Option<Polygon> {
        let (a, b) = seg.endpoints();
        let mirror_line = seg.get_line();
        let border_a = mirror_line.perpendicular_through(a);
        let border_b = mirror_line.perpendicular_through(b);

        Some(
            self.crop_to_halfplane(border_a, border_a.side(b))?
                .crop_to_halfplane(border_b, border_b.side(a))?
                .reflect_over_line(mirror_line),
        )
    }
}

impl From<&Polygon> for Mesh {
    fn from(polygon: &Polygon) -> Self {
        Mesh::new(PrimitiveTopology::TriangleList)
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_POSITION,
                polygon
                    .vertices
                    .iter()
                    .map(|p| [p.x, p.y, 0.0])
                    .collect::<Vec<_>>(),
            )
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_NORMAL,
                vec![[0.0, 0.0, 1.0]; polygon.num_vertices()],
            )
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_UV_0,
                polygon
                    .vertices
                    .iter()
                    .map(Vec2::to_array)
                    .collect::<Vec<_>>(),
            )
            .with_indices(Some(Indices::U32(
                (1..(polygon.num_vertices() as u32 - 1))
                    .flat_map(|i| [0, i, i + 1])
                    .collect::<Vec<_>>(),
            )))
    }
}

impl From<Polygon> for Collider {
    fn from(polygon: Polygon) -> Self {
        let n = polygon.num_vertices();
        Collider::polyline(
            polygon.vertices,
            Some(
                (0..n as u32)
                    .map(|i| [i, (i + 1) % n as u32])
                    .collect::<Vec<_>>(),
            ),
        )
    }
}
