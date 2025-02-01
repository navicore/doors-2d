use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;

use crate::floorplan::FloorPlanEvent;

use super::environ_component::{CurrentFloorPlan, Ground};

// Define window size and environment constants
pub const WINDOW_WIDTH: f32 = 1200.0;
pub const WINDOW_HEIGHT: f32 = 800.0;
pub const BOUNDARY_THICKNESS: f32 = 0.1;
pub const BOUNCE_EFFECT: f32 = 0.4;

pub const WALL_DISTANCE_FROM_CENTER: f32 = 1500.0;
pub const FLOOR_CEILING_WIDTH: f32 = WALL_DISTANCE_FROM_CENTER * 2.0;

pub fn handle_floor_plan_changes(
    mut floorplan_events: EventReader<FloorPlanEvent>,
    mut current_floorplan: ResMut<CurrentFloorPlan>,
    //mut _events: EventWriter<PlatformEvent>,
) {
    for event in floorplan_events.read() {
        debug!("Floor plan event received!");

        let new_floorplan = event.floorplan.clone();

        if let Some(current_plan) = &current_floorplan.floorplan {
            if *current_plan != new_floorplan {
                debug!("Floor plan has changed!");

                // Calculate the differences and fire other events
                // For example, you can spawn new platforms based on the differences
                // commands.spawn(...);
            }
        }

        // Update the current floor plan
        current_floorplan.floorplan = Some(new_floorplan);
    }
}

pub fn setup_environment(mut commands: Commands) {
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
