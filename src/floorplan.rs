use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;

#[derive(Clone)]
struct Room {
    pub id: String,
    pub name: String,
}

#[derive(Clone)]
struct Door {
    pub id: String,
    pub name: String,
}

struct FloorPlan {
    graph: DiGraph<Room, Door>,
}

impl FloorPlan {
    pub fn new() -> Self {
        FloorPlan {
            graph: DiGraph::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) -> NodeIndex {
        self.graph.add_node(room)
    }

    pub fn add_door(&mut self, from: NodeIndex, to: NodeIndex, door: Door) {
        self.graph.add_edge(from, to, door);
    }

    pub fn get_doors(&self, room_index: NodeIndex) -> Vec<&Door> {
        self.graph
            .edges(room_index)
            .map(|edge| edge.weight())
            .collect()
    }

    pub fn get_connected_room(&self, room_index: NodeIndex, door_id: &str) -> Option<&Room> {
        for edge in self.graph.edges(room_index) {
            if edge.weight().id == door_id {
                let target_index = edge.target();
                return self.graph.node_weight(target_index);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_room_and_door() {
        let mut floor_plan = FloorPlan::new();

        let room1 = Room {
            id: "1".to_string(),
            name: "Room 1".to_string(),
        };
        let room2 = Room {
            id: "2".to_string(),
            name: "Room 2".to_string(),
        };

        let room1_index = floor_plan.add_room(room1);
        let room2_index = floor_plan.add_room(room2);

        let door = Door {
            id: "1".to_string(),
            name: "Door 1".to_string(),
        };
        floor_plan.add_door(room1_index, room2_index, door);

        let doors = floor_plan.get_doors(room1_index);
        assert_eq!(doors.len(), 1);
        assert_eq!(doors[0].name, "Door 1");

        if let Some(connected_room) = floor_plan.get_connected_room(room1_index, &doors[0].id) {
            assert_eq!(connected_room.name, "Room 2");
        } else {
            panic!("Connected room not found");
        }
    }

    #[test]
    fn test_get_connected_room() {
        let mut floor_plan = FloorPlan::new();

        let room1 = Room {
            id: "1".to_string(),
            name: "Room 1".to_string(),
        };
        let room2 = Room {
            id: "2".to_string(),
            name: "Room 2".to_string(),
        };
        let room3 = Room {
            id: "3".to_string(),
            name: "Room 3".to_string(),
        };

        let room1_index = floor_plan.add_room(room1);
        let room2_index = floor_plan.add_room(room2);
        let room3_index = floor_plan.add_room(room3);

        let door1 = Door {
            id: "1".to_string(),
            name: "Door 1".to_string(),
        };
        let door2 = Door {
            id: "2".to_string(),
            name: "Door 2".to_string(),
        };

        floor_plan.add_door(room1_index, room2_index, door1);
        floor_plan.add_door(room2_index, room3_index, door2);

        let doors = floor_plan.get_doors(room2_index);
        assert_eq!(doors.len(), 1);

        if let Some(connected_room) = floor_plan.get_connected_room(room2_index, &doors[0].id) {
            assert_eq!(connected_room.name, "Room 3");
        } else {
            panic!("Connected room not found for door 1");
        }

        if let Some(connected_room) = floor_plan.get_connected_room(room2_index, &doors[0].id) {
            assert_eq!(connected_room.name, "Room 3");
        } else {
            panic!("Connected room not found for door 2");
        }
    }
}
