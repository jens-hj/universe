use bevy::prelude::*;
use bevy_dynamics::DynamicsPlugin;

// use crate::{electromagnetic_interaction, gravity, strong_interaction};
use crate::{
    apply_forces, detect_atoms, maintain_atom_hitbox_colour, spawn_atom_hitbox,
};

pub struct MechanicsPlugin;

impl Plugin for MechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DynamicsPlugin).add_systems(
            Update,
            (
                apply_forces,
                detect_atoms,
                spawn_atom_hitbox,
                maintain_atom_hitbox_colour,
                // write_atom_info,
            ),
        );
    }
}
