use bevy::prelude::*;

use super::{
    state_component::{GameState, Transition},
    state_system::{transition_in_system, transition_out_system},
};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, (transition_out_system, transition_in_system))
            .insert_resource(Transition::default());
    }
}
