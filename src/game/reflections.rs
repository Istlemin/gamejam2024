use bevy::prelude::*;

use crate::geometry::{Croppable, LineSegment, Reflectable};

use super::{spawn_polygon, Materials, Platform};

pub struct ReflectionsPlugin;

impl Plugin for ReflectionsPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

fn reflect_platforms_over_line_segment(
    seg: LineSegment,
    commands: &mut Commands,
    platforms: Query<(Entity, &Transform, &Platform)>,
    materials: &Res<Materials>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    for (entity, transform, platform) in platforms.iter() {
        let polygon = platform.get_transformed_polygon(transform);
        let (a, b) = seg.endpoints();
        let mirror_line = seg.get_line();
        let border_a = mirror_line.perpendicular_through(a);
        let border_b = mirror_line.perpendicular_through(b);

        if let Some(middle) = polygon
            .crop_to_halfplane(border_a, border_a.side(b))
            .and_then(|polygon| polygon.crop_to_halfplane(border_b, border_b.side(a)))
        {
            let sides = [
                polygon.crop_to_halfplane(border_a, -border_a.side(b)),
                polygon.crop_to_halfplane(border_b, -border_b.side(a)),
            ];

            commands.entity(entity).despawn();
            for poly in sides.into_iter().flatten() {
                spawn_polygon(Vec2::new(0.0, 0.0), poly, commands, materials, meshes);
            }
            spawn_polygon(
                Vec2::new(0.0, 0.0),
                middle.reflect_over_line(mirror_line),
                commands,
                materials,
                meshes,
            );
        }
    }
}
