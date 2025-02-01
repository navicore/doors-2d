use bevy::prelude::*;

use crate::floorplan::FloorPlan;

#[derive(Component)]
pub struct Ground;

#[derive(Default, Resource)]
pub struct CurrentFloorPlan {
    pub floorplan: Option<FloorPlan>,
}
