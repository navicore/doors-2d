use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;

// Define window size and environment constants
const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;
const BOUNDARY_THICKNESS: f32 = 0.1;
const BOUNCE_EFFECT: f32 = 0.4;

pub struct EnvironPlugin;

impl Plugin for EnvironPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_environment);
    }
}

fn setup_environment(mut commands: Commands) {
    // Spawn the ground with physics and centralized boundary thickness
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(WINDOW_WIDTH / 2.0, BOUNDARY_THICKNESS)),
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
    ));

    // Left wall
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(BOUNDARY_THICKNESS, WINDOW_HEIGHT / 2.0)),
        Transform::from_xyz(-WINDOW_WIDTH / 2.0 + BOUNDARY_THICKNESS, 0.0, 0.0),
        Friction {
            dynamic_coefficient: 0.5,
            static_coefficient: 0.6,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: BOUNCE_EFFECT,
            combine_rule: CoefficientCombine::Max,
        },
    ));

    // Right wall
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(BOUNDARY_THICKNESS, WINDOW_HEIGHT / 2.0)),
        Transform::from_xyz(WINDOW_WIDTH / 2.0 - BOUNDARY_THICKNESS, 0.0, 0.0),
        Friction {
            dynamic_coefficient: 0.5,
            static_coefficient: 0.6,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: BOUNCE_EFFECT,
            combine_rule: CoefficientCombine::Max,
        },
    ));

    // Top boundary
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(WINDOW_WIDTH / 2.0, BOUNDARY_THICKNESS)),
        Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - BOUNDARY_THICKNESS, 0.0),
        Restitution {
            coefficient: BOUNCE_EFFECT,
            combine_rule: CoefficientCombine::Max,
        },
    ));
}
