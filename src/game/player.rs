use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::AppState;

use super::{GameDirection, Materials, Player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player)
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(AppState::InGame)),
            ).add_systems(
                Update,
                player_controller.run_if(in_state(AppState::InGame)),
            );
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
        Collider::cuboid(0.5, 0.5),
        Player {
            speed:10.0,
            facing_direction:GameDirection::Right
        },
    )).insert(Velocity {
        linvel: Vec2::new(0.0, 0.0),
        angvel: 0.0,
    });
}

fn camera_follow_player(
    mut cameras: Query<&mut Transform, With<Camera>>,
    players: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    for player in players.iter() {
        for mut camera in cameras.iter_mut() {
            camera.translation.x = player.translation.x;
            camera.translation.y = player.translation.y;
        }
    }
}


pub fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Player, &mut Velocity)>,
) {
    for (mut player, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.linvel = Vec2::new(-player.speed, velocity.linvel.y).into();
            player.facing_direction = GameDirection::Left
        }
        if keyboard_input.pressed(KeyCode::Right) {
            velocity.linvel = Vec2::new(player.speed, velocity.linvel.y).into();
            player.facing_direction = GameDirection::Right
        }
    }
}
