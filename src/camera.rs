use bevy::prelude::*;

use crate::schedule::InGameSet;

const CAMERA_MOVE_SPEED: f32 = 10.0; // Speed at which the camera moves
const SCREEN_HALF_WIDTH: f32 = 600.0; // Half of window width (assuming 1200x800 resolution)
const SCROLL_THRESHOLD: f32 = 400.0; // Distance from the screen edge before scrolling

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, follow_player.in_set(InGameSet::EntityUpdates));
    }
}

// Marker component for tracking the camera
#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d, MainCamera, // Mark the camera for easy querying
    ));
}

type PlayerQuery<'a> = Query<'a, 'a, &'a Transform, With<crate::player::Player>>;
type CameraQuery<'a> = Query<'a, 'a, &'a mut Transform, With<MainCamera>>;

fn follow_player(mut query_set: ParamSet<(PlayerQuery<'_>, CameraQuery<'_>)>) {
    if let Ok(player_transform) = query_set.p0().get_single() {
        let player_x = player_transform.translation.x;

        if let Ok(mut camera_transform) = query_set.p1().get_single_mut() {
            let camera_x = camera_transform.translation.x;

            // If player is near the right edge of the screen
            if player_x > camera_x + SCREEN_HALF_WIDTH - SCROLL_THRESHOLD {
                camera_transform.translation.x += CAMERA_MOVE_SPEED;
            }
            // If player is near the left edge of the screen
            if player_x < camera_x - SCREEN_HALF_WIDTH + SCROLL_THRESHOLD {
                camera_transform.translation.x -= CAMERA_MOVE_SPEED;
            }
        }
    }
}
