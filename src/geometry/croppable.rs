use super::Line;

pub trait Croppable: Sized {
    fn crop_to_halfplane(&self, line: Line, side: f32) -> Option<Self>;
}
