mod camera;
pub use camera::*;

mod components;
pub use components::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugins(PlayerPlugin)
            .add_systems(PreStartup, setup)
            .add_systems(OnEnter(AppState::InGame), add_test_sprite);
    }
}

fn add_test_sprite(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: materials.floor_material.clone(),
            custom_size: Vec2::new(10.0, 10.0).into(),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.)),
        ..Default::default()
    });
}

fn setup(mut commands: Commands) {
    commands.insert_resource(Materials {
        player_material: Color::rgb(0.969, 0.769, 0.784).into(),
        floor_material: Color::rgb(0.7, 0.7, 0.7).into(),
        bullet_material: Color::rgb(0.8, 0.8, 0.).into(),
    });
    commands.spawn(new_camera_2d());
}
