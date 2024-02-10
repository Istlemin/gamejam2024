use bevy::prelude::{Color, Resource};

#[derive(Resource)]
pub struct Materials {
    pub player_material: Color,
    pub floor_material: Color,
    pub bullet_material: Color,
}
