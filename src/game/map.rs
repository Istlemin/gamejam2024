use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{geometry::Polygon, AppState};

use super::{DeathZone, DespawnOnRestart, Materials, Platform};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_floor);
    }
}

pub fn spawn_floor(
    mut commands: Commands,
    materials: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    spawn_platform(Vec2::new(0.0, 0.0), &mut commands, &materials, &mut meshes);
    spawn_platform(Vec2::new(15.0, 5.0), &mut commands, &materials, &mut meshes);
    spawn_platform(
        Vec2::new(15.0, -5.0),
        &mut commands,
        &materials,
        &mut meshes,
    );

    add_death_zone(&mut commands, &materials, -20.0);
}

pub fn spawn_polygon(
    location: Vec2,
    polygon: Polygon,
    commands: &mut Commands,
    materials: &Res<Materials>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(&polygon)).into(),
            transform: Transform::from_translation(Vec3::new(location.x, location.y, 0.)),
            material: materials.floor_material.clone_weak(),
            ..default()
        },
        Collider::from(polygon.clone()),
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        DespawnOnRestart {},
        Platform::new(polygon),
    ));
}

pub fn spawn_platform(
    location: Vec2,
    commands: &mut Commands,
    materials: &Res<Materials>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let poly = Polygon::new(vec![
        Vec2::new(-5., -0.5),
        Vec2::new(5., -0.5),
        Vec2::new(5., 0.5),
        Vec2::new(-5., 0.5),
    ]);
    spawn_polygon(location, poly, commands, materials, meshes);
}

fn add_death_zone(commands: &mut Commands, materials: &Res<Materials>, y: f32) {
    let width = 100.;
    let height = 10.0;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: materials.death_zone_material.into(),
                custom_size: Vec2::new(width, height).into(),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, y, 0.)),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(width / 2.0, height / 2.0),
        ActiveEvents::COLLISION_EVENTS,
        DeathZone {},
        DespawnOnRestart {},
    ));
}
