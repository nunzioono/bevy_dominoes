mod utils;

use bevy::prelude::*;
use utils::default_plugins;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(default_plugins())
        .init_state::<AppState>() // Initialize the state as a resource
        .add_systems(Startup, setup)
        .add_systems(Update, click_handler) // Add the click handler system
        .add_systems(Update, update_text)
        .run();
}

#[derive(Component)]
struct MyText;

fn update_text (
    state: Res<State<AppState>>,
    mut query: Query<(&mut Text, &mut MyText)>
) {
    let new_text = match state.get() {
        AppState::Menu => "InGame",
        AppState::InGame => "Menu",
    };
    let mut mytext = query.single_mut().0;
    mytext.sections[0].value = new_text.to_string();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Text entity
    commands.spawn((
        TextBundle::from_section(
            "Menu".to_string(),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            },
        ),
        MyText
    ));
}

fn click_handler(
    mouse_input: Res<ButtonInput<MouseButton>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        match state.get() {
            AppState::Menu => {
                next_state.set(AppState::InGame);
            },
            AppState::InGame => {
                next_state.set(AppState::Menu);
            },
        };
    }
}
