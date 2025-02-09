use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;
use leafwing_input_manager::Actionlike;
use serde::Deserialize;

// Define movement constants
pub const PLAYER_MOVE_SPEED: f32 = 500.0; // Horizontal movement speed
pub const PLAYER_JUMP_FORCE: f32 = 25000.0; // Jump force applied when pressing space
pub const PLAYER_GRAVITY_SCALE: f32 = 25.0; // Gravity multiplier for falling speed

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

//    pub fn from_atlas_image(image: Handle<Image>, atlas: TextureAtlas) -> Self {
impl PlayerBundle {
    pub fn new(
        texture: Handle<Image>,
        texture_atlas_layout: Handle<TextureAtlasLayout>,
        animation_indices: PlayerAnimationIndices,
    ) -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            //collider: Collider::from(SharedShape::cuboid(40.0, 40.0)),
            collider: Collider::from(SharedShape::cuboid(15.0, 15.0)), // todo: base on image
            external_force: ExternalForce::default(),
            gravity: GravityScale(PLAYER_GRAVITY_SCALE),
            mass: Mass(1.0),
            friction: Friction {
                dynamic_coefficient: 0.3,
                static_coefficient: 0.5,
                combine_rule: CoefficientCombine::Average,
            },
            sprite: Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
            ),
            player: Player,
            movable: Movable,
            grounded: Grounded(false),
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    MoveLeft,
    MoveRight,
    Jump,
    Enter,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Grounded(pub bool);

#[derive(Component)]
pub struct Movable;

#[derive(Component, Clone)]
pub struct PlayerAnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct PlayerAnimationTimer(pub Timer);

#[derive(Deserialize)]
pub struct Frame {
    pub frame: Rect,
    pub rotated: bool,
    pub trimmed: bool,
    pub spriteSourceSize: Rect,
    pub sourceSize: Size,
    pub duration: u32,
}

#[derive(Deserialize)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Deserialize)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

#[derive(Deserialize)]
pub struct SpriteSheetData {
    pub frames: std::collections::HashMap<String, Frame>,
}
