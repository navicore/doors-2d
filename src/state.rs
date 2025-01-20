use bevy::{prelude::*, text::TextBounds};

#[derive(Debug, Component)]
struct GameOverText;

#[derive(Debug, Component)]
struct GameOverCamera;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    GameOver,
    GameRestart,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(
                Update,
                (
                    game_state_input_events,
                    transition_to_game_restart.run_if(in_state(GameState::GameOver)),
                ),
            )
            .add_systems(
                OnEnter(GameState::GameOver),
                (spawn_game_over_camera, display_game_over_text).chain(),
            )
            .add_systems(
                OnEnter(GameState::GameRestart),
                (remove_game_over_text, despawn_game_over_camera).chain(),
            );
    }
}

pub fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::InGame),
            GameState::GameOver | GameState::GameRestart => (),
        }
    } else if keyboard_input.just_pressed(KeyCode::KeyQ) {
        // exit the game
        std::process::exit(0);
    }
}

fn spawn_game_over_camera(mut commands: Commands) {
    commands.spawn((Camera2d, GameOverCamera));
}

fn despawn_game_over_camera(mut commands: Commands, query: Query<Entity, With<GameOverCamera>>) {
    for camera_entity in query.iter() {
        commands.entity(camera_entity).despawn();
    }
}

fn display_game_over_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let box_size = Vec2::new(300.0, 200.0);
    let box_position = Vec2::new(0.0, -250.0);
    // Demonstrate text wrapping
    let slightly_smaller_text_font = TextFont {
        font,
        font_size: 35.0,
        ..default()
    };
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.25, 0.25, 0.75), box_size),
            Transform::from_translation(box_position.extend(0.0)),
            GameOverText,
        ))
        .with_children(|builder| {
            builder.spawn((
                //Text2d::new("Game Over - Press <Enter> to play again"),
                Text2d::new("Game Over !"),
                slightly_smaller_text_font.clone(),
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                // Wrap text in the rectangle
                TextBounds::from(box_size),
                // ensure the text is drawn on top of the box
                Transform::from_translation(Vec3::Z),
                GameOverText,
            ));
        });
}

fn remove_game_over_text(mut commands: Commands, query: Query<Entity, With<GameOverText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn transition_to_game_restart(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::GameRestart);
    }
}
