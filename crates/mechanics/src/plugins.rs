use bevy::prelude::*;

// use crate::{electromagnetic_interaction, gravity, strong_interaction};
use crate::apply_forces;

pub struct MechanicsPlugin;

impl Plugin for MechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_forces);
    }
}
