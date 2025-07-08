use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{game::components::HitCounter, AppState};

use super::{Bullet, DespawnOnRestart, GameDirection, LifeTimer, Materials, Player};

#[derive(Event)]
pub struct BulletFiredEvent {
    pub position: Vec2,
    pub direction: GameDirection,
}

#[derive(Event)]
pub struct BulletHitEvent {
    pub target: Entity,
    pub bullet: Entity,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletFiredEvent>()
            .add_event::<BulletHitEvent>()
            .add_systems(
                Update,
                (
                    spawn_bullet.run_if(in_state(AppState::InGame)),
                    check_player_hit.run_if(in_state(AppState::InGame)),
                    player_hit.run_if(in_state(AppState::InGame)),
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
                velocity = Vec2::new(-40.0, 0.0);
            }
            GameDirection::Right => {
                velocity = Vec2::new(40.0, 0.0);
            }
        }
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: materials.bullet_material.into(),
                        custom_size: Vec2::new(0.4, 0.1).into(),
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
            .insert(LifeTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    }
}

fn player_hit(
    mut commands: Commands,
    mut player_velocities: Query<(Entity, &mut Velocity, &mut HitCounter), (With<Player>, Without<Bullet>)>,
    bullet_velocities: Query<(Entity, &Velocity), (With<Bullet>, Without<Player>)>,
    mut ev_hit: EventReader<BulletHitEvent>,
) {
    for BulletHitEvent { target, bullet } in ev_hit.read() {
        // debug!("Bullet hit event!");
        if let Ok((_, mut player_velocity, mut hit_counter)) = player_velocities.get_mut(*target) {
            // debug!("Matched player!");
            if let Ok((_, bullet_velocity)) = bullet_velocities.get(*bullet) {
                debug!("Hit counter: {}", hit_counter.count);
                //debug!("Matched bullet!");
                player_velocity.linvel += bullet_velocity.linvel.normalize() * hit_counter.count;
                hit_counter.hit();
            }
        }

        if let Some(mut entity_commands) = commands.get_entity(*bullet) {
            entity_commands.despawn();
        }
    }
}

pub fn check_player_hit(
    players: Query<Entity, With<Player>>,
    bullets: Query<Entity, With<Bullet>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut send_hit_event: EventWriter<BulletHitEvent>,
) {
    for contact_event in contact_events.read() {
        if let CollisionEvent::Started(h1, h2, _) = contact_event {
            // let player_result1 = players.get_mut(*h1);
            // let player_result2 = players.get_mut(*h2);
            if let Ok(player_entity) = players.get(*h1).or(players.get(*h2)) {
                if let Ok(bullet_entity) = bullets.get(*h1).or(bullets.get(*h2)) {
                    send_hit_event.send(BulletHitEvent {
                        target: player_entity,
                        bullet: bullet_entity,
                    });
                    debug!("Sending bullet event!");
                    //player_hit(&mut commands, player_entity, bullet_entity);
                }
            }
        }
    }
}
