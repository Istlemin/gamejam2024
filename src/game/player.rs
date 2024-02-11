use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{geometry::LineSegment, AppState};

use super::{
    reflections::MirrorUseEvent, BulletFiredEvent, DeathZone, DespawnOnRestart, GameDirection,
    KeyBindings, Mirror, Player, PowerupState,
};

pub struct PlayerPlugin;

#[derive(Event)]
pub struct GameOverEvent {
    pub lost_player: i32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>()
            .add_event::<MirrorSpawnEvent>()
            .add_systems(OnEnter(AppState::InGame), spawn_players)
            .add_systems(
                Update,
                (
                    //camera_follow_player.run_if(in_state(AppState::InGame)),
                    (
                        player_controller,
                        jump_reset,
                        check_death_collision,
                        animate_sprite,
                        draw_mirrors,
                        spawn_mirror,
                    )
                        .run_if(in_state(AppState::InGame)),
                ),
            );
    }
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn spawn_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    spawn_player(
        0,
        Transform::from_xyz(1.0, 10.0, 0.0),
        Color::rgb(0.969, 0.200, 0.300),
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        KeyBindings {
            left: KeyCode::Left,
            right: KeyCode::Right,
            jump: KeyCode::Up,
            shoot: KeyCode::ControlRight,
            powerup: KeyCode::ShiftRight,
        },
    );
    spawn_player(
        1,
        Transform::from_xyz(-1.0, 10.0, 0.0),
        Color::rgb(0.300, 0.200, 0.900),
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        KeyBindings {
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::W,
            shoot: KeyCode::Space,
            powerup: KeyCode::M,
        },
    );
}

fn spawn_player(
    player_id: i32,
    position: Transform,
    color: Color,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    key_bindings: KeyBindings,
) {
    let texture_handle = asset_server.load("textures/gabe-idle-run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    let scale = Vec3 {
        x: 0.125,
        y: 0.125,
        z: 0.125,
    };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: position.with_scale(scale),
            ..Default::default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::cuboid(8., 12.),
        KinematicCharacterController::default(),
        ActiveEvents::COLLISION_EVENTS,
        Player {
            speed: 10.0,
            facing_direction: GameDirection::Right,
            jump_impulse: 30.0,
            is_jumping: false,
            id: player_id,
            last_shoot_time: Duration::new(0, 0),
            shoot_interval: Duration::new(0, 100_000_000),
            key_bindings,
            powerup: None,
        },
        Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        },
        DespawnOnRestart {},
    ));
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

pub fn player_go_left(
    player: &mut Player,
    velocity: &mut Velocity,
    sprite: &mut TextureAtlasSprite,
) {
    velocity.linvel = Vec2::new(-player.speed, velocity.linvel.y).into();
    player.facing_direction = GameDirection::Left;
    sprite.flip_x = true
}

pub fn player_go_right(
    player: &mut Player,
    velocity: &mut Velocity,
    sprite: &mut TextureAtlasSprite,
) {
    velocity.linvel = Vec2::new(player.speed, velocity.linvel.y).into();
    player.facing_direction = GameDirection::Right;
    sprite.flip_x = false
}

pub fn player_jump(player: &mut Player, velocity: &mut Velocity) {
    if !player.is_jumping {
        player.is_jumping = true;
        velocity.linvel = Vec2::new(velocity.linvel.x, player.jump_impulse).into();
    }
}

pub fn player_shoot(
    player: &mut Player,
    transform: &mut Transform,
    send_fire_event: &mut EventWriter<BulletFiredEvent>,
    time: &Res<Time>,
) {
    if player.last_shoot_time + player.shoot_interval > time.elapsed() {
        return;
    } else {
        let bullet_pos;

        match player.facing_direction {
            GameDirection::Left => {
                bullet_pos = Vec2::new(transform.translation.x - 1.3, transform.translation.y);
            }
            GameDirection::Right => {
                bullet_pos = Vec2::new(transform.translation.x + 1.3, transform.translation.y);
            }
        }

        let event = BulletFiredEvent {
            position: bullet_pos,
            direction: player.facing_direction,
        };
        send_fire_event.send(event);

        player.last_shoot_time = time.elapsed();
    }
}

pub fn player_use_powerup(
    player: &mut Player,
    player_entity: Entity,
    transform: &mut Transform,
    send_mirror_spawn_event: &mut EventWriter<MirrorSpawnEvent>,
    send_mirror_use_event: &mut EventWriter<MirrorUseEvent>,
) {
    player.powerup = if let Some(powerup) = player.powerup {
        debug!("Powerup activated");
        match powerup {
            PowerupState::Mirror {
                r#type,
                point1: None,
                point2: _,
            } => {
                send_mirror_spawn_event.send(MirrorSpawnEvent {
                    owner: player_entity,
                });
                Some(PowerupState::Mirror {
                    r#type,
                    point1: Some(transform.translation.xy()),
                    point2: None,
                })
            }
            PowerupState::Mirror {
                r#type,
                point1: Some(p1),
                point2: None,
            } => Some(PowerupState::Mirror {
                r#type,
                point1: Some(p1),
                point2: Some(transform.translation.xy()),
            }),
            PowerupState::Mirror {
                r#type,
                point1: Some(p1),
                point2: Some(p2),
            } => {
                send_mirror_use_event.send(MirrorUseEvent {
                    mirror: LineSegment::new(p1, p2),
                    mirror_type: r#type,
                });
                None
            }
        }
    } else {
        None
    };
}

