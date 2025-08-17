use bevy::{prelude::*, window::PresentMode, window::WindowMode};
// use wasm_bindgen::prelude::*;
use bevy::log::LogPlugin;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

mod game;
mod menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

use game::{GamePlugin, KeyBindings, PlayerControls};
use menu::MenuPlugin;

mod geometry;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn run() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Platformer!".to_string(),
                    resolution: (1040.0, 800.0).into(),
                    mode: WindowMode::Windowed,
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            })
            .set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,gamejam2024=debug".into(),
                level: bevy::log::Level::DEBUG,
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
    .insert_resource(PlayerControls {
        controls: vec![
            KeyBindings {
                left: KeyCode::KeyA,
                right: KeyCode::KeyD,
                jump: KeyCode::KeyW,
                shoot: KeyCode::KeyC,
                powerup: KeyCode::KeyV,
                butterfly: KeyCode::KeyB,
            },
            KeyBindings {
                left: KeyCode::ArrowLeft,
                right: KeyCode::ArrowRight,
                jump: KeyCode::ArrowUp,
                shoot: KeyCode::Comma,
                powerup: KeyCode::Period,
                butterfly: KeyCode::Slash,
            },
        ],
    })
    .add_plugins((GamePlugin, MenuPlugin))
    .init_state::<AppState>()
    .run();
}
