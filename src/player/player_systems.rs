use super::player_component::{
    Action, Movable, PlayerAnimationIndices, PlayerAnimationTimer, PlayerBundle, PLAYER_JUMP_FORCE,
    PLAYER_MOVE_SPEED,
};
use super::Player;
use crate::door::Door;
use crate::door::Platform;
use crate::player::player_component::Grounded;
use crate::room::room_component::{CurrentFloorPlan, RoomState};
use crate::room::{Floor, WINDOW_HEIGHT};
use crate::state::state_component::FadeEffect;
use crate::state::GameState;
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    InputManagerBundle,
};

pub fn player_enters_new_room(
    mut commands: Commands,
    room_state: Res<RoomState>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    if !room_state.is_changed() {
        return;
    }
    if let Some(previous_room_id) = room_state.previous_room_id.clone() {
        room_state.doors.iter().for_each(|door_state| {
            if door_state.room_id == previous_room_id {
                let new_location: Vec2 = door_state.position;
                let (player_entity, _) = player_query.single();
                commands.entity(player_entity).insert(
                    Transform::from_scale(Vec3 {
                        x: 4.0,
                        y: 4.0,
                        z: 2.0,
                    })
                    .with_translation(Vec3::new(
                        new_location.x,
                        new_location.y + 50.0,
                        2.0,
                    )),
                );
            }
        });
    }
}

// pub fn animate_player(
//     time: Res<Time>,
//     mut query: Query<(
//         &PlayerAnimationIndices,
//         &mut PlayerAnimationTimer,
//         &mut Sprite,
//     )>,
// ) {
//     for (indices, mut timer, mut sprite) in &mut query {
//         timer.tick(time.delta());
//
//         if timer.just_finished() {
//             if let Some(atlas) = &mut sprite.texture_atlas {
//                 atlas.index = if atlas.index == indices.last {
//                     indices.first
//                 } else {
//                     atlas.index + 1
//                 };
//             }
//         }
//     }
// }

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    //let texture = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture = asset_server.load("gabe-idle-run.png");
    //let texture = asset_server.load("stickman.png");
    let layout = TextureAtlasLayout::from_grid(
        // note, thie stickman is bad
        UVec2::splat(24),
        7,
        1,
        None, //Some(UVec2::splat(4)),
        None, //Some(UVec2::splat(4)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = PlayerAnimationIndices { first: 1, last: 6 };

    let input_map = InputMap::new([
        (Action::Jump, KeyCode::Space),
        (Action::MoveLeft, KeyCode::ArrowLeft),
        (Action::MoveRight, KeyCode::ArrowRight),
        (Action::Enter, KeyCode::ArrowUp),
    ]);
    commands.spawn((
        InputManagerBundle::with_map(input_map),
        PlayerBundle::new(texture, texture_atlas_layout, animation_indices.clone()),
        Transform::from_scale(Vec3 {
            x: 4.0,
            y: 4.0,
            z: 2.0,
        })
        .with_translation(Vec3::new(0.0, 0.0, 2.0)),
        //Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - 50.0, 1.0),
        animation_indices,
        PlayerAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

pub fn player_movement(
    time: Res<Time>,
    mut animation_query: Query<(
        &PlayerAnimationIndices,
        &mut PlayerAnimationTimer,
        &mut Sprite,
    )>,
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
            for (indices, mut timer, mut sprite) in &mut animation_query {
                timer.tick(time.delta());

                if timer.just_finished() {
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = if atlas.index == indices.last {
                            indices.first
                        } else {
                            atlas.index + 1
                        };
                    }
                }
            }
        }
    }
}

