use bevy::prelude::*;

use super::{
    state_component::GameState,
    state_system::{display_paused_text, game_state_input_events, pause_game, remove_pause_text},
};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, (pause_game, game_state_input_events))
            .add_systems(OnEnter(GameState::Paused), display_paused_text)
            .add_systems(OnExit(GameState::Paused), remove_pause_text);
    }
}
