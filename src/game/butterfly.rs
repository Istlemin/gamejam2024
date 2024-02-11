use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::AppState;

use super::{AnimationIndices, AnimationTimer, DespawnOnRestart, Player};

pub struct ButterflyPlugin;

#[derive(Event)]
pub struct ButterflyEvent {
    pub player_id: i32,
}
impl Plugin for ButterflyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButterflyEvent>()
        .add_systems(OnEnter(AppState::InGame), spawn_butterfly)
        .add_systems(Update, (move_butterfly, butterfly_use).run_if(in_state(AppState::InGame)))
            // .add_systems(
            //     Update,
            //     (
            //         //camera_follow_player.run_if(in_state(AppState::InGame)),
            //         // animate_sprite.run_if(in_state(AppState::InGame)),
            //     ),
            // )
            ;
    }
}

#[derive(Component, Clone)]
pub struct Butterfly {}

fn spawn_butterfly(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/butterfly.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(70.0, 65.0), 13, 7, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 86 };
    let scale = Vec3 {
        x: 0.02,
        y: 0.02,
        z: 0.02,
    };

    commands.spawn((
        Butterfly {},
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(scale),
            ..Default::default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Direction::Up,
        DespawnOnRestart {},
    ));
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn move_butterfly(time: Res<Time>, mut position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut position {
        match *logo {
            Direction::Up => transform.translation.y += 3. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 3. * time.delta_seconds(),
        }

        if transform.translation.y > 20. {
            *logo = Direction::Down;
        } else if transform.translation.y < -20. {
            *logo = Direction::Up;
        }
    }
}

fn reflect_player_through_point(
    velocity: &mut Velocity,
    transform: &mut Transform,
    reflection_point: Transform,
) {
    let pos = transform.translation;
    let reflection_pos = reflection_point.translation;
    let new_pos = reflection_pos + reflection_pos - pos;
    transform.translation = new_pos;
    velocity.linvel = velocity.linvel * -1.0;
}

fn butterfly_use(
    butterfly: Query<(&Butterfly, &Transform), Without<Player>>,
    mut players: Query<(&Player, &mut Transform, &mut Velocity), Without<Butterfly>>,
    mut butterfly_event: EventReader<ButterflyEvent>,
) {
    for event in butterfly_event.read() {
        for (_butterfly, butterfly_position) in butterfly.iter() {
            for (player, mut player_position, mut player_velocity) in players.iter_mut() {
                if (player.id == event.player_id) {
                    reflect_player_through_point(
                        &mut player_velocity,
                        &mut player_position,
                        *butterfly_position,
                    )
                }
            }
        }
    }
}
