use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    TransitioningOut,
    TransitioningIn,
}

#[derive(Debug, Resource)]
pub struct Transition {
    pub progress: f32,

    #[allow(dead_code)]
    pub captured_image: Option<Handle<Image>>,
}

//capture_screen
//fade_out
//clear_captured_image

impl Default for Transition {
    fn default() -> Self {
        Self {
            progress: 0.0,
            captured_image: None,
        }
    }
}

impl Transition {
    pub fn capture_screen(
        &mut self,
        //commands: &mut Commands,
        camera_query: &Query<Entity, With<Camera2d>>,
    ) {
        // Logic to capture the screen and store it in captured_image
        // This is a placeholder implementation
        for _ in camera_query.iter() {
            // Assuming you have a method to capture the screen image
            let image_handle = Handle::<Image>::default(); // Replace with actual capture logic
            self.captured_image = Some(image_handle);
        }
    }

    pub fn fade_out(&self) {
        info!("Fade out transition {self:?}");
        // Logic to fade out the captured image
        // This is a placeholder implementation
        // if let Some(image_handle) = &self.captured_image {
        //     // Apply fade out effect to the image
        //     // This is a placeholder implementation
        // }
    }

    pub fn clear_captured_image(&mut self) {
        self.captured_image = None;
    }
}