#[derive(Event)]
pub struct MirrorSpawnEvent {
    pub owner: Entity,
}

fn spawn_mirror(
    mut commands: Commands,
    mut events: EventReader<MirrorSpawnEvent>,
    asset_server: Res<AssetServer>,
) {
    for MirrorSpawnEvent { owner } in events.read() {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("textures/mirror.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0.7, 1.5)),
                    ..default()
                },
                ..default()
            },
            Mirror { owner: *owner },
        ));
    }
}

fn draw_mirrors(
    mut mirrors: Query<(Entity, &Mirror, &mut Transform), Without<Player>>,
    players: Query<(&Player, &Transform)>,
    mut commands: Commands,
) {
    for (entity, mirror, mut transform) in mirrors.iter_mut() {
        let mirror = if let Ok((player, player_transform)) = players.get(mirror.owner) {
            match player.powerup {
                Some(PowerupState::Mirror {
                    point1: Some(p1),
                    point2,
                    ..
                }) => Some(LineSegment::new(
                    p1,
                    point2.unwrap_or(player_transform.translation.xy()),
                )),
                _ => None,
            }
        } else {
            None
        };

        if let Some(mirror) = mirror {
            transform.translation = mirror.mid_point().extend(0.0);
            transform.rotation = Quat::from_axis_angle(
                Vec3::Z,
                Vec2::new(0.0, 1.0).angle_between(mirror.get_line().direction()),
            );
            transform.scale = Vec3::new(1.0, mirror.length(), 1.0);
        } else {
            commands.entity(entity).despawn();
        }
    }
}

pub fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(
        Entity,
        &mut Player,
        &mut Velocity,
        &mut Transform,
        &mut TextureAtlasSprite,
    )>,
    mut send_fire_event: EventWriter<BulletFiredEvent>,
    time: Res<Time>,
    mut app_state: ResMut<NextState<AppState>>,
    mut send_mirror_use_event: EventWriter<MirrorUseEvent>,
    mut send_mirror_spawn_event: EventWriter<MirrorSpawnEvent>,
) {
    for (entity, mut player, mut velocity, mut transform, mut sprite) in players.iter_mut() {
        if keyboard_input.pressed(player.key_bindings.left) {
            player_go_left(&mut player, &mut velocity, &mut sprite);
        }
        if keyboard_input.pressed(player.key_bindings.right) {
            player_go_right(&mut player, &mut velocity, &mut sprite);
        }
        if keyboard_input.pressed(player.key_bindings.jump) {
            player_jump(&mut player, &mut velocity);
        }
        if keyboard_input.pressed(player.key_bindings.shoot) {
            player_shoot(&mut player, &mut transform, &mut send_fire_event, &time);
        }
        if keyboard_input.just_pressed(player.key_bindings.powerup) {
            player_use_powerup(
                &mut player,
                entity,
                &mut transform,
                &mut send_mirror_spawn_event,
                &mut send_mirror_use_event,
            )
        }
    }
    if keyboard_input.just_pressed(KeyCode::R) {
        app_state.set(AppState::MainMenu);
    }
}

pub fn jump_reset(
    mut query: Query<(Entity, &mut Player)>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for contact_event in contact_events.read() {
        for (player_entity, mut jumper) in query.iter_mut() {
            set_jumping_false_if_touching_floor(player_entity, &mut jumper, contact_event);
        }
    }
}

fn set_jumping_false_if_touching_floor(
    player_entity: Entity,
    player: &mut Player,
    event: &CollisionEvent,
) {
    if let CollisionEvent::Started(h1, h2, _) = event {
        if h1 == &player_entity || h2 == &player_entity {
            player.is_jumping = false
        }
    }
}

fn check_death_collision(
    mut players: Query<(Entity, &Player)>,
    mut death_zones: Query<(Entity, &DeathZone)>,
    mut contact_events: EventReader<CollisionEvent>,
    mut send_game_over_event: EventWriter<GameOverEvent>,
) {
    for contact_event in contact_events.read() {
        if let CollisionEvent::Started(h1, h2, _) = contact_event {
            if let Ok((player_entity, player)) = players.get(*h1).or(players.get(*h2)) {
                if let Ok((death_zones_entity, death_zones)) =
                    death_zones.get(*h1).or(death_zones.get(*h2))
                {
                    send_game_over_event.send(GameOverEvent {
                        lost_player: player.id,
                    })
                }
            }
        }
    }
}
