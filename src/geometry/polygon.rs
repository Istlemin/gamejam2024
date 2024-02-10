use super::Point;

pub struct Polygon {
    vertices: Vec<Point>,
}

impl Polygon {
    pub fn vertices(&self) -> &Vec<Point> {
        &self.vertices
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }
}
