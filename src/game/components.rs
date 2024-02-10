use std::time::{Duration, Instant};

use bevy::prelude::*;

#[derive(Resource)]
pub struct Materials {
    pub player_material: Color,
    pub floor_material: Handle<ColorMaterial>,
    pub bullet_material: Color,
}

#[derive(Copy, Clone, PartialEq)]
pub enum GameDirection {
    Left,
    Right,
}

#[derive(Component)]

pub struct Player {
    pub speed: f32,
    pub facing_direction: GameDirection,
    pub jump_impulse: f32,
    pub is_jumping: bool,
    pub id: i32,
    pub last_shoot_time: Duration,
    pub shoot_interval: Duration,
}

#[derive(Component)]
pub struct Bullet {}

#[derive(Component)]
pub struct Platform;

#[derive(Component)]
pub struct DespawnOnRestart {}
