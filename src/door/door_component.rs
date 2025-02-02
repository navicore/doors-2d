use bevy::prelude::Component;

pub const PLATFORM_WIDTH: f32 = 200.0;
pub const PLATFORM_HEIGHT: f32 = 20.0;
pub const BOUNCE_EFFECT: f32 = 0.1;

#[derive(Component)]
pub struct Platform {}

#[derive(Component)]
pub struct Door {
    pub room_id: String,
    pub room_name: String,
}
