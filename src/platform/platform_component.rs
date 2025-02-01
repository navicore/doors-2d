use bevy::prelude::Component;

// Constants for platform placement
pub const PLATFORM_WIDTH: f32 = 200.0;
pub const PLATFORM_HEIGHT: f32 = 20.0;
pub const PLATFORM_Y_POS: f32 = -100.0; // 50% above the floor assuming a floor at -WINDOW_HEIGHT / 2.0
pub const BOUNCE_EFFECT: f32 = 0.1;

// Component to identify platforms
#[derive(Component)]
pub struct Platform;

#[derive(Component)]
pub struct Door;

// #[derive(Event)]
// pub struct PlatformEvent {
//     pub door: Door,
// }
