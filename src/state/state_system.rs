use bevy::prelude::*;

use super::state_component::{GameState, Transition};

pub fn transition_out_system(
    mut next_state: ResMut<NextState<GameState>>,
    state: ResMut<State<GameState>>,
    mut transition: ResMut<Transition>,
    time: Res<Time>,
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera2d>>,
) {
    if *state.get() == GameState::TransitioningOut {
        if transition.progress == 0.0 {
            // Capture the current screen
            // Assuming you have a method to capture the screen and store it in transition
            transition.capture_screen(&camera_query);

            // Store the captured image in the transition resource
            // This is assumed to be handled by the capture_screen method

            // Disable the camera
            for camera in camera_query.iter() {
                commands.entity(camera).despawn();
            }
        }

        // Increment the transition progress
        transition.progress += time.delta_secs();
        if transition.progress >= 1.0 {
            transition.progress = 0.0;
            next_state.set(GameState::TransitioningIn);
        }

        if transition.progress < 1.0 {
            // Fade out from the captured_image
            transition.fade_out();
        }
    }
}

pub fn transition_in_system(
    mut next_state: ResMut<NextState<GameState>>,
    state: ResMut<State<GameState>>,
    mut transition: ResMut<Transition>,
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera2d>>,
) {
    if *state.get() == GameState::TransitioningIn {
        // Restore camera
        for camera in camera_query.iter() {
            commands.entity(camera).insert(Camera2d);
        }

        // Set next_state to GameState::InGame
        next_state.set(GameState::InGame);

        // Set transition to 0.0 and None for captured_image
        transition.progress = 0.0;
        transition.clear_captured_image();
    }
}
