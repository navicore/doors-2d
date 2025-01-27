use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;

use super::platform_component::{
    Door, Platform, BOUNCE_EFFECT, PLATFORM_HEIGHT, PLATFORM_WIDTH, PLATFORM_Y_POS,
};

pub fn spawn_platforms(mut commands: Commands) {
    let platform_positions = vec![
        Vec2::new(-300.0, PLATFORM_Y_POS),
        Vec2::new(150.0, PLATFORM_Y_POS + 100.0),
        Vec2::new(400.0, PLATFORM_Y_POS - 50.0),
    ];

    for position in platform_positions {
        // Spawn the platform
        commands.spawn((
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
            Platform,
            Sprite {
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
                ..default()
            },
        ));

        // Spawn the door on top of the platform
        commands.spawn((
            Door,
            Transform::from_xyz(
                position.x,
                position.y + PLATFORM_HEIGHT / 2.0 + PLATFORM_WIDTH / 4.0,
                0.1,
            ), // Adjust the position to sit on the platform
            Sprite {
                color: Color::srgb(0.3, 0.3, 0.3),
                custom_size: Some(Vec2::new(PLATFORM_WIDTH / 4.0, PLATFORM_WIDTH / 2.0)), // Twice as tall as wide
                ..default()
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::platform::platform_component::{PLATFORM_HEIGHT, PLATFORM_WIDTH, PLATFORM_Y_POS};
    use crate::platform::Platform;
    use crate::platform::PlatformPlugin;
    use bevy::prelude::*;

    #[test]
    fn test_platform_spawning() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(PlatformPlugin);

        // Run the startup systems to spawn the platforms
        app.update();

        // Get the world reference
        let world = app.world_mut();

        // Query for the platforms
        let mut query = world.query::<(&Transform, &Sprite, &Platform)>();
        let platforms: Vec<_> = query.iter(world).collect();

        // Check that the correct number of platforms were spawned
        assert_eq!(platforms.len(), 3);

        // Check the properties of each platform
        let expected_positions = [
            Vec2::new(-300.0, PLATFORM_Y_POS),
            Vec2::new(150.0, PLATFORM_Y_POS + 100.0),
            Vec2::new(400.0, PLATFORM_Y_POS - 50.0),
        ];

        for (i, (transform, sprite, _)) in platforms.iter().enumerate() {
            assert_eq!(transform.translation.x, expected_positions[i].x);
            assert_eq!(transform.translation.y, expected_positions[i].y);
            assert_eq!(sprite.color, Color::srgb(0.5, 0.5, 0.5));
            assert_eq!(
                sprite.custom_size.unwrap(),
                Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)
            );
        }
    }
}
