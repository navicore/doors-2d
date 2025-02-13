use crate::camera::camera_systems::{follow_player, spawn_camera};
use bevy::{color::palettes::tailwind::GRAY_200, prelude::*};
use bevy_lit::prelude::Lighting2dPlugin;

/// a 2D camera that follows the player perpendicularly
pub struct CameraPlugin;

#[derive(Component)]
struct MovingLights;

const X_EXTENT: f32 = 700.;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Lighting2dPlugin)
            .insert_resource(ClearColor(Color::from(GRAY_200)))
            .add_systems(Startup, spawn_camera)
            .add_systems(FixedUpdate, update_moving_lights)
            .add_systems(Update, follow_player);
    }
}

fn update_moving_lights(
    time: Res<Time>,
    mut point_light_query: Query<&mut Transform, With<MovingLights>>,
) {
    for mut transform in &mut point_light_query {
        transform.rotation *= Quat::from_rotation_z(time.delta_secs() / 12.0);
    }
}
