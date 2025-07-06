mod butterfly;
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

mod maps;
use maps::*;

use bevy::{
    prelude::*,
    render::texture::{
        ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
    },
};
use bevy_rapier2d::prelude::*;

use crate::AppState;

use self::{butterfly::ButterflyPlugin, reflections::ReflectionsPlugin};

pub use components::{KeyBindings, PlayerAction, PlayerControls};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(MapPlugin)
            .add_plugins(BulletPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ButterflyPlugin)
            .add_plugins(ReflectionsPlugin)
            .add_plugins(PowerupsPlugin)
            .add_systems(PreStartup, setup)
            .add_systems(Update, game_over)
            .add_systems(OnEnter(AppState::MainMenu), cleanup)
            .add_systems(
                Update,
                wait_for_restart.run_if(in_state(AppState::GameOver)),
            )
            .add_systems(Update, tick_timers.run_if(in_state(AppState::InGame)));
    }
}

fn wait_for_restart(
    mut app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.get_just_pressed().next().is_some() {
        app_state.set(AppState::MainMenu);
    }
}

fn game_over(
    mut commands: Commands,
    mut read_game_over_event: EventReader<GameOverEvent>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for GameOverEvent { lost_player } in read_game_over_event.read() {
        app_state.set(AppState::GameOver);
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                DespawnOnRestart {},
            ))
            .with_children(|parrent| {
                parrent.spawn(TextBundle::from_section(
                    format!("Player {} won", 2 - lost_player),
                    TextStyle {
                        font_size: 100.0,
                        ..default()
                    },
                ));
            });
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
    commands.insert_resource(get_map1());
    commands.spawn(new_camera_2d());
}

fn tick_timers(
    mut commands: Commands,
    mut cooldowns: Query<(Entity, &mut LifeTimer)>,
    time: Res<Time>,
) {
    // debug!("ticking");
    for (bullet, mut cooldown) in &mut cooldowns {
        cooldown.0.tick(time.delta());

        if cooldown.0.finished() {
            commands.entity(bullet).despawn();
        }
    }
}
