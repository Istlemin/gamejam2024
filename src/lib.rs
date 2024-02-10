use bevy::{prelude::*, window::PresentMode, window::WindowMode};
use wasm_bindgen::prelude::wasm_bindgen;
// use wasm_bindgen::prelude::*;

mod game;
mod menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame
}

use game::GamePlugin;
use menu::MenuPlugin;

#[wasm_bindgen]
pub fn run() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Platformer!".to_string(),
            resolution: (640.0, 400.0).into(),
            mode: WindowMode::Windowed,
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugins((GamePlugin, MenuPlugin))
    .run();
}
