use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::{prelude::*, text::TextBounds};

use crate::{room::room_component::RoomState, state::GameState};

use super::door_component::{Door, Platform, BOUNCE_EFFECT, PLATFORM_HEIGHT, PLATFORM_WIDTH};

#[allow(clippy::type_complexity)]
pub fn spawn_platforms(
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    room_state: Res<RoomState>,
    param_set: ParamSet<(Query<Entity, With<Platform>>, Query<Entity, With<Door>>)>,
) {
    if !room_state.is_changed() {
        return;
    }

    debug!("Room state changed, respawning platforms...");

    despawn_existing_platforms(&mut commands, param_set);
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = create_text_font(font);

    for (position, room_name, room_id) in room_state.clone().doors.into_iter().map(|door_state| {
        (
            door_state.position,
            door_state.room_name,
            door_state.room_id,
        )
    }) {
        spawn_platform(&mut commands, position, &text_font, room_name.clone());
        spawn_door(&mut commands, position, room_id, room_name);
    }
    //next_state.set(GameState::TransitioningIn);
}

#[allow(clippy::type_complexity)]
fn despawn_existing_platforms(
    commands: &mut Commands,
    mut param_set: ParamSet<(Query<Entity, With<Platform>>, Query<Entity, With<Door>>)>,
) {
    for entity in param_set.p0().iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in param_set.p1().iter() {
        commands.entity(entity).despawn();
    }
}

fn create_text_font(font: Handle<Font>) -> TextFont {
    TextFont {
        font,
        font_size: 14.0,
        ..default()
    }
}

fn spawn_platform(
    commands: &mut Commands,
    position: Vec2,
    text_font: &TextFont,
    room_name: String,
) {
    commands
        .spawn((
            RigidBody::Static,
            Collider::from(SharedShape::cuboid(
                PLATFORM_WIDTH / 2.0,
                PLATFORM_HEIGHT / 2.0,
            )),
            Transform::from_xyz(position.x, position.y, 0.0),
            Friction {
                dynamic_coefficient: 0.6,
                static_coefficient: 0.8,
                combine_rule: CoefficientCombine::Average,
            },
            Restitution {
                coefficient: BOUNCE_EFFECT,
                combine_rule: CoefficientCombine::Max,
            },
            Platform {},
            Sprite {
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new(room_name),
                text_font.clone(),
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                TextBounds::from(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
                Transform::from_translation(Vec3::Z),
            ));
        });
}

fn spawn_door(commands: &mut Commands, position: Vec2, room_id: String, room_name: String) {
    commands.spawn((
        Door { room_id, room_name },
        Transform::from_xyz(
            position.x,
            position.y + PLATFORM_HEIGHT / 2.0 + PLATFORM_WIDTH / 4.0,
            0.1,
        ),
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(PLATFORM_WIDTH / 4.0, PLATFORM_WIDTH / 2.0)),
            ..default()
        },
    ));
}
