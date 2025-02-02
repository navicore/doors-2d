use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;

use crate::room::room_component::RoomState;

use super::platform_component::{Door, Platform, BOUNCE_EFFECT, PLATFORM_HEIGHT, PLATFORM_WIDTH};

pub fn spawn_platforms(
    mut commands: Commands,
    room_state: Res<RoomState>,
    query: Query<Entity, With<Platform>>,
) {
    if !room_state.is_changed() {
        return;
    }

    info!("Room state changed, respawning platforms...");

    // Despawn all existing platforms
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn new platforms based on the current room state
    for (position, id) in room_state
        .clone()
        .doors
        .into_iter()
        .map(|door_state| (door_state.position, door_state.id))
    {
        // Spawn the platform
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
            Platform { id },
            Sprite {
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
                ..default()
            },
        ));

        // Spawn the door on top of the platform
        commands.spawn((
            Door,
            Transform::from_xyz(
                position.x,
                position.y + PLATFORM_HEIGHT / 2.0 + PLATFORM_WIDTH / 4.0,
                0.1,
            ), // Adjust the position to sit on the platform
            Sprite {
                color: Color::srgb(0.3, 0.3, 0.3),
                custom_size: Some(Vec2::new(PLATFORM_WIDTH / 4.0, PLATFORM_WIDTH / 2.0)), // Twice as tall as wide
                ..default()
            },
        ));
    }
}
