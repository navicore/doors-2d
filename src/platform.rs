use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;

// Constants for platform placement
const PLATFORM_WIDTH: f32 = 200.0;
const PLATFORM_HEIGHT: f32 = 20.0;
const PLATFORM_Y_POS: f32 = -100.0; // 50% above the floor assuming a floor at -WINDOW_HEIGHT / 2.0
const BOUNCE_EFFECT: f32 = 0.1;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_platforms);
    }
}

// Component to identify platforms
#[derive(Component)]
pub struct Platform;

fn spawn_platforms(mut commands: Commands) {
    // Hardcoded platform data representing possible dynamic inputs
    let platform_positions = vec![
        Vec2::new(-300.0, PLATFORM_Y_POS),
        Vec2::new(150.0, PLATFORM_Y_POS + 100.0),
        Vec2::new(400.0, PLATFORM_Y_POS - 50.0),
    ];

    for position in platform_positions {
        commands.spawn((
            RigidBody::Static,
            Collider::from(SharedShape::cuboid(
                PLATFORM_WIDTH / 2.0,
                PLATFORM_HEIGHT / 2.0,
            )),
            Transform::from_xyz(position.x, position.y, 0.0),
            Friction {
                dynamic_coefficient: 0.6,
                static_coefficient: 0.8,
                combine_rule: CoefficientCombine::Average,
            },
            Restitution {
                coefficient: BOUNCE_EFFECT,
                combine_rule: CoefficientCombine::Max,
            },
            Platform, // Tag for platform objects
            Sprite {
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
                ..default()
            },
        ));
    }
}
