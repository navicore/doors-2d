use bevy::prelude::*;

use crate::floorplan::FloorPlan;

pub const WINDOW_WIDTH: f32 = 1200.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

#[derive(Component)]
pub struct Ground;

#[derive(Default, Resource)]
pub struct CurrentFloorPlan {
    pub floorplan: Option<FloorPlan>,
}

#[derive(Resource)]
pub struct EnvironState {
    pub wall_distance_from_center: f32,
    pub floor_ceiling_width: f32,
    pub boundary_thickness: f32,
    pub bounce_effect: f32,
}

impl Default for EnvironState {
    fn default() -> Self {
        Self {
            wall_distance_from_center: 1500.0,
            floor_ceiling_width: 3000.0,
            boundary_thickness: 0.1,
            bounce_effect: 0.4,
        }
    }
}
