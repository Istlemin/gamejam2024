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
    texture_coords: Vec<Point>,
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
    pub fn new(vertices: Vec<Point>, texture_coords: Option<Vec<Point>>) -> Polygon {
        let mut new_texture_coords = texture_coords.unwrap_or_else(|| vertices.clone());

        let new_verts = if signed_area(&vertices) >= 0.0 {
            vertices
        } else {
            new_texture_coords.reverse();
            vertices.into_iter().rev().collect()
        };

        Polygon {
            vertices: new_verts,
            texture_coords: new_texture_coords,
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
                .rev()
                .collect(),
            texture_coords: self.texture_coords.clone().into_iter().rev().collect(),
        }
    }

    fn reflect_over_point(&self, origin: Point) -> Self {
        Polygon {
            vertices: self
                .vertices
                .iter()
                .map(|x: &Point| x.reflect_over_point(origin))
                .rev()
                .collect(),
            texture_coords: self.texture_coords.clone().into_iter().rev().collect(),
        }
    }
}

fn interpolate(
    last_p: Point,
    last_texture: Point,
    next_p: Point,
    next_texture: Point,
    p: Point,
) -> Point {
    let s = (p - last_p).length();
    let t = (p - next_p).length();

    (last_texture * t + next_texture * s) / (s + t)
}

impl Croppable for Polygon {
    fn crop_to_halfplane(&self, line: super::Line, side: f32) -> Option<Self> {
        let start = (0..self.vertices.len())
            .find(|index: &usize| line.is_on_side(self.vertices[*index], side))?;

        let mut new_vertices = vec![self.vertices[start]];
        let mut new_texture_coords = vec![self.texture_coords[start]];
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
                        new_texture_coords.push(interpolate(
                            self.vertices[last],
                            self.texture_coords[last],
                            self.vertices[nx],
                            self.texture_coords[nx],
                            intersection,
                        ));
                        new_vertices.push(intersection);
                    }
                }
                new_vertices.push(self.vertices[nx]);
                new_texture_coords.push(self.texture_coords[nx])
            } else {
                if line.is_on_side(self.vertices[last], side) {
                    let intersection = line
                        .intersect(Line::new_through(self.vertices[last], self.vertices[nx]))
                        .expect("Expected intersection, but no intersection has been found");

                    if (intersection - *new_vertices.last().unwrap()).length() > EPS {
                        new_texture_coords.push(interpolate(
                            self.vertices[last],
                            self.texture_coords[last],
                            self.vertices[nx],
                            self.texture_coords[nx],
                            intersection,
                        ));
                        new_vertices.push(intersection);
                    }
                }
            }
        }
        new_vertices.pop();

        Some(Polygon {
            vertices: new_vertices,
            texture_coords: new_texture_coords,
        })
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
