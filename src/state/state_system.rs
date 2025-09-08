use super::{
    state_component::{FadeEffect, FadeOverlay},
    GameState,
};
use crate::{
    constants::{
        CURTAIN_DURATION, FADE_IN_DURATION, FADE_OUT_DURATION, FADE_OVERLAY_Z, WINDOW_HEIGHT,
    },
    room::room_component::{CurrentFloorPlan, RoomState},
};
use bevy::prelude::*;

pub fn setup_fade_overlay(mut commands: Commands, room_state: Res<RoomState>) {
    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.0), // Fully transparent initially
            custom_size: Some(Vec2::new(room_state.floor_ceiling_width, WINDOW_HEIGHT)), // Cover screen
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, FADE_OVERLAY_Z), // Render on top with a high z value
        FadeOverlay,                                   // Add this tag so our query finds it
    ));

    commands.insert_resource(FadeEffect {
        curtain: 0.0,
        alpha: 0.0,
        fading_out: false,
    });
}

pub fn update_fade_overlay(
    room_state: Res<RoomState>,
    mut query: Query<(&mut Sprite, &mut Transform), With<FadeOverlay>>,
) {
    if !room_state.is_changed() {
        return;
    }
    for (mut sprite, mut transform) in &mut query {
        sprite.custom_size = Some(Vec2::new(room_state.floor_ceiling_width, WINDOW_HEIGHT));
        transform.translation.z = FADE_OVERLAY_Z;
    }
}

pub fn fade_out(
    mut next_state: ResMut<NextState<GameState>>,
    mut fade: ResMut<FadeEffect>,
    mut fade_query: Query<&mut Sprite, With<FadeOverlay>>,
    time: Res<Time>,
    current_floor_plan: Res<CurrentFloorPlan>,
) {
    // Check if we're at the start room - don't do fade if we are
    if let Some(plan) = &current_floor_plan.floorplan
        && let Ok(start_room) = plan.get_start_room()
        && current_floor_plan.you_are_here == Some(start_room.id.clone())
    {
        fade.fading_out = false; // Switch to fading in
        next_state.set(GameState::RoomChange);
        debug!("Skipping fade out for start room");
        return;
    }

    let mut sprite = fade_query.single_mut();

    if fade.fading_out {
        fade.alpha += time.delta_secs() / FADE_OUT_DURATION; // Slow fade-out
        fade.alpha = fade.alpha.min(1.0); // Clamp at full opacity
        sprite.color.set_alpha(fade.alpha);

        if fade.alpha >= 1.0 {
            debug!("Fade out complete");
            fade.fading_out = false; // Switch to fading in
                                     //next_state.set(GameState::RoomChange);
            next_state.set(GameState::RoomChange);
        }
    }
}

pub fn room_change_curtain(
    mut next_state: ResMut<NextState<GameState>>,
    mut fade: ResMut<FadeEffect>,
    time: Res<Time>,
) {
    fade.curtain += time.delta_secs() * CURTAIN_DURATION; // Slow fade-out
    fade.curtain = fade.curtain.min(1.0); // Clamp at full opacity

    if fade.curtain >= 1.0 {
        debug!("Curtain complete");
        next_state.set(GameState::TransitioningIn);
        fade.curtain = 0.0; // Reset for next time
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
        fade.alpha -= time.delta_secs() / FADE_IN_DURATION; // Slow fade-in
        fade.alpha = fade.alpha.max(0.0); // Clamp at full transparency
        sprite.color.set_alpha(fade.alpha);

        if fade.alpha <= 0.0 {
            debug!("Fade in complete");
            next_state.set(GameState::InGame); // Transition complete
        }
    }
}
