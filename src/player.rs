use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;
use leafwing_input_manager::{
    plugin::InputManagerPlugin,
    prelude::{ActionState, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::{
    environ::WINDOW_HEIGHT,
    movement::{Grounded, Movable},
    schedule::InGameSet,
    state::GameState,
};

// Define movement constants
const PLAYER_MOVE_SPEED: f32 = 500.0; // Horizontal movement speed
const PLAYER_JUMP_FORCE: f32 = 25000.0; // Jump force applied when pressing space
const PLAYER_GRAVITY_SCALE: f32 = 25.0; // Gravity multiplier for falling speed

#[derive(Bundle)]
pub struct PlayerBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub external_force: ExternalForce,
    pub gravity: GravityScale,
    pub mass: Mass,
    pub friction: Friction,
    pub sprite: Sprite,
    pub player: Player,
    pub movable: Movable,
    pub grounded: Grounded,
}

impl PlayerBundle {
    pub fn new() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::from(SharedShape::cuboid(40.0, 40.0)),
            external_force: ExternalForce::default(),
            gravity: GravityScale(PLAYER_GRAVITY_SCALE),
            mass: Mass(1.0),
            friction: Friction {
                dynamic_coefficient: 0.3,
                static_coefficient: 0.5,
                combine_rule: CoefficientCombine::Average,
            },
            sprite: Sprite {
                color: Color::srgb(0.3, 0.6, 1.0),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            player: Player,
            movable: Movable,
            grounded: Grounded(false),
        }
    }
}
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(
                Update,
                player_movement
                    .in_set(InGameSet::UserInput)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_plugins(InputManagerPlugin::<Action>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    MoveLeft,
    MoveRight,
    Jump,
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    let input_map = InputMap::new([
        (Action::Jump, KeyCode::Space),
        (Action::MoveLeft, KeyCode::ArrowLeft),
        (Action::MoveRight, KeyCode::ArrowRight),
    ]);
    commands.spawn((
        InputManagerBundle::with_map(input_map),
        PlayerBundle::new(),
        Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - 50.0, 1.0),
    ));
}

fn player_movement(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SchedulePlugin;
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
