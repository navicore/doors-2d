use crate::camera::camera_component::MainCamera;
use bevy::{
    color::palettes::tailwind::{BLUE_300, BLUE_600},
    prelude::*,
};
use bevy_lit::prelude::{AmbientLight2d, Lighting2dSettings, PointLight2d, RaymarchSettings};

use super::camera_component::{MovingLights, X_EXTENT};

const CAMERA_MOVE_SPEED: f32 = 10.0; // Speed at which the camera moves
const SCREEN_HALF_WIDTH: f32 = 600.0; // Half of window width (assuming 1200x800 resolution)
const SCROLL_THRESHOLD: f32 = 400.0; // Distance from the screen edge before scrolling

type PlayerQuery<'a> = Query<'a, 'a, &'a Transform, With<crate::player::Player>>;
type CameraQuery<'a> = Query<'a, 'a, &'a mut Transform, With<MainCamera>>;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        MainCamera, // Mark the camera for easy querying
        Lighting2dSettings {
            blur: 32.,
            raymarch: RaymarchSettings {
                max_steps: 64,
                jitter_contrib: 0.5,
                sharpness: 10.,
            },
            ..default()
        },
        AmbientLight2d {
            brightness: 0.5,
            color: Color::from(BLUE_300),
        },
    ));

    commands
        .spawn((MovingLights, Transform::default(), Visibility::default()))
        .with_children(|builder| {
            let point_light = PointLight2d {
                intensity: 3.0,
                radius: 1100.0,
                falloff: 3.0,
                ..default()
            };

            builder.spawn((
                PointLight2d {
                    color: Color::from(BLUE_600),
                    ..point_light
                },
                Transform::from_xyz(-X_EXTENT + 50. / 2., 0.0, 0.0),
            ));

            builder.spawn((
                PointLight2d {
                    color: Color::from(BLUE_600),
                    ..point_light
                },
                Transform::from_xyz(X_EXTENT + 50. / 2., 0.0, 0.0),
            ));
        });
}

pub fn follow_player(mut query_set: ParamSet<(PlayerQuery<'_>, CameraQuery<'_>)>) {
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

pub fn update_moving_lights(
    time: Res<Time>,
    mut point_light_query: Query<&mut Transform, With<MovingLights>>,
) {
    for mut transform in &mut point_light_query {
        transform.rotation *= Quat::from_rotation_z(time.delta_secs() / 12.0);
    }
}
