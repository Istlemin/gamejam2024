use bevy::prelude::*;
use bevy::ui::ContentSize;
use bevy::{app::Plugin, ecs::schedule::OnEnter};

use crate::game::{PlayerAction, PlayerControls};
use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::MainMenu), main_menu_setup)
            .add_systems(
                Update,
                (
                    menu_action.run_if(in_state(AppState::MainMenu)),
                    select_key_binding.run_if(in_state(AppState::MainMenu)),
                    set_key_binding.run_if(in_state(AppState::MainMenu)),
                ),
            )
            .add_systems(OnExit(AppState::MainMenu), cleanup)
            .add_event::<SelectKeyBinding>()
            .add_event::<SetKeyBinding>();
    }
}

#[derive(Component)]
enum MenuButtonAction {
    Play,
    SelectKeyBinding { player: u32, action: PlayerAction },
}

#[derive(Component, Debug)]
enum MenuState {
    Default,
    SelectKeyBinding {
        player: u32,
        action: PlayerAction,
        text_ref: Entity,
    },
}

#[derive(Event, Clone, Copy)]
pub struct SelectKeyBinding {
    pub player: u32,
    pub action: PlayerAction,
}

#[derive(Event)]
pub struct SetKeyBinding {
    pub key: KeyCode,
}

#[derive(Component)]
struct MainMenu;

fn main_menu_setup(mut commands: Commands, controls: Res<PlayerControls>) {
    commands.spawn((MenuState::Default, MainMenu));
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
                                        parrent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    display: Display::Grid,
                                                    grid_template_columns: vec![
                                                        RepeatedGridTrack::flex(1,5.0),
                                                        RepeatedGridTrack::flex(1,1.0),
                                                        RepeatedGridTrack::flex(1,4.0),
                                                    ],
                                                    row_gap: Val::Px(5.0),
                                                    column_gap: Val::Px(5.0),
                                                    ..default()
                                                },
                                                ..default()
                                            })
                                            .with_children(|parrent| {
                                                for (label, action, note) in [
                                                    ("Left", PlayerAction::Left, ""),
                                                    ("Right", PlayerAction::Right, ""),
                                                    ("Jump", PlayerAction::Jump, ""),
                                                    ("Shoot", PlayerAction::Shoot, ""),
                                                    (
                                                        "Use Powerup",
                                                        PlayerAction::Powerup,
                                                        "(hold and release)",
                                                    ),
                                                    ("Use Butterfly", PlayerAction::Butterfly, ""),
                                                ] {
                                                    parrent
                                                        .spawn(NodeBundle {
                                                            style: Style {
                                                                justify_content:
                                                                    JustifyContent::End,
                                                                ..default()
                                                            },
                                                            ..default()
                                                        })
                                                        .with_children(|parrent| {
                                                            parrent.spawn(
                                                                TextBundle::from_section(
                                                                    format!("{label} = "),
                                                                    TextStyle::default(),
                                                                ),
                                                            );
                                                        });
                                                    parrent
                                                        .spawn(NodeBundle {
                                                            style: Style {
                                                                justify_content:
                                                                    JustifyContent::Center,
                                                                ..default()
                                                            },
                                                            border_color: Color::RED.into(),
                                                            ..default()
                                                        })
                                                        .with_children(|parrent| {
                                                            parrent
                                                                .spawn((ButtonBundle {
                                                                    background_color: Color::GRAY
                                                                        .into(),
                                                                    style: Style {
                                                                        width: Val::Auto,
                                                                        min_width: Val::Px(15.0),
                                                                        padding: UiRect::all(
                                                                            Val::Px(2.0),
                                                                        ),
                                                                        justify_content:
                                                                            JustifyContent::Center,
                                                                        ..default()
                                                                    },
                                                                    ..default()
                                                                }, MenuButtonAction::SelectKeyBinding { player: i as u32, action }))
                                                                .with_children(|parrent| {
                                                                    parrent.spawn(
                                                                        TextBundle::from_section(
                                                                            format!(
                                                                                "{:?}",
                                                                                controls.controls[i][action]
                                                                            ),
                                                                            TextStyle::default(),
                                                                        ),
                                                                    );
                                                                });
                                                        });
                                                    parrent.spawn(TextBundle::from_section(
                                                        note,
                                                        TextStyle::default(),
                                                    ));
                                                }
                                            });
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
    mut select_key_binding_send: EventWriter<SelectKeyBinding>,
    mut set_key_binding_send: EventWriter<SetKeyBinding>,
    menu_state: Query<&MenuState>,
) {
    let menu_state = menu_state.iter().next().expect("Menu state should exist");
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match *menu_button_action {
                MenuButtonAction::Play => app_state.set(AppState::InGame),
                MenuButtonAction::SelectKeyBinding { player, action } => {
                    select_key_binding_send.send(SelectKeyBinding { player, action })
                }
            }
        }
    }
    if let Some(key) = keyboard_input.get_just_pressed().next().copied() {
        println!("{:?}", menu_state);
        if let MenuState::SelectKeyBinding { .. } = menu_state {
            set_key_binding_send.send(SetKeyBinding { key });
        } else {
            app_state.set(AppState::InGame);
        }
    }
}

fn set_key_binding_button_highlighting(mut background_color: Mut<BackgroundColor>, selected: bool) {
    if selected {
        background_color.0 = Color::BLUE;
    } else {
        background_color.0 = Color::GRAY;
    }
}

fn select_key_binding(
    mut select_key_binding: EventReader<SelectKeyBinding>,
    mut selectors: Query<(&MenuButtonAction, &mut BackgroundColor, &Children), With<Button>>,
    mut menu_state: Query<&mut MenuState>,
) {
    let mut menu_state = menu_state
        .iter_mut()
        .next()
        .expect("Menu state should exist");
    for SelectKeyBinding {
        player: select_player,
        action: select_action,
    } in select_key_binding.read()
    {
        for (action, background_color, children) in selectors.iter_mut() {
            if let MenuButtonAction::SelectKeyBinding { player, action } = *action {
                let selected = (player, action) == (*select_player, *select_action);
                if selected {
                    *menu_state = MenuState::SelectKeyBinding {
                        player,
                        action,
                        text_ref: children[0],
                    };
                }
                set_key_binding_button_highlighting(background_color, selected);
            }
        }
    }
}

fn set_key_binding(
    mut set_key_binding: EventReader<SetKeyBinding>,
    mut text: Query<&mut Text>,
    mut menu_state: Query<&mut MenuState>,
    mut buttons: Query<(&mut BackgroundColor, &MenuButtonAction), With<Button>>,
    mut controls: ResMut<PlayerControls>,
) {
    for SetKeyBinding { key } in set_key_binding.read() {
        let mut menu_state = menu_state.iter_mut().next().expect("Menu state should exist");
        let MenuState::SelectKeyBinding {
            player,
            action,
            text_ref,
        } = *menu_state
        else {
            return;
        };
        let mut text = text.get_mut(text_ref).unwrap();

        *text = Text::from_section(format!("{:?}", key), TextStyle::default());
        controls.controls[player as usize][action] = *key;

        for (bc, _) in buttons
            .iter_mut()
            .filter(|(_, action)| matches!(action, MenuButtonAction::SelectKeyBinding { .. }))
        {
            set_key_binding_button_highlighting(bc, false);
        }

        *menu_state = MenuState::Default;
    }
}

fn cleanup(to_despawn: Query<Entity, With<MainMenu>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
