use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::{prelude::*, text::TextBounds};
use bevy_lit::prelude::LightOccluder2d;

use crate::room::room_component::RoomState;

use super::door_component::{Door, Platform, BOUNCE_EFFECT, PLATFORM_HEIGHT, PLATFORM_WIDTH};

#[allow(clippy::type_complexity)]
pub fn spawn_platforms(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    room_state: Res<RoomState>,
    query: Query<Entity, With<Platform>>,
) {
    if !room_state.is_changed() {
        return;
    }

    debug!("Room state changed, respawning platforms...");

    despawn_existing_platforms(&mut commands, query);
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = create_text_font(font);

    for (position, room_name, room_id) in room_state.clone().doors.into_iter().map(|door_state| {
        (
            door_state.position,
            door_state.room_name,
            door_state.room_id,
        )
    }) {
        spawn_platform(&mut commands, position, &text_font, room_name, room_id);
    }
}

#[allow(clippy::type_complexity)]
fn despawn_existing_platforms(commands: &mut Commands, query: Query<Entity, With<Platform>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
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
    room_id: String,
) {
    let platform_components = (
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
        LightOccluder2d::default(),
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
            ..default()
        },
    );

    let text_components = (
        Text2d::new(room_name.clone()),
        text_font.clone(),
        TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
        TextBounds::from(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
        Transform::from_translation(Vec3::Z),
    );

    let door_components = (
        Door { room_id, room_name },
        Transform::from_xyz(0.0, PLATFORM_HEIGHT / 2.0 + PLATFORM_WIDTH / 4.0, 0.1),
        LightOccluder2d::default(),
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(PLATFORM_WIDTH / 4.0, PLATFORM_WIDTH / 2.0)),
            ..default()
        },
    );

    commands
        .spawn(platform_components)
        .with_children(|builder| {
            builder.spawn(text_components);
            builder.spawn(door_components);
        });
}
