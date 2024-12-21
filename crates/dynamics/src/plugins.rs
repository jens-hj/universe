use bevy::prelude::*;

use crate::{apply_acceleration, apply_velocity};

pub struct DynamicsPlugin;

impl Plugin for DynamicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_acceleration, apply_velocity));
    }
}
