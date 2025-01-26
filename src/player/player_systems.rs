use super::player_component::{Action, PlayerBundle, PLAYER_JUMP_FORCE, PLAYER_MOVE_SPEED};
use super::Player;
use crate::environ::WINDOW_HEIGHT;
use crate::{movement::Grounded, platform::Door};
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    InputManagerBundle,
};

pub fn spawn_player(mut commands: Commands) {
    let input_map = InputMap::new([
        (Action::Jump, KeyCode::Space),
        (Action::MoveLeft, KeyCode::ArrowLeft),
        (Action::MoveRight, KeyCode::ArrowRight),
        (Action::Enter, KeyCode::ArrowUp),
    ]);
    commands.spawn((
        InputManagerBundle::with_map(input_map),
        PlayerBundle::new(),
        Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - 50.0, 1.0),
    ));
}

pub fn player_movement(
    mut query: Query<(&mut ExternalForce, &Grounded, &ActionState<Action>), With<Player>>,
) {
    if let Ok((mut force, grounded, action_state)) = query.get_single_mut() {
        force.set_force(Vec2::ZERO);

        if grounded.0 && action_state.pressed(&Action::Jump) {
            force.apply_force(Vec2::new(0.0, PLAYER_JUMP_FORCE));
        }
        if action_state.pressed(&Action::MoveLeft) {
            force.apply_force(Vec2::new(-PLAYER_MOVE_SPEED, 0.0));
        }
        if action_state.pressed(&Action::MoveRight) {
            force.apply_force(Vec2::new(PLAYER_MOVE_SPEED, 0.0));
        }
    }
}

pub fn detect_player_at_door(
    player_query: Query<&Transform, With<Player>>,
    door_query: Query<&Transform, With<Door>>,
    action_state_query: Query<&ActionState<Action>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for door_transform in door_query.iter() {
            let distance = player_transform
                .translation
                .distance(door_transform.translation);
            if distance < 20.0 {
                for action_state in action_state_query.iter() {
                    if action_state.pressed(&Action::Enter) {
                        info!("Player is in front of the door and pressed the enter door action!");
                        // Add your logic for entering the door here
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{player::PlayerPlugin, SchedulePlugin};
    use bevy::input::InputPlugin;

    #[test]
    fn test_player_spawning() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(InputPlugin)
            .add_plugins(SchedulePlugin)
            .add_plugins(PlayerPlugin);

        // Run the startup systems to spawn the player
        app.update();

        // Get the world reference
        let world = app.world_mut();

        // Query for the player
        let mut query = world.query::<(&Transform, &Player)>();
        let players: Vec<_> = query.iter(world).collect();

        // Check that the player was spawned
        assert_eq!(
            players.len(),
            1,
            "Expected 1 player, found {}",
            players.len()
        );

        // Check the properties of the player
        let (transform, _) = players[0];
        assert_eq!(transform.translation.x, 0.0);
        assert_eq!(transform.translation.y, WINDOW_HEIGHT / 2.0 - 50.0);
    }

    #[test]
    fn test_input_handling() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(InputPlugin)
            .add_plugins(SchedulePlugin)
            .add_plugins(PlayerPlugin);
        // Run the startup systems to spawn the player
        app.update();
        // Get the world reference
        let world = app.world_mut();
        // Query for the player
        let mut query = world.query::<(&mut ActionState<Action>, &Player)>();
        let players: Vec<_> = query.iter(world).collect();
        // Check that the player was spawned
        assert_eq!(
            players.len(),
            1,
            "Expected 1 player, found {}",
            players.len()
        );
        // Check the properties of the player
        let (action_state, _) = players[0];
        // Check that the player is not moving
        assert!(!action_state.pressed(&Action::MoveLeft));
        assert!(!action_state.pressed(&Action::MoveRight));
        assert!(!action_state.pressed(&Action::Jump));
        // Simulate pressing the left arrow key
        // TODO
    }
}
