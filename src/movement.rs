use avian2d::prelude::*;
use bevy::prelude::*;

use crate::schedule::InGameSet;

#[derive(Component)]
pub struct Grounded(pub bool);

#[derive(Component)]
pub struct Movable;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_grounded.in_set(InGameSet::CollisionDetection));
    }
}

fn check_grounded(
    mut collision_events: EventReader<Collision>,
    mut query: Query<(Entity, &mut Grounded), With<Movable>>,
) {
    if query.is_empty() {
        warn!("No entities found with Movable and Grounded components.");
    }
    let entities: Vec<Entity> = query.iter().map(|(entity, _)| entity).collect();

    for (_, mut grounded) in &mut query {
        grounded.0 = false; // Reset grounded state each frame

        for collision in collision_events.read() {
            let contacts = &collision.0;

            // Ensure the collision is not a sensor and check if the player is one of the entities
            if !contacts.is_sensor
                && (entities.contains(&contacts.entity1) || entities.contains(&contacts.entity2))
            {
                // Check if the collision happened during the current frame
                if contacts.during_current_frame {
                    grounded.0 = true;
                }
            }
        }
    }
}
