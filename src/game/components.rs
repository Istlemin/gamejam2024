use std::{
    ops::{Index, IndexMut},
    time::Duration,
};

use bevy::{prelude::*, time::Stopwatch};

use crate::geometry::{LineSegment, Polygon};

#[derive(Copy, Clone, PartialEq)]
pub struct PlatformDescription {
    pub location: Vec2,
    pub width: f32,
    pub height: f32,
}

#[derive(Resource)]
pub struct MapDescription {
    pub platforms: Vec<PlatformDescription>,
    pub death_zone: f32,
}

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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerAction {
    Left,
    Right,
    Jump,
    Shoot,
    Powerup,
    Butterfly,
}

#[derive(Copy, Clone, PartialEq)]
pub struct KeyBindings {
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub shoot: KeyCode,
    pub powerup: KeyCode,
    pub butterfly: KeyCode,
}

impl Index<PlayerAction> for KeyBindings {
    type Output = KeyCode;

    fn index(&self, action: PlayerAction) -> &Self::Output {
        match action {
            PlayerAction::Left => &self.left,
            PlayerAction::Right => &self.right,
            PlayerAction::Jump => &self.jump,
            PlayerAction::Shoot => &self.shoot,
            PlayerAction::Powerup => &self.powerup,
            PlayerAction::Butterfly => &self.butterfly,
        }
    }
}

impl IndexMut<PlayerAction> for KeyBindings {
    fn index_mut(&mut self, action: PlayerAction) -> &mut Self::Output {
        match action {
            PlayerAction::Left => &mut self.left,
            PlayerAction::Right => &mut self.right,
            PlayerAction::Jump => &mut self.jump,
            PlayerAction::Shoot => &mut self.shoot,
            PlayerAction::Powerup => &mut self.powerup,
            PlayerAction::Butterfly => &mut self.butterfly,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct MirrorType {
    pub reflect_players: bool,
    pub reflect_platforms: bool,
    pub reflect_bullets: bool,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PowerupState {
    Mirror { r#type: MirrorType, placed: bool },
}

#[derive(Component, Clone)]
pub struct Player {
    pub speed: f32,
    pub facing_direction: GameDirection,
    pub jump_impulse: f32,
    pub is_jumping: bool,
    pub id: i32,
    pub last_shoot_time: Duration,
    pub shoot_interval: Duration,
    pub last_butterfly_time: Duration,
    pub butterfly_interval: Duration,
    pub key_bindings: KeyBindings,
    pub powerup: Option<PowerupState>,
    pub is_running: bool,
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
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct LifeTimer(pub Timer);

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
            self.polygon.texture_coords().clone(),
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

const MIRROR_ANGULAR_VEL: f32 = 3.0;
const MIRROR_HALF_HEIGHT: f32 = 2.0;

#[derive(Component)]
pub struct Mirror {
    pub owner: Entity,
    pub time: Stopwatch,
    pub position: Vec2,
}

impl Mirror {
    pub fn get_line(&self) -> LineSegment {
        let offset =
            Vec2::from_angle(self.time.elapsed_secs() * MIRROR_ANGULAR_VEL) * MIRROR_HALF_HEIGHT;
        LineSegment::new(self.position + offset, self.position - offset)
    }
}

#[derive(Resource)]
pub struct PlayerControls {
    pub controls: Vec<KeyBindings>,
}
