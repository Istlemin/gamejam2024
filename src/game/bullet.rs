use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::AppState;

use super::{Bullet, DespawnOnRestart, GameDirection, Materials, Player};

#[derive(Event)]
pub struct BulletFiredEvent {
    pub position: Vec2,
    pub direction: GameDirection,
}

#[derive(Component)]
struct BulletLifeTimer(Timer);

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletFiredEvent>().add_systems(
            Update,
            (
                spawn_bullet.run_if(in_state(AppState::InGame)),
                check_player_hit.run_if(in_state(AppState::InGame)),
                tick_bullet_timers.run_if(in_state(AppState::InGame)),
            ),
        );
    }
}

pub fn spawn_bullet(
    mut commands: Commands,
    materials: Res<Materials>,
    mut ev_fired: EventReader<BulletFiredEvent>,
) {
    for BulletFiredEvent {
        position,
        direction,
    } in ev_fired.read()
    {
        let velocity;
        match direction {
            GameDirection::Left => {
                velocity = Vec2::new(-20.0, 0.0);
            }
            GameDirection::Right => {
                velocity = Vec2::new(20.0, 0.0);
            }
        }
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: materials.bullet_material.into(),
                        custom_size: Vec2::new(0.4, 0.06).into(),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.)),
                    ..Default::default()
                },
                RigidBody::KinematicVelocityBased,
                //LockedAxes::ROTATION_LOCKED,
                //GravityScale(0.0),
                Collider::cuboid(0.02, 0.03),
                ActiveEvents::COLLISION_EVENTS,
                Bullet {},
                Velocity {
                    linvel: velocity,
                    angvel: 0.0,
                },
                DespawnOnRestart {},
            ))
            .insert(BulletLifeTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    }
}

fn tick_bullet_timers(
    mut commands: Commands,
    mut cooldowns: Query<(Entity, &mut BulletLifeTimer)>,
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

fn player_hit(commands: &mut Commands, player_entity: Entity, bullet_entity: Entity) {
    commands.entity(bullet_entity).despawn();
}

pub fn check_player_hit(
    mut commands: Commands,
    players: Query<(Entity, &Player)>,
    bullets: Query<(Entity, &Bullet)>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for contact_event in contact_events.read() {
        if let CollisionEvent::Started(h1, h2, _) = contact_event {
            // let player_result1 = players.get_mut(*h1);
            // let player_result2 = players.get_mut(*h2);
            if let Ok((player_entity, player)) = players.get(*h1).or(players.get(*h2)) {
                if let Ok((bullet_entity, bullet)) = bullets.get(*h1).or(bullets.get(*h2)) {
                    player_hit(&mut commands, player_entity, bullet_entity);
                }
            }
        }
    }
}
