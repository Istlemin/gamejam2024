mod camera;
mod reflections;
use camera::*;

mod components;
use components::*;

mod player;
use player::*;

mod map;
use map::*;

mod bullet;
use bullet::*;

mod powerups;
use powerups::*;

use bevy::{
    prelude::*,
    render::texture::{
        ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
    },
};
use bevy_rapier2d::prelude::*;

use crate::AppState;

use self::reflections::ReflectionsPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(MapPlugin)
            .add_plugins(BulletPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ReflectionsPlugin)
            .add_plugins(PowerupsPlugin)
            .add_systems(PreStartup, setup)
            .add_systems(Update, game_over)
            .add_systems(OnExit(AppState::InGame), cleanup);
    }
}

fn game_over(
    mut read_game_over_event: EventReader<GameOverEvent>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for GameOverEvent { lost_player } in read_game_over_event.read() {
        app_state.set(AppState::MainMenu);
    }
}

fn cleanup(to_despawn: Query<Entity, With<DespawnOnRestart>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let sampler_desc = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        ..Default::default()
    };

    let settings = move |s: &mut ImageLoaderSettings| {
        s.sampler = ImageSampler::Descriptor(sampler_desc.clone());
    };

    let floor_texture: Handle<Image> =
        asset_server.load_with_settings("textures/grass.png", settings);
    commands.insert_resource(RapierConfiguration {
        gravity: Vec2 { x: 0.0, y: -70.0 },
        ..Default::default()
    });
    commands.insert_resource(Materials {
        player_material: Color::rgb(0.969, 0.769, 0.784).into(),
        floor_material: materials.add(floor_texture.into()),
        death_zone_material: Color::rgb(0.5, 0.0, 0.).into(),
        bullet_material: Color::rgb(0.8, 0.8, 0.).into(),
    });
    commands.spawn(new_camera_2d());
}
