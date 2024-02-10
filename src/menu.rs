use bevy::prelude::*;
use bevy::{app::Plugin, ecs::schedule::OnEnter};

use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::MainMenu), main_menu_setup)
        .add_systems(Startup, main_menu_setup);
    }
}

#[derive(Component)]
enum MenuButtonAction {
    Play,
}

fn main_menu_setup(mut commands: Commands) {
    info!("What?");
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parrent| {
            parrent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(20.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::BLUE.into(),
                        ..default()
                    },
                    MenuButtonAction::Play,
                ))
                .with_children(|parrent| {
                    parrent.spawn(TextBundle::from_section("Play", TextStyle::default()));
                });
        });
}
