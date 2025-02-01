use bevy::prelude::*;

use crate::floorplan::FloorPlan;

pub const WINDOW_WIDTH: f32 = 1200.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

#[derive(Component)]
pub struct Ground;
#[derive(Component)]
pub struct LeftWall;
#[derive(Component)]
pub struct RightWall;
#[derive(Component)]
pub struct TopBoundary;

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

// this is temporary until we can order the recalculation of room size
const DEFAULT_WALL_DISTANCE_FROM_CENTER: f32 = 1500.0;

impl Default for EnvironState {
    fn default() -> Self {
        Self {
            wall_distance_from_center: DEFAULT_WALL_DISTANCE_FROM_CENTER,
            floor_ceiling_width: DEFAULT_WALL_DISTANCE_FROM_CENTER * 2.0,
            boundary_thickness: 0.1,
            bounce_effect: 0.4,
        }
    }
}
