use bevy::prelude::*;
use dynamics::DynamicsPlugin;

// use crate::{electromagnetic_interaction, gravity, strong_interaction};
use crate::{apply_forces, detect_atoms, spawn_atom_hitbox, write_atom_info};

pub struct MechanicsPlugin;

impl Plugin for MechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DynamicsPlugin).add_systems(
            Update,
            (
                apply_forces,
                detect_atoms,
                spawn_atom_hitbox,
                write_atom_info,
            ),
        );
    }
}
