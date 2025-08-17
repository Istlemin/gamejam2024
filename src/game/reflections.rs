use std::{f32::consts::PI, time::Duration};

use bevy::{log::tracing_subscriber::field::debug, prelude::*, render::{mesh::{Indices, PrimitiveTopology}, render_asset::RenderAssetUsages}, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::dynamics::Velocity;

use crate::{
    geometry::{utils::signed_area, Circle, Croppable, Line, LineSegment, Reflectable},
    AppState,
};

use super::{
    spawn_polygon, Bullet, DespawnOnRestart, Materials, MirrorAnimation, MirrorType, Platform,
    Player, InversionType
};

pub struct ReflectionsPlugin;

impl Plugin for ReflectionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletMirrorReflectionEvent>()
            .add_event::<PlatformsMirrorReflectionEvent>()
            .add_event::<PlayerMirrorReflectionEvent>()
            .add_event::<ReflectionEvent>()
            .add_event::<InversionEvent>()
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
    pub angs: (f32, f32),
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
        angs,
    } in inversion_event_reader.read()
    {
        dbg!("Running inversion");
        for (entity, transform, platform) in platforms.iter() {
            let polygon = platform.get_transformed_polygon(transform);
            let line_a = Line::new_through(circle.center(), circle.angle_position(angs.0));
            let line_b = Line::new_through(circle.center(), circle.angle_position(angs.1));

            let a_inside = circle.center() + Vec2::from_angle(angs.0 + PI / 2.0);
            let b_inside = circle.center() + Vec2::from_angle(angs.1 - PI / 2.0);

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
        }
    }
}

#[derive(Event)]
pub struct PlayerInversionEvent {
    pub circle: Circle,
    pub angs: (f32, f32),
}

fn invert_players(
    mut players: Query<(&mut Player, &mut Transform, &mut Velocity)>,
    mut reflection_event_reader: EventReader<PlayerInversionEvent>,
) {
    for PlayerInversionEvent {
        circle,
        angs,
    } in reflection_event_reader.read()
    {
        players.iter_mut().for_each(|(_, mut transform, mut velocity)| {
            let pos = transform.translation.xy();
            let velo_2d = velocity.linvel;

            if signed_area(&vec![
                Vec2::from_angle(angs.0),
                (pos - circle.center()).normalize(),
                Vec2::from_angle(angs.1),
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

fn get_triangle_mesh(ang: f32) -> Mesh {
    let vertices = vec![Vec3::ZERO, Vec3::X, Vec2::from_angle(ang).extend(0.0)]; // Center
    let uvs = vec![Vec2::ZERO, Vec2::X, Vec2::Y];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(vec![0, 1, 2]));
    mesh
}

fn spawn_round_mirror_effect(commands: &mut Commands, circle: Circle, angs: (f32, f32),
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(get_triangle_mesh(angs.1 - angs.0)).into(),
            transform: Transform::from_translation(circle.center().extend(0.0)).with_rotation(
                Quat::from_rotation_z(
                    angs.0,
                ),
            ).with_scale(Vec3::new(1000.0, 1000.0, 1.0)),
            material: materials.add(ColorMaterial::from_color(Color::WHITE)),
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

#[derive(Event)]

pub struct InversionEvent {
    pub inversion_type: InversionType,
    pub circle: Circle,
    pub angs: (f32, f32),
}

fn mirror_use(
    mut reflection_event_reader: EventReader<ReflectionEvent>,
    mut inversion_event_reader: EventReader<InversionEvent>,
    mut send_bullet_mirref_event: EventWriter<BulletMirrorReflectionEvent>,
    mut send_player_mirref_event: EventWriter<PlayerMirrorReflectionEvent>,
    mut send_platforms_mirref_event: EventWriter<PlatformsMirrorReflectionEvent>,
    mut send_player_inversion_event: EventWriter<PlayerInversionEvent>,
    mut send_platforms_inversion_event: EventWriter<PlatformsInversionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
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

    for InversionEvent {
        inversion_type,
        circle,
        angs,
    } in inversion_event_reader.read()
    {
        if inversion_type.reflect_platforms {
            send_platforms_inversion_event.send(PlatformsInversionEvent { circle: *circle, angs: *angs });
        }
        if inversion_type.reflect_players {
            send_player_inversion_event.send(PlayerInversionEvent { circle: *circle, angs: *angs });
        }
        spawn_round_mirror_effect(&mut commands, *circle, *angs, &mut meshes, &mut materials);
    }
}

fn animate_mirror_effect(
    mut mirrors: Query<(Entity, Option<&mut Sprite>, Option<&Handle<ColorMaterial>>, &mut MirrorAnimation)>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    for (entity, sprite_opt, material_handle_opt, mut mirror) in mirrors.iter_mut() {
        mirror.timer.tick(time.delta());

        if mirror.timer.finished() {
            commands.entity(entity).despawn();
        } else {
            if let Some(mut sprite) = sprite_opt {
                sprite.color.set_alpha(mirror.timer.fraction_remaining());
            }
            if let Some(material_handle) = material_handle_opt {
                if let Some(material) = materials.get_mut(material_handle) {
                    material.color.set_alpha(mirror.timer.fraction_remaining());
                }
            }
        }
    }
}
