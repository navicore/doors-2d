use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;

use crate::{floorplan::FloorPlanEvent, room::room_component::DoorState};

use super::room_component::{
    Ceiling, CurrentFloorPlan, Floor, LeftWall, RightWall, RoomState, WINDOW_HEIGHT,
};

const PLATFORM_X_SEPARATOR: f32 = 450.0;
const PLATFORM_Y_SEPARATOR: &[f32] = &[
    0.0, -150.0, 250.0, -100.0, -210.0, 100.0, -200.0, -50.0, 175.0, 25.0,
];

pub fn handle_floor_plan_changes(
    mut floorplan_events: EventReader<FloorPlanEvent>,
    mut current_floorplan: ResMut<CurrentFloorPlan>,
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
        let you_are_here = current_floorplan.you_are_here.as_ref().map_or_else(
            || {
                new_floorplan.get_start_room().map_or_else(
                    |_| {
                        warn!("No start room found in the floor plan.");
                        None
                    },
                    |room| Some(room.id.clone()),
                )
            },
            |location| Some(location.clone()),
        );

        // Update the current floor plan
        *current_floorplan = CurrentFloorPlan {
            floorplan: Some(new_floorplan),
            you_are_here,
        };
    }
}

pub fn update_doors(current_floorplan: Res<CurrentFloorPlan>, mut room_state: ResMut<RoomState>) {
    if !current_floorplan.is_changed() {
        return;
    }
    if let Some(floorplan) = current_floorplan.floorplan.as_ref() {
        if let Some(room_id) = &current_floorplan.you_are_here {
            info!("Updating doors...");
            match floorplan.get_doors_and_connected_rooms(room_id) {
                Ok(doors_and_rooms) => {
                    // calculate door placement and room size
                    let number_of_doors = doors_and_rooms.len();
                    #[allow(clippy::cast_precision_loss)]
                    let room_width: f32 = PLATFORM_X_SEPARATOR * (number_of_doors + 1) as f32; // empty each side of the room
                    room_state.wall_distance_from_center = room_width / 2.0;
                    room_state.floor_ceiling_width = room_width;

                    let mut room_seq = 0;
                    #[allow(clippy::cast_precision_loss)]
                    let mut room_positions: Vec<Vec2> = (0..number_of_doors)
                        .map(|i| {
                            let y_index = if room_seq < PLATFORM_Y_SEPARATOR.len() {
                                room_seq
                            } else {
                                room_seq % PLATFORM_Y_SEPARATOR.len()
                            };
                            room_seq += 1;

                            Vec2 {
                                x: PLATFORM_X_SEPARATOR.mul_add(i as f32, PLATFORM_X_SEPARATOR)
                                    - room_state.wall_distance_from_center,
                                y: PLATFORM_Y_SEPARATOR[y_index],
                            }
                        })
                        .collect();

                    for (door, _) in doors_and_rooms {
                        if let Some(position) = room_positions.pop() {
                            let door_state = DoorState {
                                id: door.id.clone(),
                                position,
                            };
                            info!("Updating door...");
                            room_state.doors.push(door_state);
                        }
                    }
                }
                _ => panic!("Failed to get doors and connected rooms"),
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn update_room(
    room_state: Res<RoomState>,
    mut param_set: ParamSet<(
        Query<(&mut Transform, &mut Collider), With<Floor>>,
        Query<(&mut Transform, &mut Collider, &mut Sprite), With<LeftWall>>,
        Query<(&mut Transform, &mut Collider, &mut Sprite), With<RightWall>>,
        Query<(&mut Transform, &mut Collider), With<Ceiling>>,
    )>,
) {
    if !room_state.is_changed() {
        return;
    }
    debug!("Updating room...");

    // Update ground
    for (mut transform, mut collider) in &mut param_set.p0() {
        *collider = Collider::from(SharedShape::cuboid(
            room_state.floor_ceiling_width / 2.0,
            room_state.boundary_thickness,
        ));
        *transform = Transform::from_xyz(
            0.0,
            -WINDOW_HEIGHT / 2.0 + room_state.boundary_thickness,
            0.0,
        );
    }

    // Update left wall
    for (mut transform, mut collider, mut sprite) in &mut param_set.p1() {
        *collider = Collider::from(SharedShape::cuboid(
            room_state.boundary_thickness,
            WINDOW_HEIGHT / 2.0,
        ));
        *transform = Transform::from_xyz(-room_state.wall_distance_from_center, 0.0, 0.0);
        sprite.custom_size = Some(Vec2::new(
            room_state.boundary_thickness * 200.0,
            WINDOW_HEIGHT,
        ));
    }

    // Update right wall
    for (mut transform, mut collider, mut sprite) in &mut param_set.p2() {
        *collider = Collider::from(SharedShape::cuboid(
            room_state.boundary_thickness,
            WINDOW_HEIGHT / 2.0,
        ));
        *transform = Transform::from_xyz(room_state.wall_distance_from_center, 0.0, 0.0);
        sprite.custom_size = Some(Vec2::new(
            room_state.boundary_thickness * 200.0,
            WINDOW_HEIGHT,
        ));
    }

    // Update top boundary
    for (mut transform, mut collider) in &mut param_set.p3() {
        *collider = Collider::from(SharedShape::cuboid(
            room_state.floor_ceiling_width / 2.0,
            room_state.boundary_thickness,
        ));
        *transform = Transform::from_xyz(
            0.0,
            WINDOW_HEIGHT / 2.0 - room_state.boundary_thickness,
            0.0,
        );
    }
}

// todo: question what is the purpose of setting up a room before there is a floorplan
pub fn setup_room(
    mut commands: Commands,
    room_state: ResMut<RoomState>,
    current_floorplan: Res<CurrentFloorPlan>,
) {
    if current_floorplan.you_are_here.is_some() {
        // room is already setup
        return;
    }
    info!("Setting up room...");

    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            room_state.floor_ceiling_width,
            room_state.boundary_thickness,
        )),
        Transform::from_xyz(
            0.0,
            -WINDOW_HEIGHT / 2.0 + room_state.boundary_thickness,
            0.0,
        ),
        Friction {
            dynamic_coefficient: 0.8,
            static_coefficient: 0.9,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: room_state.bounce_effect % 2.0,
            combine_rule: CoefficientCombine::Max,
        },
        Floor,
    ));

    // Left wall
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            room_state.boundary_thickness,
            WINDOW_HEIGHT / 2.0,
        )),
        Transform::from_xyz(-room_state.wall_distance_from_center, 0.0, 0.0),
        Friction {
            dynamic_coefficient: 0.5,
            static_coefficient: 0.6,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: room_state.bounce_effect,
            combine_rule: CoefficientCombine::Max,
        },
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5), // Matching the platform color
            custom_size: Some(Vec2::new(
                room_state.boundary_thickness * 200.0,
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
            room_state.boundary_thickness,
            WINDOW_HEIGHT / 2.0,
        )),
        Transform::from_xyz(room_state.wall_distance_from_center, 0.0, 0.0),
        Friction {
            dynamic_coefficient: 0.5,
            static_coefficient: 0.6,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: room_state.bounce_effect,
            combine_rule: CoefficientCombine::Max,
        },
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5), // Matching the platform color
            custom_size: Some(Vec2::new(
                room_state.boundary_thickness * 200.0,
                WINDOW_HEIGHT,
            )),
            ..default()
        },
        RightWall,
    ));

    // Ceiling
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            room_state.floor_ceiling_width,
            room_state.boundary_thickness,
        )),
        Transform::from_xyz(
            0.0,
            WINDOW_HEIGHT / 2.0 - room_state.boundary_thickness,
            0.0,
        ),
        Restitution {
            coefficient: room_state.bounce_effect,
            combine_rule: CoefficientCombine::Max,
        },
        Ceiling,
    ));
}
