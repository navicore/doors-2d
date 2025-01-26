use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;

// Define window size and environment constants
pub const WINDOW_WIDTH: f32 = 1200.0;
pub const WINDOW_HEIGHT: f32 = 800.0;
const BOUNDARY_THICKNESS: f32 = 0.1;
const BOUNCE_EFFECT: f32 = 0.4;

const WALL_DISTANCE_FROM_CENTER: f32 = 1500.0;
const FLOOR_CEILING_WIDTH: f32 = WALL_DISTANCE_FROM_CENTER * 2.0;

#[derive(Component)]
pub struct Ground;

/// define the game window size and environment constants and create the left and right walls, the
/// ground, and the top boundary.
pub struct EnvironPlugin;

impl Plugin for EnvironPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Kubernetes Platformer".to_string(),
                resolution: bevy::window::WindowResolution::from((WINDOW_WIDTH, WINDOW_HEIGHT)),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_environment);
    }
}

fn setup_environment(mut commands: Commands) {
    // Spawn the ground with physics and centralized boundary thickness
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            FLOOR_CEILING_WIDTH / 2.0,
            BOUNDARY_THICKNESS,
        )),
        Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + BOUNDARY_THICKNESS, 0.0),
        Friction {
            dynamic_coefficient: 0.8,
            static_coefficient: 0.9,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: BOUNCE_EFFECT % 2.0,
            combine_rule: CoefficientCombine::Max,
        },
        Ground,
    ));

    // Left wall
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(BOUNDARY_THICKNESS, WINDOW_HEIGHT / 2.0)),
        Transform::from_xyz(-WALL_DISTANCE_FROM_CENTER, 0.0, 0.0),
        Friction {
            dynamic_coefficient: 0.5,
            static_coefficient: 0.6,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: BOUNCE_EFFECT,
            combine_rule: CoefficientCombine::Max,
        },
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5), // Matching the platform color
            custom_size: Some(Vec2::new(BOUNDARY_THICKNESS * 200.0, WINDOW_HEIGHT)),
            ..default()
        },
    ));

    // Right wall
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(BOUNDARY_THICKNESS, WINDOW_HEIGHT / 2.0)),
        Transform::from_xyz(WALL_DISTANCE_FROM_CENTER, 0.0, 0.0),
        Friction {
            dynamic_coefficient: 0.5,
            static_coefficient: 0.6,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: BOUNCE_EFFECT,
            combine_rule: CoefficientCombine::Max,
        },
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5), // Matching the platform color
            custom_size: Some(Vec2::new(BOUNDARY_THICKNESS * 200.0, WINDOW_HEIGHT)),
            ..default()
        },
    ));
    // Top boundary
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            FLOOR_CEILING_WIDTH / 2.0,
            BOUNDARY_THICKNESS,
        )),
        Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - BOUNDARY_THICKNESS, 0.0),
        Restitution {
            coefficient: BOUNCE_EFFECT,
            combine_rule: CoefficientCombine::Max,
        },
    ));
}
