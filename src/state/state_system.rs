use bevy::prelude::*;

use super::state_component::{GameState, Transition};

pub fn transition_out_system(
    mut next_state: ResMut<NextState<GameState>>,
    state: ResMut<State<GameState>>,
    mut transition: ResMut<Transition>,
    time: Res<Time>,
) {
    if *state.get() == GameState::TransitioningOut {
        if transition.progress == 0.0 {
            // Capture the current screen

            // Store the captured image in the transition resource

            // disable the camera
        }

        // Increment the transition progress
        transition.progress += time.delta_secs();
        if transition.progress >= 1.0 {
            transition.progress = 0.0;
            next_state.set(GameState::TransitioningIn);
        }

        if transition.progress < 1.0 {
            // fade out from the captured_image
        }
    }
}

pub fn transition_in_system(
    //mut next_state: ResMut<NextState<GameState>>,
    state: ResMut<State<GameState>>,
    // mut transition: ResMut<Transition>,
    // time: Res<Time>,
    // mut commands: Commands,
    // camera_query: Query<Entity, With<Camera2d>>,
) {
    if *state.get() == GameState::TransitioningIn {
        // restore camera
        // set next_state to GameState::InGame
        // set transition to 0.0 and None for captured_image
    }
}
