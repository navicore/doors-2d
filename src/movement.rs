use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{environ::Ground, platform::Platform, schedule::InGameSet};

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
    ground_query: Query<Entity, (With<Ground>, Without<Movable>)>, // Query for ground entities
    platform_query: Query<Entity, (With<Platform>, Without<Movable>)>, // Query for platforms
) {
    let player_entities: Vec<Entity> = query.iter().map(|(entity, _)| entity).collect();

    for (_, mut grounded) in &mut query {
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

            if involved_entities
                .iter()
                .any(|e| ground_query.get(*e).is_ok() || platform_query.get(*e).is_ok())
            {
                grounded.0 = true;
            }
        }
    }
}
