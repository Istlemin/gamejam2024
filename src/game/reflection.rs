use crate::{
    geometry::{LineSegment, Reflectable},
    AppState,
};
use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use super::Bullet;

pub struct ReflectionPlugin;

impl Plugin for ReflectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletMirrorReflectionEvent>().add_systems(
            Update,
            (mirror_reflect_bullets.run_if(in_state(AppState::InGame)),),
        );
    }
}

#[derive(Event)]
struct BulletMirrorReflectionEvent {
    mirror: LineSegment,
}

fn mirror_reflect_bullets(
    mut bullets: Query<(&mut Bullet, &mut Transform, &mut Velocity)>,
    mut reflection_event_reader: EventReader<BulletMirrorReflectionEvent>,
) {
    for event in reflection_event_reader.read() {
        bullets.for_each_mut(|(_, mut transform, mut velocity)| {
            let pos = transform.translation.xy();
            let rot_up = transform.up().xy();
            let velo_2d = velocity.linvel;
            let line = event.mirror.get_line();

            if event.mirror.on_strip(pos) {
                let new_pos = pos.reflect_over_line(line).extend(0.0);
                let new_velo = velo_2d.reflect_over_line(line.centered_line());
                let new_up = rot_up.reflect_over_line(line.centered_line()).extend(0.0);

                transform.translation = new_pos;
                transform.look_at(new_pos + new_velo.extend(0.0), new_up);
                velocity.linvel = new_velo;
            }
        })
    }
}
