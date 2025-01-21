use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;

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
        app.add_systems(PostStartup, spawn_player).add_systems(
            Update,
            player_movement
                .in_set(InGameSet::UserInput)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        PlayerBundle::new(),
        Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - 50.0, 0.0),
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut ExternalForce, &Grounded), With<Player>>,
) {
    if let Ok((mut force, grounded)) = query.get_single_mut() {
        force.set_force(Vec2::ZERO);

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            force.apply_force(Vec2::new(-PLAYER_MOVE_SPEED, 0.0));
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            force.apply_force(Vec2::new(PLAYER_MOVE_SPEED, 0.0));
        }

        if grounded.0 && keyboard_input.pressed(KeyCode::Space) {
            force.apply_force(Vec2::new(0.0, PLAYER_JUMP_FORCE));
        }
    }
}
