use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::AppState;

use super::{Materials, Platform};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_floor);
    }
}

pub fn spawn_floor(mut commands: Commands, materials: Res<Materials>) {
    spawn_platform(Vec2::new(0.0,0.0), &mut commands, &materials);
    spawn_platform(Vec2::new(15.0,5.0), &mut commands, &materials);
    spawn_platform(Vec2::new(15.0,-5.0), &mut commands, &materials);
}

pub fn spawn_platform(location: Vec2, commands: &mut  Commands, materials: &Res<Materials>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: materials.floor_material.clone(),
                custom_size: Vec2::new(10.0, 1.0).into(),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(location.x, location.y, 0.)),
            ..Default::default()
        },
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        Collider::cuboid(5.0, 0.5),
        Platform
    ));
}

