use std::{ops::Add, time::Duration};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::AppState;

use super::{
    BulletFiredEvent, DeathZone, DespawnOnRestart, GameDirection, KeyBindings, Materials, Player,
};

pub struct PlayerPlugin;

#[derive(Event)]
pub struct GameOverEvent {
    pub lost_player: i32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>()
            .add_systems(OnEnter(AppState::InGame), spawn_players)
            .add_systems(
                Update,
                (
                    //camera_follow_player.run_if(in_state(AppState::InGame)),
                    player_controller.run_if(in_state(AppState::InGame)),
                    jump_reset.run_if(in_state(AppState::InGame)),
                    check_death_collision.run_if(in_state(AppState::InGame)),
                ),
            );
    }
}

fn spawn_players(mut commands: Commands, materials: Res<Materials>) {
    spawn_player(
        0,
        Color::rgb(0.969, 0.200, 0.300),
        &mut commands,
        KeyBindings {
            left: KeyCode::Left,
            right: KeyCode::Right,
            jump: KeyCode::Up,
            shoot: KeyCode::ControlRight,
        },
    );
    spawn_player(
        1,
        Color::rgb(0.300, 0.200, 0.900),
        &mut commands,
        KeyBindings {
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::W,
            shoot: KeyCode::Space,
        },
    );
}

fn spawn_player(player_id: i32, color: Color, commands: &mut Commands, key_bindings: KeyBindings) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: color.into(),
                custom_size: Vec2::new(1.0, 1.0).into(),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.)),
            ..Default::default()
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::cuboid(0.5, 0.5),
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

pub fn player_go_left(player: &mut Player, velocity: &mut Velocity) {
    velocity.linvel = Vec2::new(-player.speed, velocity.linvel.y).into();
    player.facing_direction = GameDirection::Left
}

pub fn player_go_right(player: &mut Player, velocity: &mut Velocity) {
    velocity.linvel = Vec2::new(player.speed, velocity.linvel.y).into();
    player.facing_direction = GameDirection::Right
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
                bullet_pos = Vec2::new(transform.translation.x - 0.7, transform.translation.y);
            }
            GameDirection::Right => {
                bullet_pos = Vec2::new(transform.translation.x + 0.7, transform.translation.y);
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

pub fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Player, &mut Velocity, &mut Transform)>,
    mut send_fire_event: EventWriter<BulletFiredEvent>,
    time: Res<Time>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (mut player, mut velocity, mut transform) in players.iter_mut() {
        if keyboard_input.pressed(player.key_bindings.left) {
            player_go_left(&mut player, &mut velocity);
        }
        if keyboard_input.pressed(player.key_bindings.right) {
            player_go_right(&mut player, &mut velocity);
        }
        if keyboard_input.pressed(player.key_bindings.jump) {
            player_jump(&mut player, &mut velocity);
        }
        if keyboard_input.pressed(player.key_bindings.shoot) {
            player_shoot(&mut player, &mut transform, &mut send_fire_event, &time);
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
