use super::door_component::{Door, Platform, BOUNCE_EFFECT, PLATFORM_HEIGHT, PLATFORM_WIDTH};
use crate::room::room_component::{DoorState, RoomState};
use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::{
    color::palettes::{css::GREY, tailwind::BLUE_600},
    prelude::*,
    sprite::Anchor,
    text::TextBounds,
};
use bevy_lit::prelude::{LightOccluder2d, PointLight2d};

const PLATFORM_LAYER: f32 = 0.0;
const TEXT_LAYER: f32 = 100.0;
const DOOR_LAYER: f32 = 0.0;
const LIGHT_LAYER: f32 = 0.0;

#[allow(clippy::type_complexity)]
pub fn spawn_platforms(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    room_state: Res<RoomState>,
    query: Query<Entity, With<Platform>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if !room_state.is_changed() {
        return;
    }

    //let room_name = plan.you_are_here.clone().unwrap_or("unknown".to_string());
    let room_name = room_state
        .room_id
        .clone()
        .unwrap_or_else(|| "unknown".to_string());

    debug!("Room state changed, respawning platforms...");

    despawn_existing_platforms(&mut commands, query);
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = create_text_font(font.clone());
    let sign_font = create_sign_font(font);

    for door_state in room_state.clone().doors {
        spawn_platform(
            &mut commands,
            room_state.clone(),
            door_state,
            &text_font,
            &sign_font,
            &mut meshes,
            room_name.clone(),
        );
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

fn create_sign_font(font: Handle<Font>) -> TextFont {
    TextFont {
        font,
        font_size: 18.0,
        ..default()
    }
}

fn spawn_platform(
    commands: &mut Commands,
    room_state: RoomState,
    door_state: DoorState,
    text_font: &TextFont,
    sign_font: &TextFont,
    meshes: &mut ResMut<Assets<Mesh>>,
    room_name: String,
) {
    debug!("Spawning platform for room: {room_name}");
    let platform_shape = meshes.add(Rectangle::new(PLATFORM_WIDTH, PLATFORM_HEIGHT));

    let platform_component = (
        Mesh2d(platform_shape),
        LightOccluder2d::default(),
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            PLATFORM_WIDTH / 2.0,
            PLATFORM_HEIGHT / 2.0,
        )),
        Transform::from_xyz(door_state.position.x, door_state.position.y, PLATFORM_LAYER),
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
    );

    let text_component = (
        Text2d::new(door_state.room_name.clone()),
        text_font.clone(),
        Anchor::Center,
        TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
        TextBounds::from(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
        Transform::from_translation(Vec3::new(0.0, 0.0, TEXT_LAYER)), // Set a high z value
    );

    let exit_text_component = room_state.previous_room_id.and_then(|previous_room_id| {
        if door_state.room_id == previous_room_id {
            Some((
                Text2d::new(format!("You are in the {room_name} room.")),
                TextColor(bevy::prelude::Color::Srgba(GREY)),
                sign_font.clone(),
                Anchor::Center,
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                TextBounds::from(Vec2::new(PLATFORM_WIDTH * 2.0, PLATFORM_HEIGHT * 2.0)),
                Transform::from_translation(Vec3::new(0.0, -50.0, TEXT_LAYER)), // Set a high z value
            ))
        } else {
            None
        }
    });

    let door_component = (
        Door {
            room_id: door_state.room_id,
        },
        Transform::from_xyz(
            0.0,
            PLATFORM_HEIGHT / 2.0 + PLATFORM_WIDTH / 4.0,
            DOOR_LAYER,
        ),
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(PLATFORM_WIDTH / 4.0, PLATFORM_WIDTH / 2.0)),
            ..default()
        },
    );

    let light_component = (
        PointLight2d {
            intensity: 1.5,
            radius: 600.0,
            falloff: 2.0,
            color: Color::from(BLUE_600),
            ..default()
        },
        Transform::from_xyz(
            0.0,
            PLATFORM_HEIGHT.mul_add(-2.0, PLATFORM_WIDTH),
            LIGHT_LAYER,
        ),
    );

    commands.spawn(platform_component).with_children(|builder| {
        builder.spawn(text_component);
        builder.spawn(door_component);
        builder.spawn(light_component);
        if let Some(exit_text_component) = exit_text_component {
            builder.spawn(exit_text_component);
        }
    });
}
