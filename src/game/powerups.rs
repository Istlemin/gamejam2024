use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::common_conditions::on_timer};
use bevy_rapier2d::{
    geometry::{ActiveEvents, Collider},
    pipeline::CollisionEvent,
};
use rand::prelude::*;

use crate::{
    game::{DespawnOnRestart, MirrorType},
    AppState,
};

use super::{Player, PlayerSpawnEvent, Powerup, PowerupState};

pub struct PowerupsPlugin;

impl Plugin for PowerupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PowerupSpawnEvent>()
            .add_event::<PowerupCollectionEvent>()
            .add_systems(
                OnEnter(AppState::InGame),
                spawn_powerup_trackers.after(super::player::spawn_players),
            )
            .add_systems(
                Update,
                check_powerup_color.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                check_powerup_spawn
                    .run_if(on_timer(Duration::from_secs(1)))
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                check_powerup_collection.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                handle_powerup_collection.run_if(in_state(AppState::InGame)),
            )
            .add_systems(Update, spawn_powerup.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Event)]
struct PowerupSpawnEvent {}

const POWERUP_XMIN: f32 = -20.0;
const POWERUP_XMAX: f32 = 20.0;
const POWERUP_YMIN: f32 = -5.0;
const POWERUP_YMAX: f32 = 20.0;

fn spawn_powerup(
    mut commands: Commands,
    mut powerup_event: EventReader<PowerupSpawnEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in powerup_event.read() {
        let x = POWERUP_XMIN + random::<f32>() * (POWERUP_XMAX - POWERUP_XMIN);
        let y = POWERUP_YMIN + random::<f32>() * (POWERUP_YMAX - POWERUP_YMIN);
        info!("Spawning Powerup at {:?} {:?}", x, y);

        let reflections = thread_rng().gen_range(1..8);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(0.5).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            },
            Collider::ball(0.5),
            ActiveEvents::COLLISION_EVENTS,
            DespawnOnRestart {},
            Powerup::Mirror(MirrorType {
                reflect_bullets: (reflections & 1) > 0,
                reflect_players: (reflections & 2) > 0,
                reflect_platforms: (reflections & 4) > 0,
            }),
        ));
    }
}

#[derive(Event)]
pub struct PowerupCollectionEvent {
    player_entity: Entity,
    powerup_entity: Entity,
}

fn handle_powerup_collection(
    mut commands: Commands,
    mut collection_events: EventReader<PowerupCollectionEvent>,
    mut players: Query<(Entity, &mut Player)>,
    powerups: Query<(Entity, &Powerup)>,
) {
    for PowerupCollectionEvent {
        player_entity,
        powerup_entity,
    } in collection_events.read()
    {
        if let Ok((_, mut player)) = players.get_mut(*player_entity) {
            if let Ok((_, powerup)) = powerups.get(*powerup_entity) {
                commands.entity(*powerup_entity).despawn();

                player.powerup = Some(match *powerup {
                    Powerup::Mirror(mirror_type) => PowerupState::Mirror {
                        r#type: mirror_type,
                        point1: None,
                        point2: None,
                    },
                });
            }
        }
    }
}

fn check_powerup_collection(
    players: Query<(Entity, &Player)>,
    powerups: Query<(Entity, &Powerup)>,
    mut contact_events: EventReader<CollisionEvent>,
    mut send_collection_event: EventWriter<PowerupCollectionEvent>,
) {
    for contact_event in contact_events.read() {
        if let CollisionEvent::Started(h1, h2, _) = contact_event {
            if let Ok((player_entity, _)) = players.get(*h1).or(players.get(*h2)) {
                if let Ok((powerup_entity, _)) = powerups.get(*h1).or(powerups.get(*h2)) {
                    send_collection_event.send(PowerupCollectionEvent {
                        player_entity,
                        powerup_entity,
                    });
                }
            }
        }
    }
}

const POWERUP_PROBABILITY: f32 = 0.05;

fn check_powerup_spawn(mut spawn_event: EventWriter<PowerupSpawnEvent>) {
    if random::<f32>() < POWERUP_PROBABILITY {
        spawn_event.send(PowerupSpawnEvent {})
    }
}

#[derive(Component)]
struct PowerupTracker {
    player: Entity,
}

fn get_powerup_color(state: Option<PowerupState>) -> Color {
    match state {
        None => Color::NONE,
        Some(PowerupState::Mirror { r#type, .. }) => Color::Rgba {
            red: if r#type.reflect_bullets { 0.6 } else { 0.0 },
            green: if r#type.reflect_platforms { 0.8 } else { 0.0 },
            blue: if r#type.reflect_players { 0.8 } else { 0.0 },
            alpha: 1.0,
        },
    }
}

fn check_powerup_color(
    mut query: Query<(&mut BackgroundColor, &PowerupTracker)>,
    players: Query<&Player>,
) {
    query.for_each_mut(|(mut color, powerup_tracker)| {
        if let Ok(player) = players.get(powerup_tracker.player) {
            color.0 = get_powerup_color(player.powerup);
        }
    });
}

fn spawn_powerup_trackers(
    mut commands: Commands,
    mut player_spawn_event: EventReader<PlayerSpawnEvent>,
) {
    for PlayerSpawnEvent { player_id, player } in player_spawn_event.read() {
        info!("Creating for player {:?}", player_id);

        let mut style = Style {
            width: Val::VMin(5.0),
            height: Val::VMin(5.0),
            position_type: PositionType::Absolute,
            bottom: Val::Percent(5.0),
            ..default()
        };

        if *player_id == 0 {
            style.left = Val::Percent(7.5);
        } else {
            style.right = Val::Percent(7.5);
        }

        commands.spawn((
            NodeBundle {
                style,
                background_color: Color::NONE.into(),
                ..default()
            },
            PowerupTracker { player: *player },
            DespawnOnRestart {},
        ));
    }
}
