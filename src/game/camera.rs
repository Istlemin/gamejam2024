use bevy::{math::Vec3, prelude::*, render::camera::ScalingMode};

pub fn new_camera_2d() -> Camera2dBundle {
    let mut camera = Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: 100.0,
                max_height: 50.0,
            },
            ..default()
        },
        ..default()
    };
    camera.transform.translation = Vec3::new(0.0, 0.0, 10.0);
    camera.transform.look_at(Vec3::ZERO, Vec3::Y);
    return camera;
}
