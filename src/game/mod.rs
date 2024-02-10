mod camera;
use camera::*;

mod components;
use components::*;

mod player;
use player::*;

mod map;
use map::*;


use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(MapPlugin)
            .add_plugins(PlayerPlugin)
            .add_systems(PreStartup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(RapierConfiguration {
            gravity: Vec2 {x:0.0,y:-70.0},
            ..Default::default()
        });
    commands.insert_resource(Materials {
        player_material: Color::rgb(0.969, 0.769, 0.784).into(),
        floor_material: Color::rgb(0.7, 0.7, 0.7).into(),
        bullet_material: Color::rgb(0.8, 0.8, 0.).into(),
    });
    commands.spawn(new_camera_2d());
}
