use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::{
    geometry::{utils::signed_area, Circle, Croppable, Line, LineSegment, Reflectable},
    AppState,
};

use super::{
    spawn_polygon, Bullet, DespawnOnRestart, Materials, MirrorAnimation, MirrorType, Platform,
    Player,
};

pub struct ReflectionsPlugin;

impl Plugin for ReflectionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletMirrorReflectionEvent>()
            .add_event::<PlatformsMirrorReflectionEvent>()
            .add_event::<PlayerMirrorReflectionEvent>()
            .add_event::<ReflectionEvent>()
            .add_event::<PlatformsInversionEvent>()
            .add_event::<PlayerInversionEvent>()
            .add_systems(
                Update,
                (
                    mirror_reflect_platforms,
                    mirror_reflect_bullets,
                    mirror_reflect_players,
                    animate_mirror_effect,
                    mirror_use,
                    invert_platforms,
                    invert_players,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Event)]
pub struct PlatformsMirrorReflectionEvent {
    pub mirror: LineSegment,
}

fn mirror_reflect_platforms(
    mut reflection_event_reader: EventReader<PlatformsMirrorReflectionEvent>,
    mut commands: Commands,
    platforms: Query<(Entity, &Transform, &Platform)>,
    materials: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for PlatformsMirrorReflectionEvent { mirror } in reflection_event_reader.read() {
        for (entity, transform, platform) in platforms.iter() {
            let polygon = platform.get_transformed_polygon(transform);
            let (a, b) = mirror.endpoints();
            let mirror_line = mirror.get_line();
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
pub struct PlatformsInversionEvent {
    pub circle: Circle,
    pub angle_start: Vec2,
    pub angle_len: f32,
}

fn invert_platforms(
    mut inversion_event_reader: EventReader<PlatformsInversionEvent>,
    mut commands: Commands,
    platforms: Query<(Entity, &Transform, &Platform)>,
    materials: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for PlatformsInversionEvent {
        circle,
        angle_start,
        angle_len,
    } in inversion_event_reader.read()
    {
        dbg!("Running inversion");
        for (entity, transform, platform) in platforms.iter() {
            let polygon = platform.get_transformed_polygon(transform);
            let line_a = Line::new_through(circle.center(), circle.center() + *angle_start);
            let dir_b = Vec2::from_angle(*angle_len).rotate(*angle_start);
            let line_b = Line::new_through(circle.center(), circle.center() + dir_b);

            let a_inside = Vec2::from_angle(PI / 2.0).rotate(*angle_start);
            let b_inside = Vec2::from_angle(-PI / 2.0).rotate(dir_b);

            let temp_1 = polygon.crop_to_halfplane(line_a, line_a.side(a_inside));
            let temp_2 = polygon.crop_to_halfplane(line_a, -line_a.side(a_inside));

            let both_inside = temp_1
                .as_ref()
                .and_then(|p| p.crop_to_halfplane(line_b, line_b.side(b_inside)));
            let one_inside_1 = temp_1
                .as_ref()
                .and_then(|p| p.crop_to_halfplane(line_b, -line_b.side(b_inside)));
            let one_inside_2 = temp_2
                .as_ref()
                .and_then(|p| p.crop_to_halfplane(line_b, line_b.side(b_inside)));
            let both_outside = temp_2
                .as_ref()
                .and_then(|p| p.crop_to_halfplane(line_b, -line_b.side(b_inside)));

            if *angle_len <= PI {
                if let Some(both) = both_inside {
                    commands.entity(entity).despawn();
                    for poly in [one_inside_1, one_inside_2, both_outside]
                        .into_iter()
                        .flatten()
                    {
                        spawn_polygon(
                            Vec2::new(0.0, 0.0),
                            poly,
                            &mut commands,
                            &materials,
                            &mut meshes,
                        );
                    }

                    if let Some(inverted) = both.invert_over_circle(*circle) {
                        spawn_polygon(
                            Vec2::new(0.0, 0.0),
                            inverted,
                            &mut commands,
                            &materials,
                            &mut meshes,
                        );
                    }
                }
            } else {
                if both_inside.is_none() && one_inside_1.is_none() && one_inside_2.is_none() {
                    continue;
                }
                commands.entity(entity).despawn();

                if let Some(outside) = both_outside {
                    spawn_polygon(
                        Vec2::new(0.0, 0.0),
                        outside,
                        &mut commands,
                        &materials,
                        &mut meshes,
                    );
                }

                for poly in [one_inside_1, one_inside_2, both_inside]
                    .into_iter()
                    .flatten()
                {
                    if let Some(new_poly) = poly.invert_over_circle(*circle) {
                        spawn_polygon(
                            Vec2::new(0.0, 0.0),
                            new_poly,
                            &mut commands,
                            &materials,
                            &mut meshes,
                        );
                    }
                }
            }
        }
    }
}

#[derive(Event)]
pub struct PlayerInversionEvent {
    pub circle: Circle,
    pub angle_start: Vec2,
    pub angle_len: f32,
}

fn invert_players(
    mut players: Query<(&mut Player, &mut Transform, &mut Velocity)>,
    mut reflection_event_reader: EventReader<PlayerInversionEvent>,
) {
    for PlayerInversionEvent {
        circle,
        angle_start,
        angle_len,
    } in reflection_event_reader.read()
    {
        players.iter_mut().for_each(|(_, mut transform, mut velocity)| {
            let pos = transform.translation.xy();
            let velo_2d = velocity.linvel;

            if signed_area(&vec![
                angle_start.normalize(),
                (pos - circle.center()).normalize(),
                Vec2::from_angle(*angle_len)
                    .rotate(*angle_start)
                    .normalize(),
            ]) >= 0.0
            {
                let new_pos = pos
                    .invert_over_circle(*circle)
                    .or(Some(Vec2::new(1000.0, 0.0)))
                    .unwrap();
                let new_velo = (pos + velo_2d)
                    .invert_over_circle(*circle)
                    .or(Some(Vec2::new(2000.0, 0.0)))
                    .unwrap()
                    - new_pos;

                transform.translation = new_pos.extend(0.0);
                velocity.linvel = new_velo;
            }
        })
    }
}

#[derive(Event)]
pub struct BulletMirrorReflectionEvent {
    pub mirror: LineSegment,
}

fn mirror_reflect_bullets(
    mut bullets: Query<(&Bullet, &mut Transform, &mut Velocity)>,
    mut reflection_event_reader: EventReader<BulletMirrorReflectionEvent>,
) {
    for event in reflection_event_reader.read() {
        bullets.iter_mut().for_each(|(_, mut transform, mut velocity)| {
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

#[derive(Event)]
pub struct PlayerMirrorReflectionEvent {
    pub mirror: LineSegment,
}

fn mirror_reflect_players(
    mut players: Query<(&mut Player, &mut Transform, &mut Velocity)>,
    mut reflection_event_reader: EventReader<PlayerMirrorReflectionEvent>,
) {
    for event in reflection_event_reader.read() {
        players.iter_mut().for_each(|(_, mut transform, mut velocity)| {
            let pos = transform.translation.xy();
            let velo_2d = velocity.linvel;
            let line = event.mirror.get_line();

            if event.mirror.on_strip(pos) {
                let new_pos = pos.reflect_over_line(line).extend(0.0);
                let new_velo = velo_2d.reflect_over_line(line.centered_line());

                transform.translation = new_pos;
                velocity.linvel = new_velo;
            }
        })
    }
}

fn spawn_mirror_effect(commands: &mut Commands, mirror: LineSegment) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Vec2::new(1000.0, mirror.length()).into(),
                ..Default::default()
            },
            transform: Transform::from_translation(mirror.mid_point().extend(0.0)).with_rotation(
                Quat::from_rotation_z(
                    Vec2::new(0.0, 1.0).angle_between(mirror.get_line().direction()),
                ),
            ),
            ..Default::default()
        },
        MirrorAnimation {
            timer: Timer::new(Duration::from_millis(500), TimerMode::Once),
        },
        DespawnOnRestart {},
    ));
}

#[derive(Event)]

pub struct ReflectionEvent {
    pub mirror_type: MirrorType,
    pub mirror: LineSegment,
}

fn mirror_use(
    mut reflection_event_reader: EventReader<ReflectionEvent>,
    mut send_bullet_mirref_event: EventWriter<BulletMirrorReflectionEvent>,
    mut send_player_mirref_event: EventWriter<PlayerMirrorReflectionEvent>,
    mut send_platforms_mirref_event: EventWriter<PlatformsMirrorReflectionEvent>,
    mut commands: Commands,
) {
    for ReflectionEvent {
        mirror,
        mirror_type,
    } in reflection_event_reader.read()
    {
        if mirror_type.reflect_bullets {
            send_bullet_mirref_event.send(BulletMirrorReflectionEvent { mirror: *mirror });
        }
        if mirror_type.reflect_platforms {
            send_platforms_mirref_event.send(PlatformsMirrorReflectionEvent { mirror: *mirror });
        }
        if mirror_type.reflect_players {
            send_player_mirref_event.send(PlayerMirrorReflectionEvent { mirror: *mirror });
        }
        spawn_mirror_effect(&mut commands, *mirror);
    }
}

fn animate_mirror_effect(
    mut mirrors: Query<(Entity, &mut Sprite, &mut MirrorAnimation)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut mirror) in mirrors.iter_mut() {
        mirror.timer.tick(time.delta());

        if mirror.timer.finished() {
            commands.entity(entity).despawn();
        } else {
            sprite.color.set_a(mirror.timer.fraction_remaining());
        }
    }
}
