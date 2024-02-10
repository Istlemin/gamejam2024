use bevy::prelude::*;

#[derive(Resource)]
pub struct Materials {
    pub player_material: Color,
    pub floor_material: Color,
    pub bullet_material: Color,
}

#[derive(Copy, Clone)]
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
    pub id:i32,
}

#[derive(Component)]
pub struct Platform;
