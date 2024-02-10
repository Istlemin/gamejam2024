pub use bevy::math::Vec2 as Point;

mod croppable;
mod point;
mod polygon;
mod reflectable;
mod segments;
mod utils;

pub use croppable::Croppable;
pub use polygon::Polygon;
pub use reflectable::Reflectable;
pub use segments::{Line, LineSegment};
