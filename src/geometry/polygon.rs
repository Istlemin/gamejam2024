use std::cmp;

use super::utils::{num_integrate, signed_area, split_weight_function, EPS};
use super::{Circle, Point};
use super::{Croppable, Line, LineSegment, Reflectable};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_rapier2d::geometry::Collider;

#[derive(Debug, Clone)]
pub struct Polygon {
    vertices: Vec<Point>,
    texture_coords: Vec<Point>,
}

impl Polygon {
    pub fn new(mut vertices: Vec<Point>, mut texture_coords: Vec<Point>) -> Polygon {
        if signed_area(&vertices) < 0.0 {
            vertices.reverse();
            texture_coords.reverse();
        };

        Polygon {
            vertices,
            texture_coords,
        }
    }

    pub fn vertices(&self) -> &Vec<Point> {
        &self.vertices
    }

    pub fn texture_coords(&self) -> &Vec<Point> {
        &self.texture_coords
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn area(&self) -> f32 {
        signed_area(&self.vertices)
    }

    pub fn sanitize_polynomial(&self, min_dist: f32, min_area: f32) -> Option<Polygon> {
        if self.num_vertices() < 3 {
            return None;
        }
        let mut vertices_out = vec![self.vertices[0]];
        let mut texture_coords_out = vec![self.texture_coords[0]];

        for j in 1..self.num_vertices() {
            let vert = self.vertices[j];

            if (vert - *vertices_out.last().unwrap()).length() >= min_dist {
                vertices_out.push(vert);
                texture_coords_out.push(self.texture_coords[j])
            }
        }

        if (*vertices_out.last().unwrap() - vertices_out[0]).length() < min_dist {
            vertices_out.pop();
            texture_coords_out.pop();
        }

        if vertices_out.len() > 2 && signed_area(&vertices_out).abs() > min_area {
            Some(Polygon {
                vertices: vertices_out,
                texture_coords: texture_coords_out,
            })
        } else {
            None
        }
    }

    pub fn border(&self) -> Vec<LineSegment> {
        let n = self.num_vertices();

        (0..n)
            .map(|j| LineSegment::new(self.vertices[j], self.vertices[(j + 1) % n]))
            .collect()
    }
}

const POLYGON_RES: usize = 120;

impl Reflectable for Polygon {
    type InvertOutType = Option<Polygon>;

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

    fn invert_over_circle(&self, circle: Circle) -> Self::InvertOutType {
        let segments = self.border();

        let n = self.num_vertices();
        let texture_coords_segs =
            (0..n).map(|j| (self.texture_coords[j], self.texture_coords[(j + 1) % n]));

        let integral_cnt: Vec<usize> = segments
            .iter()
            .map(|seg| cmp::min(400, (seg.length() / 0.05).ceil() as usize))
            .collect();

        let sizes: Vec<f32> = segments
            .iter()
            .zip(integral_cnt.iter())
            .map(|(seg, n)| {
                *num_integrate(*seg, *n, &|x| split_weight_function(x, circle.center()))
                    .last()
                    .unwrap()
            })
            .collect();
        let total: f32 = sizes.iter().sum();

        let mut verts_out = Vec::<Point>::new();
        let mut texture_coords_out = Vec::<Point>::new();

        for ((seg, size), (t_a, t_b)) in segments.iter().zip(sizes.iter()).zip(texture_coords_segs)
        {
            let split = seg.weighted_split(
                &|x| split_weight_function(x, circle.center()),
                (size * (POLYGON_RES as f32) / total).ceil() as usize,
            );

            let (a, b) = seg.endpoints();

            for vertex in split {
                if let Some(p) = vertex.invert_over_circle(circle) {
                    verts_out.push(p);
                    texture_coords_out.push(interpolate_texture_coords(a, t_a, b, t_b, vertex));
                }
            }
        }

        Polygon::new(verts_out, texture_coords_out).sanitize_polynomial(0.001, 0.001)
    }
}

fn interpolate_texture_coords(
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
                    if let Some(intersection) =
                        line.intersect(Line::new_through(self.vertices[last], self.vertices[nx]))
                    {
                        if (intersection - *new_vertices.last().unwrap()).length() > EPS
                            && (intersection - self.vertices[nx]).length() > EPS
                        {
                            new_texture_coords.push(interpolate_texture_coords(
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
                new_vertices.push(self.vertices[nx]);
                new_texture_coords.push(self.texture_coords[nx])
            } else {
                if line.is_on_side(self.vertices[last], side) {
                    if let Some(intersection) =
                        line.intersect(Line::new_through(self.vertices[last], self.vertices[nx]))
                    {
                        if (intersection - *new_vertices.last().unwrap()).length() > EPS {
                            new_texture_coords.push(interpolate_texture_coords(
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
        }
        new_vertices.pop();
        new_texture_coords.pop();

        Polygon::new(new_vertices, new_texture_coords).sanitize_polynomial(EPS, 0.01)
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
                    .texture_coords
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
