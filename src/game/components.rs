use std::time::Duration;

use bevy::prelude::*;

use crate::geometry::Polygon;

#[derive(Resource)]
pub struct Materials {
    pub player_material: Color,
    pub floor_material: Handle<ColorMaterial>,
    pub bullet_material: Color,
    pub death_zone_material: Color,
}

#[derive(Copy, Clone, PartialEq)]
pub enum GameDirection {
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq)]
pub struct KeyBindings {
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub shoot: KeyCode,
    pub powerup: KeyCode,
}

#[derive(Copy, Clone, PartialEq)]
pub struct MirrorType {
    pub reflect_players: bool,
    pub reflect_platforms: bool,
    pub reflect_bullets: bool,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PowerupState {
    Mirror {
        r#type: MirrorType,
        point1: Option<Vec2>,
        point2: Option<Vec2>,
    },
}

#[derive(Component, Copy, Clone)]
pub struct Player {
    pub speed: f32,
    pub facing_direction: GameDirection,
    pub jump_impulse: f32,
    pub is_jumping: bool,
    pub id: i32,
    pub last_shoot_time: Duration,
    pub shoot_interval: Duration,
    pub key_bindings: KeyBindings,
    pub powerup: Option<PowerupState>,
}

#[derive(Component)]
pub struct Bullet {}

#[derive(Component)]
pub struct DespawnOnRestart {}

#[derive(Component)]
pub struct Platform {
    polygon: Polygon,
}

#[derive(Component)]
pub enum Powerup {
    Mirror(MirrorType),
}

#[derive(Component)]
pub struct DeathZone {}

impl Platform {
    pub fn get_transformed_polygon(&self, transform: &Transform) -> Polygon {
        Polygon::new(
            self.polygon
                .vertices()
                .iter()
                .map(|pt| transform.transform_point(pt.extend(0.0)).xy())
                .collect(),
        )
    }

    pub fn new(polygon: Polygon) -> Self {
        Self { polygon }
    }
}

#[derive(Component)]
pub struct MirrorAnimation {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Mirror {
    pub owner: Entity,
}
