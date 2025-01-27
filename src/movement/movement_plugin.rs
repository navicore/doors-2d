use bevy::prelude::*;

use super::movement_systems::check_grounded;
use crate::scheduler::schedule_plugin::InGameSet;

///the main movement this plugin manages is the grounded state of the player
pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_grounded.in_set(InGameSet::CollisionDetection));
    }
}
