use bevy::prelude::*;

use crate::{electromagnetic_interaction, gravity, strong_interaction};

pub struct MechanicsPlugin;

impl Plugin for MechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (gravity, electromagnetic_interaction, strong_interaction),
        );
    }
}
