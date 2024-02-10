use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{geometry::Polygon, AppState};

use super::{DespawnOnRestart, Materials, Platform};

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
