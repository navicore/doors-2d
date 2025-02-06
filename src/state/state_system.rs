use bevy::prelude::*;

use crate::room::{room_component::WINDOW_WIDTH, WINDOW_HEIGHT};

use super::{
    state_component::{FadeEffect, FadeOverlay},
    GameState,
};

fn setup_fade_overlay(
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera2d>>, // Find the camera entity
) {
    let camera_entity = camera_query.single();

    // Spawn a fullscreen black sprite for fade effect
    let fade_overlay = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.0, 0.0, 0.0, 0.0), // Fully transparent initially
                    custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)), // Covers screen
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 1.0), // Slightly above to cover everything
                ..default()
            },
            FadeOverlay, // Tag it to query later
        ))
        .id();

    // Attach the fade overlay to the camera so it moves together
    commands.entity(camera_entity).add_child(fade_overlay);

    println!("Fade Overlay Setup Complete.");
}

pub fn fade_out(
    mut next_state: ResMut<NextState<GameState>>,
    mut fade: ResMut<FadeEffect>,
    mut fade_query: Query<&mut Sprite, With<FadeOverlay>>,
    time: Res<Time>,
) {
    let mut sprite = fade_query.single_mut();

    if fade.fading_out {
        fade.alpha += time.delta_secs() * 1.5; // Slow fade-out
        fade.alpha = fade.alpha.min(1.0); // Clamp at full opacity
        sprite.color.set_alpha(fade.alpha);

        println!("Fading Out: Alpha = {}", fade.alpha);

        if fade.alpha >= 1.0 {
            println!("Finished Fading Out, starting Fade In...");
            fade.fading_out = false; // Switch to fading in
                                     //next_state.set(GameState::RoomChange);
            next_state.set(GameState::TransitioningIn);
        }
    }
}

pub fn fade_in(
    mut next_state: ResMut<NextState<GameState>>,
    mut fade: ResMut<FadeEffect>,
    mut fade_query: Query<&mut Sprite, With<FadeOverlay>>,
    time: Res<Time>,
) {
    let mut sprite = fade_query.single_mut();

    if !fade.fading_out {
        fade.alpha -= time.delta_secs() * 1.5; // Slow fade-in
        fade.alpha = fade.alpha.max(0.0); // Clamp at full transparency
        sprite.color.set_alpha(fade.alpha);

        println!("Fading In: Alpha = {}", fade.alpha);

        if fade.alpha <= 0.0 {
            println!("Finished Fading In, ready for next transition.");
            next_state.set(GameState::InGame); // Transition complete
        }
    }
}
