use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::{
    geometry::{Croppable, LineSegment, Reflectable},
    AppState,
};

use super::{spawn_polygon, Bullet, Materials, Platform, Player};

pub struct ReflectionsPlugin;

impl Plugin for ReflectionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletMirrorReflectionEvent>()
            .add_event::<PlatformsMirrorReflectionEvent>()
            .add_systems(
                Update,
                (
                    mirror_reflect_platforms,
                    // reflect_platforms_over_line_segment_on_key_press,
                    mirror_reflect_bullets,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Event)]
struct PlatformsMirrorReflectionEvent {
    seg: LineSegment,
}

fn mirror_reflect_platforms_on_key_press(
    keyboard_input: Res<Input<KeyCode>>,
    players: Query<(&Player, &Transform)>,
    mut send_reflection_event: EventWriter<PlatformsMirrorReflectionEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::H) {
        for (player, transformation) in players.iter() {
            if player.id != 0 {
                continue;
            }

            let seg = LineSegment::new(
                transformation.translation.xy(),
                transformation.translation.xy() + Vec2::new(2.0, 3.0),
            );

            send_reflection_event.send(PlatformsMirrorReflectionEvent { seg });
        }
    }
}

fn mirror_reflect_platforms(
    mut reflection_event_reader: EventReader<PlatformsMirrorReflectionEvent>,
    mut commands: Commands,
    platforms: Query<(Entity, &Transform, &Platform)>,
    materials: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for PlatformsMirrorReflectionEvent { seg } in reflection_event_reader.read() {
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
                    spawn_polygon(
                        Vec2::new(0.0, 0.0),
                        poly,
                        &mut commands,
                        &materials,
                        &mut meshes,
                    );
                }
                spawn_polygon(
                    Vec2::new(0.0, 0.0),
                    middle.reflect_over_line(mirror_line),
                    &mut commands,
                    &materials,
                    &mut meshes,
                );
            }
        }
    }
}

#[derive(Event)]
struct BulletMirrorReflectionEvent {
    mirror: LineSegment,
}

fn mirror_reflect_bullets(
    mut bullets: Query<(&Bullet, &mut Transform, &mut Velocity)>,
    mut reflection_event_reader: EventReader<BulletMirrorReflectionEvent>,
) {
    for event in reflection_event_reader.read() {
        bullets.for_each_mut(|(_, mut transform, mut velocity)| {
            let pos = transform.translation.xy();
            let velo_2d = velocity.linvel;
            let line = event.mirror.get_line();

            if event.mirror.on_strip(pos) {
                let new_pos = pos.reflect_over_line(line).extend(0.0);
                let new_velo = velo_2d.reflect_over_line(line.centered_line());

                transform.translation = new_pos;
                transform.rotation = Quat::from_rotation_z(Vec2::X.angle_between(new_velo));
                velocity.linvel = new_velo;
            }
        })
    }
}
