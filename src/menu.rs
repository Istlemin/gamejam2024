use bevy::prelude::*;
use bevy::{app::Plugin, ecs::schedule::OnEnter};

use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::MainMenu), main_menu_setup)
            .add_systems(Update, menu_action.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), cleanup);
    }
}

#[derive(Component)]
enum MenuButtonAction {
    Play,
}

#[derive(Component)]
struct MainMenu;

fn main_menu_setup(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            MainMenu,
        ))
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

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => app_state.set(AppState::InGame),
            }
        }
    }
}

fn cleanup(to_despawn: Query<Entity, With<MainMenu>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
