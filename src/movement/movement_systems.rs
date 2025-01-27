use super::movement_component::{Grounded, Movable};
use crate::environ::Ground;
use crate::platform::Platform;
use avian2d::prelude::*;
use bevy::prelude::*;

#[allow(clippy::type_complexity)]
pub fn check_grounded(
    mut collision_events: EventReader<Collision>,
    mut query: Query<(Entity, &mut Grounded, &Transform), With<Movable>>,
    ground_query: Query<(Entity, &Transform), (With<Ground>, Without<Movable>)>, // Query for ground entities
    platform_query: Query<(Entity, &Transform), (With<Platform>, Without<Movable>)>, // Query for platforms
) {
    let player_entities: Vec<Entity> = query.iter().map(|(entity, _, _)| entity).collect();

    for (_, mut grounded, player_transform) in &mut query {
        grounded.0 = false; // Reset grounded state each frame

        for collision in collision_events.read() {
            let contacts = &collision.0;

            if contacts.is_sensor {
                continue;
            }

            let involved_entities = [contacts.entity1, contacts.entity2];
            if !involved_entities
                .iter()
                .any(|e| player_entities.contains(e))
            {
                continue;
            }

            for entity in &involved_entities {
                if let Ok((_, ground_transform)) = ground_query.get(*entity) {
                    if player_transform.translation.y > ground_transform.translation.y {
                        grounded.0 = true;
                    }
                } else if let Ok((_, platform_transform)) = platform_query.get(*entity) {
                    if player_transform.translation.y > platform_transform.translation.y {
                        grounded.0 = true;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_gets_grounded_on_collision() {
        let mut app = App::new();

        // Add the necessary components, plugins, and systems
        app.add_plugins(MinimalPlugins) // Use minimal set to speed up tests
            .add_event::<Collision>() // Register the Collision event
            .add_systems(Update, check_grounded); // Add our system

        // Spawn a movable player entity
        let player_entity = app
            .world_mut()
            .spawn((
                Movable,
                Grounded(false),
                Transform::from_xyz(0.0, 1.0, 0.0), // Positioned above the ground
            ))
            .id();

        // Spawn a ground entity
        let ground_entity = app
            .world_mut()
            .spawn((
                Ground,
                Transform::from_xyz(0.0, 0.0, 0.0), // Ground is below the player
            ))
            .id();

        // Send a collision event where the player lands on the ground

        app.world_mut().send_event(Collision(Contacts {
            entity1: player_entity,
            entity2: ground_entity,
            body_entity1: None, // No body entities involved for testing
            body_entity2: None,
            manifolds: vec![], // Empty manifolds for simplicity in the test
            is_sensor: false,
            during_current_frame: true, // Simulate contact occurring in the current frame
            during_previous_frame: false,
            total_normal_impulse: 0.0,
            total_tangent_impulse: 0.0, // Or `Vector2::ZERO` for 3D, depending on enabled features
        }));

        // Run the app once to process the event
        app.update();

        // Check if the player is now grounded
        let grounded_component = app.world().get::<Grounded>(player_entity).unwrap();
        assert!(
            grounded_component.0,
            "Player should be grounded after collision."
        );
    }

    #[test]
    fn test_player_remains_ungrounded_without_collision() {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins)
            .add_event::<Collision>()
            .add_systems(Update, check_grounded);

        let player_entity = app
            .world_mut()
            .spawn((Movable, Grounded(false), Transform::from_xyz(0.0, 1.0, 0.0)))
            .id();

        app.update();

        let grounded_component = app.world().get::<Grounded>(player_entity).unwrap();
        assert!(
            !grounded_component.0,
            "Player should remain ungrounded without collision."
        );
    }
}
