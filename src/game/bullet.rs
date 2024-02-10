use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::AppState;

use super::{GameDirection, Materials, Player, Bullet};

#[derive(Event)]
pub struct BulletFiredEvent {
    pub position: Vec2,
    pub direction: GameDirection,
}


pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletFiredEvent>().add_systems(Update, spawn_bullet.run_if(in_state(AppState::InGame)));
    }
}


pub fn spawn_bullet(
    mut commands: Commands, materials: Res<Materials>,
    mut ev_fired: EventReader<BulletFiredEvent>,
) {

    for BulletFiredEvent{position,direction} in ev_fired.read() {
        
        let velocity;
        match direction {
            GameDirection::Left => {
                velocity = Vec2::new(-10.0,0.0);
            }
            GameDirection::Right => {
                velocity = Vec2::new(10.0,0.0);
            }
        }
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: materials.bullet_material.into(),
                    custom_size: Vec2::new(1.0, 0.1).into(),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.)),
                ..Default::default()
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::cuboid(0.5, 0.5),
            ActiveEvents::COLLISION_EVENTS,
            Bullet {},
        )).insert(Velocity {
            linvel: velocity,
            angvel: 0.0,
        });
    }
}