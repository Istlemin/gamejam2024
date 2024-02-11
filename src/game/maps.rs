use bevy::math::Vec2;

use super::{MapDescription, PlatformDescription};

pub fn get_map1() -> MapDescription {
    MapDescription {
        platforms: vec![
            PlatformDescription {
                location: Vec2::new(0.0, 0.0),
                width: 10.0,
                height: 1.0,
            },
            PlatformDescription {
                location: Vec2::new(0.0, 10.0),
                width: 10.0,
                height: 1.0,
            },
            PlatformDescription {
                location: Vec2::new(15.0, 5.0),
                width: 10.0,
                height: 1.0,
            },
            PlatformDescription {
                location: Vec2::new(15.0, -5.0),
                width: 10.0,
                height: 1.0,
            },
            PlatformDescription {
                location: Vec2::new(-15.0, 5.0),
                width: 10.0,
                height: 1.0,
            },
            PlatformDescription {
                location: Vec2::new(-15.0, -5.0),
                width: 10.0,
                height: 1.0,
            },
            PlatformDescription {
                location: Vec2::new(-25.0, -5.0),
                width: 10.0,
                height: 1.0,
            },
            PlatformDescription {
                location: Vec2::new(25.0, -5.0),
                width: 10.0,
                height: 1.0,
            },
            PlatformDescription {
                location: Vec2::new(-25.0, 10.0),
                width: 10.0,
                height: 1.0,
            },
            PlatformDescription {
                location: Vec2::new(25.0, 10.0),
                width: 10.0,
                height: 1.0,
            },
        ],

        death_zone: -20.0,
    }
}
