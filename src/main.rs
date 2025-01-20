use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;

// Define window size
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

// Player movement constants
const PLAYER_SPEED: f32 = 1000.0; // Increase speed significantly
const JUMP_FORCE: f32 = 2000.0; // Stronger jump force

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Kubernetes Platformer".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn the camera
    commands.spawn(Camera2d);

    // Spawn the ground with physics
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(WINDOW_WIDTH / 2.0, 20.0)), // Ground collider
        Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + 20.0, 0.0),
    ));

    commands.spawn((
        RigidBody::Dynamic,
        Collider::from(SharedShape::cuboid(40.0, 40.0)),
        ExternalForce::default(),
        GravityScale(1.0),
        Mass(1.0), // Lower mass for better responsiveness to forces
        Sprite {
            color: Color::srgb(0.3, 0.6, 1.0),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        Player,
    ));

    info!("Setup complete.");
}

// Handle player movement using physics forces
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut ExternalForce, With<Player>>,
) {
    let mut force = query.single_mut();

    // Reset horizontal force each frame
    force.set_force(Vec2::ZERO);

    // Apply left/right movement forces
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        force.apply_force(Vec2::new(-PLAYER_SPEED, 0.0));
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        force.apply_force(Vec2::new(PLAYER_SPEED, 0.0));
    }

    // Apply jump force only when space is pressed
    if keyboard_input.just_pressed(KeyCode::Space) {
        force.apply_force(Vec2::new(0.0, JUMP_FORCE));
    }
}
