use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;

// Define window size
const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;
const BOUNDARY_THICKNESS: f32 = 0.5;
const BOUNCE_EFFECT: f32 = 0.4;

// Define movement constants
const PLAYER_MOVE_SPEED: f32 = 1000.0; // Horizontal movement speed
const PLAYER_JUMP_FORCE: f32 = 35000.0; // Jump force applied when pressing space
const PLAYER_GRAVITY_SCALE: f32 = 25.0; // Gravity multiplier for falling speed

#[derive(Component)]
struct Grounded(bool);

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
        .add_systems(Update, (player_movement, check_grounded))
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn the camera
    commands.spawn(Camera2d);

    // Spawn the ground with physics and centralized boundary thickness
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(WINDOW_WIDTH / 2.0, BOUNDARY_THICKNESS)),
        Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + BOUNDARY_THICKNESS, 0.0),
        Friction {
            dynamic_coefficient: 0.8,
            static_coefficient: 0.9,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: BOUNCE_EFFECT % 2.0,
            combine_rule: CoefficientCombine::Max,
        },
    ));

    // Left wall
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(BOUNDARY_THICKNESS, WINDOW_HEIGHT / 2.0)),
        Transform::from_xyz(-WINDOW_WIDTH / 2.0 + BOUNDARY_THICKNESS, 0.0, 0.0),
        Friction {
            dynamic_coefficient: 0.5,
            static_coefficient: 0.6,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: BOUNCE_EFFECT,
            combine_rule: CoefficientCombine::Max,
        },
    ));

    // Right wall
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(BOUNDARY_THICKNESS, WINDOW_HEIGHT / 2.0)),
        Transform::from_xyz(WINDOW_WIDTH / 2.0 - BOUNDARY_THICKNESS, 0.0, 0.0),
        Friction {
            dynamic_coefficient: 0.5,
            static_coefficient: 0.6,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: BOUNCE_EFFECT,
            combine_rule: CoefficientCombine::Max,
        },
    ));

    // Top boundary
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(WINDOW_WIDTH / 2.0, BOUNDARY_THICKNESS)),
        Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - BOUNDARY_THICKNESS, 0.0),
        Restitution {
            coefficient: BOUNCE_EFFECT,
            combine_rule: CoefficientCombine::Max,
        },
    ));

    // Spawn the player with physics
    commands.spawn((
        RigidBody::Dynamic,
        Collider::from(SharedShape::cuboid(40.0, 40.0)),
        ExternalForce::default(),
        GravityScale(PLAYER_GRAVITY_SCALE), // Use gravity scale constant
        Mass(1.0),
        Friction {
            dynamic_coefficient: 0.3,
            static_coefficient: 0.5,
            combine_rule: CoefficientCombine::Average,
        },
        Sprite {
            color: Color::srgb(0.3, 0.6, 1.0),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        Player,
        Grounded(false),
    ));

    info!("Setup complete.");
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut ExternalForce, &Grounded), With<Player>>,
) {
    if let Ok((mut force, grounded)) = query.get_single_mut() {
        force.set_force(Vec2::ZERO);

        // Restrict movement to when the player is on the ground
        if grounded.0 {
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                force.apply_force(Vec2::new(-PLAYER_MOVE_SPEED, 0.0));
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                force.apply_force(Vec2::new(PLAYER_MOVE_SPEED, 0.0));
            }

            if keyboard_input.just_pressed(KeyCode::Space) {
                force.apply_force(Vec2::new(0.0, PLAYER_JUMP_FORCE));
            }
        }
    }
}

fn check_grounded(
    mut collision_events: EventReader<Collision>,
    mut query: Query<(Entity, &mut Grounded), With<Player>>,
) {
    // Collect all player entities to avoid mutable borrow conflict
    let player_entities: Vec<Entity> = query.iter().map(|(entity, _)| entity).collect();

    for (_, mut grounded) in &mut query {
        grounded.0 = false; // Reset grounded state each frame

        for collision in collision_events.read() {
            let contacts = &collision.0;

            // Ensure the collision is not a sensor and check if the player is one of the entities
            if !contacts.is_sensor
                && (player_entities.contains(&contacts.entity1)
                    || player_entities.contains(&contacts.entity2))
            {
                // Check if the collision happened during the current frame
                if contacts.during_current_frame {
                    grounded.0 = true;
                }
            }
        }
    }
}
