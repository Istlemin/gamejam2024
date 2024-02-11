use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use bevy_rapier2d::prelude::*;

use crate::AppState;

use super::{
    butterfly::ButterflyEvent, reflections::ReflectionEvent, AnimationIndices, AnimationTimer, BulletFiredEvent, DeathZone, DespawnOnRestart, GameDirection, KeyBindings, Mirror, MirrorType, Platform, Player, PowerupState
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
            .add_event::<PlayerSpawnEvent>()
            .add_event::<MirrorUseEvent>()
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
                        use_mirror,
                    )
                        .run_if(in_state(AppState::InGame)),
                ),
            );
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Player,
    )>,
) {
    for (indices, mut timer, mut sprite, player) in &mut query {
        if player.is_running {
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = if sprite.index == indices.last {
                    indices.first
                } else {
                    sprite.index + 1
                };
            }
        } else {
            sprite.index = 0;
        }
    }
}

pub fn spawn_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut spawn_event_sender: EventWriter<PlayerSpawnEvent>,
) {
    info!("Spawning Players");
    spawn_player(
        0,
        Transform::from_xyz(-1.0, 7.0, 0.0),
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
            butterfly: KeyCode::F,
        },
        &mut spawn_event_sender,
    );
    spawn_player(
        1,
        Transform::from_xyz(1.0, 7.0, 0.0),
        Color::rgb(0.969, 0.200, 0.300),
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        KeyBindings {
            left: KeyCode::Left,
            right: KeyCode::Right,
            jump: KeyCode::Up,
            shoot: KeyCode::Period,
            powerup: KeyCode::ShiftRight,
            butterfly: KeyCode::Slash,
        },
        &mut spawn_event_sender,
    );
}

#[derive(Event)]
pub struct PlayerSpawnEvent {
    pub player_id: i32,
    pub player: Entity,
}

fn spawn_player(
    player_id: i32,
    position: Transform,
    color: Color,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    key_bindings: KeyBindings,
    spawn_event_sender: &mut EventWriter<PlayerSpawnEvent>,
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

    let entity = commands.spawn((
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
            last_butterfly_time: Duration::new(0, 0),
            butterfly_interval: Duration::new(3, 0),
            key_bindings,
            powerup: None,
            is_running: false,
        },
        Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        },
        DespawnOnRestart {},
    ));
    spawn_event_sender.send(PlayerSpawnEvent {
        player_id,
        player: entity.id(),
    })
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
    sprite.flip_x = true;
    player.is_running = true;
}

pub fn player_go_right(
    player: &mut Player,
    velocity: &mut Velocity,
    sprite: &mut TextureAtlasSprite,
) {
    velocity.linvel = Vec2::new(player.speed, velocity.linvel.y).into();
    player.facing_direction = GameDirection::Right;
    sprite.flip_x = false;
    player.is_running = true;
}

pub fn player_jump(player: &mut Player, velocity: &mut Velocity) {
    if !player.is_jumping {
        player.is_jumping = true;
        velocity.linvel = Vec2::new(velocity.linvel.x, player.jump_impulse).into();
    }
}

