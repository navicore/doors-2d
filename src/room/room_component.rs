use std::time::Duration;

use bevy::prelude::*;

use crate::floorplan::FloorPlan;

#[derive(Component)]
pub struct Floor;
#[derive(Component)]
pub struct LeftWall;
#[derive(Component)]
pub struct RightWall;
#[derive(Component)]
pub struct Ceiling;

#[derive(Default, Resource)]
pub struct CurrentFloorPlan {
    pub floorplan: Option<FloorPlan>,
    pub refreshed: Duration, // update every time we sync to the external state
    #[allow(dead_code)]
    pub modified: Duration, // update every time we modify due to changes in the external world
    pub you_are_here: Option<String>,
    pub you_were_here: Option<String>,
}

#[derive(Resource, Clone)]
pub struct DoorState {
    pub room_id: String,
    pub room_name: String,
    pub position: Vec2,
    #[allow(dead_code)]
    pub is_exit: bool,
}

#[derive(Resource, Clone)]
pub struct RoomState {
    pub wall_distance_from_center: f32,
    pub floor_ceiling_width: f32,
    pub boundary_thickness: f32,
    pub bounce_effect: f32,
    pub doors: Vec<DoorState>,
    pub room_id: Option<String>,
    pub previous_room_id: Option<String>,
}

// this is temporary until we can order the recalculation of room size
const DEFAULT_WALL_DISTANCE_FROM_CENTER: f32 = 1500.0;

impl Default for RoomState {
    fn default() -> Self {
        Self {
            wall_distance_from_center: DEFAULT_WALL_DISTANCE_FROM_CENTER,
            floor_ceiling_width: DEFAULT_WALL_DISTANCE_FROM_CENTER * 2.0,
            boundary_thickness: 0.1,
            bounce_effect: 0.4,
            doors: vec![],
            room_id: None,
            previous_room_id: None,
        }
    }
}
