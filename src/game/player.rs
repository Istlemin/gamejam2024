use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::AppState;

use super::Materials;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player);
    }
}

fn spawn_player(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: materials.player_material.clone(),
                custom_size: Vec2::new(1.0, 1.0).into(),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.)),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5)
    ));
}
