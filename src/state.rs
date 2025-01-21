use bevy::{prelude::*, text::TextBounds};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
}

#[derive(Debug, Component)]
struct PausedText;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, (pause_game, game_state_input_events))
            .add_systems(OnEnter(GameState::Paused), display_paused_text)
            .add_systems(OnExit(GameState::Paused), remove_pause_text);
    }
}

fn remove_pause_text(mut commands: Commands, query: Query<Entity, With<PausedText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
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
        }
    } else if keyboard_input.just_pressed(KeyCode::KeyQ) {
        // exit the game
        std::process::exit(0);
    }
}

fn pause_game(mut time: ResMut<Time<Virtual>>, state: Res<State<GameState>>) {
    if *state == GameState::Paused {
        time.set_relative_speed(0.0); // Freeze physics and animation
    } else {
        time.set_relative_speed(1.0); // Resume physics
    }
}

fn display_paused_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let box_size = Vec2::new(200.0, 75.0);
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
            PausedText,
        ))
        .with_children(|builder| {
            builder.spawn((
                //Text2d::new("Game Over - Press <Enter> to play again"),
                Text2d::new("Paused"),
                slightly_smaller_text_font.clone(),
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                // Wrap text in the rectangle
                TextBounds::from(box_size),
                // ensure the text is drawn on top of the box
                Transform::from_translation(Vec3::Z),
                PausedText,
            ));
        });
}
