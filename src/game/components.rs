use bevy::prelude::*;

#[derive(Resource)]
pub struct Materials {
    pub player_material: Color,
    pub floor_material: Color,
    pub bullet_material: Color,
}

#[derive(Component)]
pub struct Player;
