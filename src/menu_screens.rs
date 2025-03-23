use bevy::prelude::*;

use crate::gamestate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>();
        app.add_systems(
            OnEnter(MenuState::Main),
            main_screen.run_if(in_state(GameState::MainMenu)),
        );
        app.add_systems(
            OnEnter(MenuState::Credits),
            credit_screen.run_if(in_state(GameState::MainMenu)),
        );
        app.add_systems(Update, button_system.run_if(in_state(GameState::MainMenu)));
        app.add_systems(OnExit(GameState::MainMenu), despawn_menu);
        app.add_systems(OnExit(MenuState::Main), despawn_menu);
        app.add_systems(OnExit(MenuState::GeneralSettings), despawn_menu);
        app.add_systems(OnExit(MenuState::AudioSettings), despawn_menu);
        app.add_systems(OnExit(MenuState::ControlSettings), despawn_menu);
        app.add_systems(OnExit(MenuState::Credits), despawn_menu);
        app.add_systems(OnEnter(MenuState::Quit), exit_app);
    }
}

const NORMAL_BUTTON_COLOR: Color = Color::srgba(0.1, 0.4, 0.4, 0.3);
const PRESSED_BUTTON_COLOR: Color = Color::srgba(0.1, 0.4, 0.1, 0.5);
const HOVERED_BUTTON_COLOR: Color = Color::srgba(0.1, 0.4, 0.1, 0.3);

#[derive(Component)]
struct MainScreenUI;
#[derive(Component)]
struct GeneralSettingsScreenUI;
#[derive(Component)]
struct AudioSettingsScreenUI;
#[derive(Component)]
struct ControlSettingsScreenUI;
#[derive(Component)]
struct CreditScreenUI;

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
enum MenuState {
    #[default]
    Main,
    GeneralSettings,
    AudioSettings,
    ControlSettings,
    Credits,
    Quit,
}

#[derive(Component, Debug)]
enum NextStateDestination {
    Menu(MenuState),
    Game(GameState),
}

fn main_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/MatrixtypeDisplay-9MyE5.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(95.0),
                height: Val::Percent(95.0),
                align_items: AlignItems::FlexStart,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::SpaceBetween,
                justify_self: JustifySelf::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            MainScreenUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Assassin"),
                TextFont {
                    font: font.clone(),
                    font_size: 50.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
            ));
            parent
                .spawn((
                    Node {
                        width: Val::Auto,
                        height: Val::Auto,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(5.)),
                        justify_content: JustifyContent::SpaceBetween,
                        row_gap: Val::Px(5.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.05)),
                    BorderRadius::all(Val::Px(10.0)),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Auto,
                                height: Val::Px(50.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                padding: UiRect::all(Val::Px(5.)),
                                ..default()
                            },
                            BorderColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            BorderRadius::MAX,
                            NextStateDestination::Game(GameState::InGame),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Start Game"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 15.0,
                                    ..default()
                                },
                                TextColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                            ));
                        });
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Auto,
                                height: Val::Px(50.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                padding: UiRect::all(Val::Px(5.)),
                                ..default()
                            },
                            BorderColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            BorderRadius::MAX,
                            NextStateDestination::Menu(MenuState::GeneralSettings),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Settings"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 15.0,
                                    ..default()
                                },
                                TextColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                            ));
                        });
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Auto,
                                height: Val::Px(50.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                padding: UiRect::all(Val::Px(5.)),
                                ..default()
                            },
                            BorderColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            BorderRadius::MAX,
                            NextStateDestination::Menu(MenuState::Credits),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Credits"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 15.0,
                                    ..default()
                                },
                                TextColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                            ));
                        });
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Auto,
                                height: Val::Px(50.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                padding: UiRect::all(Val::Px(5.)),
                                ..default()
                            },
                            BorderColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            BorderRadius::MAX,
                            NextStateDestination::Menu(MenuState::Quit),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Quit"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 15.0,
                                    ..default()
                                },
                                TextColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                            ));
                        });
                });
        });
}

fn credit_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/MatrixtypeDisplay-9MyE5.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(70.),
                height: Val::Percent(70.0),
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::SpaceBetween,
                justify_self: JustifySelf::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.05)),
            BorderRadius::all(Val::Px(10.0)),
            BorderColor(Color::srgb(0.0, 0.0, 0.0)),
            CreditScreenUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Credits"),
                        TextFont {
                            font: font.clone(),
                            font_size: 50.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                    ));
                });
            parent
                .spawn(Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(5.)),
                    justify_content: JustifyContent::SpaceBetween,
                    row_gap: Val::Px(5.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Author: Sibald Hulselmans"),
                        TextColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                        TextFont {
                            font: font.clone(),
                            font_size: 15.0,
                            ..default()
                        },
                    ));
                });
            parent
                .spawn((
                    Node {
                        width: Val::Px(80.),
                        height: Val::Px(30.),
                        align_self: AlignSelf::Start,
                        justify_self: JustifySelf::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    Button,
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    BorderRadius::MAX,
                    NextStateDestination::Menu(MenuState::Main),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Back"),
                        TextFont {
                            font: font.clone(),
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                    ));
                });
        });
}

#[allow(clippy::type_complexity)]
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &NextStateDestination),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_color, destination) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(PRESSED_BUTTON_COLOR);
                match destination {
                    NextStateDestination::Menu(state) => {
                        next_game_state.set(GameState::default());
                        next_menu_state.set(state.clone());
                    }
                    NextStateDestination::Game(state) => {
                        next_menu_state.set(MenuState::default());
                        next_game_state.set(state.clone());
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(HOVERED_BUTTON_COLOR);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(NORMAL_BUTTON_COLOR);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn despawn_menu(
    mut commands: Commands,
    query: Query<
        Entity,
        Or<(
            With<MainScreenUI>,
            With<GeneralSettingsScreenUI>,
            With<AudioSettingsScreenUI>,
            With<ControlSettingsScreenUI>,
            With<CreditScreenUI>,
        )>,
    >,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn exit_app(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}
