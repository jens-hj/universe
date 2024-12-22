use bevy::prelude::*;
use dynamics::DynamicsPlugin;

// use crate::{electromagnetic_interaction, gravity, strong_interaction};
use crate::apply_forces;

pub struct MechanicsPlugin;

impl Plugin for MechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DynamicsPlugin)
            .add_systems(Update, apply_forces);
    }
}
