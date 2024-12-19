use bevy::prelude::*;

use crate::update_mechanics;

pub struct MechanicsPlugin;

impl Plugin for MechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_mechanics);
    }
}