pub fn player_butterfly(
    player: &mut Player,
    send_butterfly_event: &mut EventWriter<ButterflyEvent>,
    time: &Res<Time>,
) {
    if player.last_butterfly_time + player.butterfly_interval > time.elapsed() {
        return;
    } else {
        let event = ButterflyEvent {
            player_id: player.id,
        };

        send_butterfly_event.send(event);

        player.last_butterfly_time = time.elapsed();
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

pub fn player_powerup_press(
    player: &mut Player,
    player_entity: Entity,
    send_mirror_spawn_event: &mut EventWriter<MirrorSpawnEvent>,
) {
    player.powerup = if let Some(powerup) = &player.powerup {
        debug!("Powerup activated");
        match powerup {
            PowerupState::Mirror {
                r#type,
                placed: false,
            } => {
                send_mirror_spawn_event.send(MirrorSpawnEvent {
                    owner: player_entity,
                });
                Some(PowerupState::Mirror {
                    r#type: *r#type,
                    placed: true,
                })
            }
            state => Some(*state),
        }
    } else {
        None
    };
}

pub fn player_powerup_release(
    player: &mut Player,
    player_entity: Entity,
    send_mirror_use_event: &mut EventWriter<MirrorUseEvent>,
) {
    player.powerup = if let Some(powerup) = player.powerup {
        debug!("Powerup released");
        match powerup {
            PowerupState::Mirror {
                r#type,
                placed: true,
            } => {
                send_mirror_use_event.send(MirrorUseEvent {
                    owner: player_entity,
                    mirror_type: r#type,
                });
                None
            }
            state => Some(state),
        }
    } else {
        None
    }
}

#[derive(Event)]
pub struct MirrorUseEvent {
    pub owner: Entity,
    pub mirror_type: MirrorType,
}

#[derive(Event)]
pub struct MirrorSpawnEvent {
    pub owner: Entity,
}

fn use_mirror(
    mut commands: Commands,
    mut events: EventReader<MirrorUseEvent>,
    mut reflection_events_send: EventWriter<ReflectionEvent>,
    mirrors: Query<(Entity, &Mirror)>,
) {
    for MirrorUseEvent { owner, mirror_type } in events.read() {
        for (entity, mirror) in mirrors.iter().filter(|(_, mirror)| mirror.owner == *owner) {
            reflection_events_send.send(ReflectionEvent {
                mirror_type: *mirror_type,
                mirror: mirror.get_line(),
            });
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_mirror(
    mut commands: Commands,
    mut events: EventReader<MirrorSpawnEvent>,
    players: Query<&Transform>,
    asset_server: Res<AssetServer>,
) {
    for MirrorSpawnEvent { owner } in events.read() {
        if let Ok(transform) = players.get(*owner) {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("textures/mirror.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(0.7, 1.5)),
                        ..default()
                    },
                    ..default()
                },
                Mirror {
                    owner: *owner,
                    time: Stopwatch::new(),
                    position: transform.translation.xy(),
                },
                DespawnOnRestart {},
            ));
        }
    }
}

fn draw_mirrors(
    mut mirrors: Query<(&mut Mirror, &mut Transform), Without<Player>>,
    time: Res<Time>,
) {
    for (mut mirror, mut transform) in mirrors.iter_mut() {
        mirror.time.tick(time.delta());
        let mirror = mirror.get_line();

        transform.translation = mirror.mid_point().extend(0.0);
        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            Vec2::new(0.0, 1.0).angle_between(mirror.get_line().direction()),
        );
        transform.scale = Vec3::new(1.0, mirror.length(), 1.0);
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
    mut send_butterfly_event: EventWriter<ButterflyEvent>,
) {
    for (entity, mut player, mut velocity, mut transform, mut sprite) in players.iter_mut() {
        player.is_running = false;
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

        if keyboard_input.just_pressed(player.key_bindings.butterfly) {
            player_butterfly(&mut player, &mut send_butterfly_event, &time);
        }
        if keyboard_input.just_pressed(player.key_bindings.powerup) {
            player_powerup_press(&mut player, entity, &mut send_mirror_spawn_event)
        }
        if keyboard_input.just_released(player.key_bindings.powerup) {
            player_powerup_release(&mut player, entity, &mut send_mirror_use_event)
        }
    }
    if keyboard_input.just_pressed(KeyCode::R) {
        app_state.set(AppState::MainMenu);
    }
}

pub fn jump_reset(
    mut query: Query<(Entity, &mut Player)>,
    mut platforms: Query<(Entity, &Platform)>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for contact_event in contact_events.read() {
        for (player_entity, mut player) in query.iter_mut() {
            if let CollisionEvent::Started(h1, h2, _) = contact_event {
                if let Ok((platform_entity, platform)) = platforms.get(*h1).or(platforms.get(*h2)) {
                    if h1 == &player_entity || h2 == &player_entity {
                        player.is_jumping = false
                    }
                }
            }
        }
    }
}

fn check_death_collision(
    mut players: Query<(&Transform, &Player)>,
    //mut death_zones: Query<(Entity, &DeathZone)>,
    //mut contact_events: EventReader<CollisionEvent>,
    mut send_game_over_event: EventWriter<GameOverEvent>,
) {
    for (transform, player) in players.iter() {
        if transform.translation.y < -40.0 {
            send_game_over_event.send(GameOverEvent {
                lost_player: player.id,
            })
        }
    }
    // for contact_event in contact_events.read() {
    //     if let CollisionEvent::Started(h1, h2, _) = contact_event {
    //         if let Ok((player_entity, player)) = players.get(*h1).or(players.get(*h2)) {
    //             if let Ok((death_zones_entity, death_zones)) =
    //                 death_zones.get(*h1).or(death_zones.get(*h2))
    //             {
    //                 send_game_over_event.send(GameOverEvent {
    //                     lost_player: player.id,
    //                 })
    //             }
    //         }
    //     }
    // }
}

fn reflect_player_through_point(mut transform: Transform, reflection_point: Transform) {
    let pos = transform.translation;
    let reflection_pos = reflection_point.translation;
    let new_pos = reflection_pos + reflection_pos - pos;
    transform = transform.with_translation(new_pos);
}
