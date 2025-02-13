use bevy::prelude::*;

// Marker component for tracking the camera
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct MovingLights;

pub const X_EXTENT: f32 = 700.;
