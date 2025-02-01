use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;

use crate::floorplan::FloorPlanEvent;

use super::environ_component::{
    CurrentFloorPlan, EnvironState, Ground, LeftWall, RightWall, TopBoundary, WINDOW_HEIGHT,
};

pub fn handle_floor_plan_changes(
    mut floorplan_events: EventReader<FloorPlanEvent>,
    mut current_floorplan: ResMut<CurrentFloorPlan>,
    //mut _events: EventWriter<PlatformEvent>,
) {
    for event in floorplan_events.read() {
        debug!("Floor plan event received.");

        let new_floorplan = event.floorplan.clone();

        if let Some(current_plan) = &current_floorplan.floorplan {
            if *current_plan != new_floorplan {
                warn!("Floor plan has changed.");

                // Calculate the differences and fire other events
                // For example, you can spawn new platforms based on the differences
                // commands.spawn(...);
            }
        } else {
            info!("Initial floor plan set.");
        }

        // Update the current floor plan
        current_floorplan.floorplan = Some(new_floorplan);
    }
}

#[allow(clippy::type_complexity)]
pub fn update_environment(
    environ_state: Res<EnvironState>,
    mut param_set: ParamSet<(
        Query<(&mut Transform, &mut Collider), With<Ground>>,
        Query<(&mut Transform, &mut Collider, &mut Sprite), With<LeftWall>>,
        Query<(&mut Transform, &mut Collider, &mut Sprite), With<RightWall>>,
        Query<(&mut Transform, &mut Collider), With<TopBoundary>>,
    )>,
) {
    if !environ_state.is_changed() {
        return;
    }
    info!("Updating changing environment...");

    // Update ground
    for (mut transform, mut collider) in &mut param_set.p0() {
        *collider = Collider::from(SharedShape::cuboid(
            environ_state.floor_ceiling_width / 2.0,
            environ_state.boundary_thickness,
        ));
        *transform = Transform::from_xyz(
            0.0,
            -WINDOW_HEIGHT / 2.0 + environ_state.boundary_thickness,
            0.0,
        );
    }

    // Update left wall
    for (mut transform, mut collider, mut sprite) in &mut param_set.p1() {
        *collider = Collider::from(SharedShape::cuboid(
            environ_state.boundary_thickness,
            WINDOW_HEIGHT / 2.0,
        ));
        *transform = Transform::from_xyz(-environ_state.wall_distance_from_center, 0.0, 0.0);
        sprite.custom_size = Some(Vec2::new(
            environ_state.boundary_thickness * 200.0,
            WINDOW_HEIGHT,
        ));
    }

    // Update right wall
    for (mut transform, mut collider, mut sprite) in &mut param_set.p2() {
        *collider = Collider::from(SharedShape::cuboid(
            environ_state.boundary_thickness,
            WINDOW_HEIGHT / 2.0,
        ));
        *transform = Transform::from_xyz(environ_state.wall_distance_from_center, 0.0, 0.0);
        sprite.custom_size = Some(Vec2::new(
            environ_state.boundary_thickness * 200.0,
            WINDOW_HEIGHT,
        ));
    }

    // Update top boundary
    for (mut transform, mut collider) in &mut param_set.p3() {
        *collider = Collider::from(SharedShape::cuboid(
            environ_state.floor_ceiling_width / 2.0,
            environ_state.boundary_thickness,
        ));
        *transform = Transform::from_xyz(
            0.0,
            WINDOW_HEIGHT / 2.0 - environ_state.boundary_thickness,
            0.0,
        );
    }
}

pub fn setup_environment(mut commands: Commands, environ_state: ResMut<EnvironState>) {
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            environ_state.floor_ceiling_width / 2.0,
            environ_state.boundary_thickness,
        )),
        Transform::from_xyz(
            0.0,
            -WINDOW_HEIGHT / 2.0 + environ_state.boundary_thickness,
            0.0,
        ),
        Friction {
            dynamic_coefficient: 0.8,
            static_coefficient: 0.9,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: environ_state.bounce_effect % 2.0,
            combine_rule: CoefficientCombine::Max,
        },
        Ground,
    ));

    // Left wall
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            environ_state.boundary_thickness,
            WINDOW_HEIGHT / 2.0,
        )),
        Transform::from_xyz(-environ_state.wall_distance_from_center, 0.0, 0.0),
        Friction {
            dynamic_coefficient: 0.5,
            static_coefficient: 0.6,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: environ_state.bounce_effect,
            combine_rule: CoefficientCombine::Max,
        },
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5), // Matching the platform color
            custom_size: Some(Vec2::new(
                environ_state.boundary_thickness * 200.0,
                WINDOW_HEIGHT,
            )),
            ..default()
        },
        LeftWall,
    ));

    // Right wall
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            environ_state.boundary_thickness,
            WINDOW_HEIGHT / 2.0,
        )),
        Transform::from_xyz(environ_state.wall_distance_from_center, 0.0, 0.0),
        Friction {
            dynamic_coefficient: 0.5,
            static_coefficient: 0.6,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: environ_state.bounce_effect,
            combine_rule: CoefficientCombine::Max,
        },
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5), // Matching the platform color
            custom_size: Some(Vec2::new(
                environ_state.boundary_thickness * 200.0,
                WINDOW_HEIGHT,
            )),
            ..default()
        },
        RightWall,
    ));

    // Top boundary
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            environ_state.floor_ceiling_width / 2.0,
            environ_state.boundary_thickness,
        )),
        Transform::from_xyz(
            0.0,
            WINDOW_HEIGHT / 2.0 - environ_state.boundary_thickness,
            0.0,
        ),
        Restitution {
            coefficient: environ_state.bounce_effect,
            combine_rule: CoefficientCombine::Max,
        },
        TopBoundary,
    ));
}