pub fn detect_player_at_door(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    player_query: Query<&Transform, With<Player>>,
    door_query: Query<(&Transform, &Door)>,
    action_state_query: Query<&ActionState<Action>>,
    mut current_floorplan: ResMut<CurrentFloorPlan>,
    mut fade: ResMut<FadeEffect>,
) {
    if *state != GameState::InGame {
        return;
    }
    if let Ok(player_transform) = player_query.get_single() {
        for (door_transform, door) in door_query.iter() {
            let distance = player_transform
                .translation
                .distance(door_transform.translation);
            if distance < 20.0 {
                for action_state in action_state_query.iter() {
                    if action_state.pressed(&Action::Enter) {
                        current_floorplan.you_were_here = current_floorplan.you_are_here.clone();
                        current_floorplan.you_are_here = Some(door.room_id.clone());
                        next_state.set(GameState::TransitioningOut);
                        fade.fading_out = true;
                    }
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn check_grounded(
    mut collision_events: EventReader<Collision>,
    mut query: Query<(Entity, &mut Grounded, &Transform), With<Movable>>,
    ground_query: Query<(Entity, &Transform), (With<Floor>, Without<Movable>)>, // Query for ground entities
    platform_query: Query<(Entity, &Transform), (With<Platform>, Without<Movable>)>, // Query for platforms
) {
    let player_entities: Vec<Entity> = query.iter().map(|(entity, _, _)| entity).collect();

    for (_, mut grounded, player_transform) in &mut query {
        grounded.0 = false; // Reset grounded state each frame

        for collision in collision_events.read() {
            let contacts = &collision.0;

            if contacts.is_sensor {
                continue;
            }

            let involved_entities = [contacts.entity1, contacts.entity2];
            if !involved_entities
                .iter()
                .any(|e| player_entities.contains(e))
            {
                continue;
            }

            for entity in &involved_entities {
                if let Ok((_, ground_transform)) = ground_query.get(*entity) {
                    if player_transform.translation.y > ground_transform.translation.y {
                        grounded.0 = true;
                    }
                } else if let Ok((_, platform_transform)) = platform_query.get(*entity) {
                    if player_transform.translation.y > platform_transform.translation.y {
                        grounded.0 = true;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_gets_grounded_on_collision() {
        let mut app = App::new();

        // Add the necessary components, plugins, and systems
        app.add_plugins(MinimalPlugins) // Use minimal set to speed up tests
            .add_event::<Collision>() // Register the Collision event
            .add_systems(Update, check_grounded); // Add our system

        // Spawn a movable player entity
        let player_entity = app
            .world_mut()
            .spawn((
                Movable,
                Grounded(false),
                Transform::from_xyz(0.0, 1.0, 0.0), // Positioned above the ground
            ))
            .id();

        // Spawn a ground entity
        let ground_entity = app
            .world_mut()
            .spawn((
                Floor,
                Transform::from_xyz(0.0, 0.0, 0.0), // Ground is below the player
            ))
            .id();

        // Send a collision event where the player lands on the ground

        app.world_mut().send_event(Collision(Contacts {
            entity1: player_entity,
            entity2: ground_entity,
            body_entity1: None, // No body entities involved for testing
            body_entity2: None,
            manifolds: vec![], // Empty manifolds for simplicity in the test
            is_sensor: false,
            during_current_frame: true, // Simulate contact occurring in the current frame
            during_previous_frame: false,
            total_normal_impulse: 0.0,
            total_tangent_impulse: 0.0, // Or `Vector2::ZERO` for 3D, depending on enabled features
        }));

        // Run the app once to process the event
        app.update();

        // Check if the player is now grounded
        let grounded_component = app.world().get::<Grounded>(player_entity).unwrap();
        assert!(
            grounded_component.0,
            "Player should be grounded after collision."
        );
    }

    #[test]
    fn test_player_remains_ungrounded_without_collision() {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins)
            .add_event::<Collision>()
            .add_systems(Update, check_grounded);

        let player_entity = app
            .world_mut()
            .spawn((Movable, Grounded(false), Transform::from_xyz(0.0, 1.0, 0.0)))
            .id();

        app.update();

        let grounded_component = app.world().get::<Grounded>(player_entity).unwrap();
        assert!(
            !grounded_component.0,
            "Player should remain ungrounded without collision."
        );
    }
}
