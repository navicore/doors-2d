use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::{color::palettes::tailwind::BLUE_600, prelude::*, sprite::Anchor, text::TextBounds};
use bevy_lit::prelude::{LightOccluder2d, PointLight2d};

use crate::room::room_component::{CurrentFloorPlan, DoorState, RoomState};

use super::door_component::{Door, Platform, BOUNCE_EFFECT, PLATFORM_HEIGHT, PLATFORM_WIDTH};

#[allow(clippy::type_complexity)]
pub fn spawn_platforms(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    room_state: Res<RoomState>,
    query: Query<Entity, With<Platform>>,
    mut meshes: ResMut<Assets<Mesh>>,
    plan: Res<CurrentFloorPlan>,
) {
    if !room_state.is_changed() {
        return;
    }

    let room_name = plan.you_are_here.clone().unwrap_or("unknown".to_string());

    debug!("Room state changed, respawning platforms...");

    despawn_existing_platforms(&mut commands, query);
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = create_text_font(font);

    for door_state in room_state.clone().doors {
        spawn_platform(
            &mut commands,
            door_state,
            &text_font,
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

//        spawn_platform(&mut commands, door_state, &text_font, &mut meshes);
fn spawn_platform(
    commands: &mut Commands,
    door_state: DoorState,
    text_font: &TextFont,
    meshes: &mut ResMut<Assets<Mesh>>,
    room_name: String,
) {
    let platform_shape = meshes.add(Rectangle::new(PLATFORM_WIDTH, PLATFORM_HEIGHT));

    let platform_component = (
        Mesh2d(platform_shape),
        LightOccluder2d::default(),
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            PLATFORM_WIDTH / 2.0,
            PLATFORM_HEIGHT / 2.0,
        )),
        Transform::from_xyz(door_state.position.x, door_state.position.y, 0.0),
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
        Transform::from_translation(Vec3::Z),
    );

    let exit_text_component = if door_state.is_exit {
        Some((
            Text2d::new(format!("You are here: {}", room_name)),
            text_font.clone(),
            Anchor::Center,
            TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
            TextBounds::from(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
            Transform::from_xyz(0.0, -50.0, 1.0),
        ))
    } else {
        None
    };

    let door_component = (
        Door {
            room_id: door_state.room_id,
            room_name: door_state.room_name,
        },
        Transform::from_xyz(0.0, PLATFORM_HEIGHT / 2.0 + PLATFORM_WIDTH / 4.0, 0.0),
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
        Transform::from_xyz(0.0, PLATFORM_HEIGHT.mul_add(-2.0, PLATFORM_WIDTH), 0.0),
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
