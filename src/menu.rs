use bevy::prelude::*;
use bevy::{app::Plugin, ecs::schedule::OnEnter};

use crate::game::PlayerControls;
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

fn main_menu_setup(mut commands: Commands, controls: Res<PlayerControls>) {
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
                .spawn(NodeBundle {
                    style: Style {
                        // This will display its children in a column, from top to bottom
                        flex_direction: FlexDirection::Column,
                        // `align_items` will align children on the cross axis. Here the main axis is
                        // vertical (column), so the cross axis is horizontal. This will center the
                        // children
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
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
                    parrent.spawn(TextBundle::from_section(
                        "Controls",
                        TextStyle {
                            font_size: 20.0,
                            ..default()
                        },
                    ));
                    parrent
                        .spawn(NodeBundle::default())
                        .with_children(|parrent| {
                            for i in 0..2 {
                                parrent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            // This will display its children in a column, from top to bottom
                                            flex_direction: FlexDirection::Column,
                                            // `align_items` will align children on the cross axis. Here the main axis is
                                            // vertical (column), so the cross axis is horizontal. This will center the
                                            // children
                                            align_items: AlignItems::Center,
                                            padding: UiRect {
                                                left: Val::Px(20.0),
                                                right: Val::Px(20.0),
                                                top: Val::Px(20.0),
                                                bottom: Val::Px(20.0),
                                            },
                                            ..default()
                                        },
                                        background_color: Color::BLACK.into(),
                                        ..default()
                                    })
                                    .with_children(|parrent| {
                                        parrent.spawn(TextBundle::from_section(
                                            format!("Player {}", i + 1),
                                            TextStyle {
                                                font_size: 20.0,
                                                ..default()
                                            },
                                        ));
                                        parrent.spawn(TextBundle::from_section(
                                            format!("Left = {:?}", controls.controls[i].left),
                                            TextStyle::default(),
                                        ));
                                        parrent.spawn(TextBundle::from_section(
                                            format!("Right = {:?}", controls.controls[i].right),
                                            TextStyle::default(),
                                        ));
                                        parrent.spawn(TextBundle::from_section(
                                            format!("Jump = {:?}", controls.controls[i].jump),
                                            TextStyle::default(),
                                        ));
                                        parrent.spawn(TextBundle::from_section(
                                            format!("Shoot = {:?}", controls.controls[i].shoot),
                                            TextStyle::default(),
                                        ));
                                        parrent.spawn(TextBundle::from_section(
                                            format!(
                                                "Use Powerup = {:?} (hold and release)",
                                                controls.controls[i].powerup
                                            ),
                                            TextStyle::default(),
                                        ));
                                        parrent.spawn(TextBundle::from_section(
                                            format!(
                                                "Use Butterfly = {:?}",
                                                controls.controls[i].butterfly
                                            ),
                                            TextStyle::default(),
                                        ));
                                    });
                            }
                        });
                });
        });
}

fn menu_action(
    keyboard_input: Res<Input<KeyCode>>,
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
    if keyboard_input.get_just_pressed().next().is_some() {
        app_state.set(AppState::InGame);
    }
}

fn cleanup(to_despawn: Query<Entity, With<MainMenu>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
