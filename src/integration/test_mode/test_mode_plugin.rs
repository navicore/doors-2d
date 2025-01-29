use crate::floorplan::FloorPlanEvent;

use super::test_mode_systems::fire_floor_plan_event;
use bevy::prelude::*;

pub struct TestModeIntegrationPlugin;

impl Plugin for TestModeIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FloorPlanEvent>()
            .add_systems(Startup, fire_floor_plan_event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::floorplan::FloorPlanEvent;

    #[test]
    fn test_fire_floor_plan_event() {
        // Create a new Bevy app and add the necessary plugins and events
        let mut app = App::new();
        app.add_plugins(TestModeIntegrationPlugin);
        app.add_event::<FloorPlanEvent>();

        // Run the startup systems
        app.update();

        // Get the world from the app and retrieve the FloorPlanEvent resource
        let world = app.world();
        let events = world.get_resource::<Events<FloorPlanEvent>>().unwrap();
        let mut event_reader = events.get_cursor();
        let events: Vec<&FloorPlanEvent> = event_reader.read(events).collect();

        // Assert that one event was fired
        assert_eq!(events.len(), 1);

        // Validate the floor plan event details
        let plan = &events[0].floor_plan;
        let start_room = plan.get_start_room().unwrap();
        assert_eq!(start_room.id, "1");

        let doors_and_rooms = plan.get_doors_and_connected_rooms(&start_room.id).unwrap();
        assert_eq!(doors_and_rooms.len(), 1);

        let other_room = doors_and_rooms[0].1;
        assert_eq!(other_room.id, "2");
    }
}
