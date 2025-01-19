use bevy::prelude::*;
use bevy_dynamics::Acceleration;
use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::element::Element;
use crate::particle::Particle;
use crate::{Atom, AtomHitbox, Kind};

const GRAVITATIONAL_CONSTANT: f32 = 50000.0;
const COULOMB_CONSTANT: f32 = 69000.0;
const STRONG_FORCE_CONSTANT: f32 = 1000000.0;
const RANGE_CONSTANT: f32 = 2.0;
const EQUILIBRIUM_DISTANCE: f32 = 5.0;
// const MAX_FORCE: f32 = 1000.0;

// Helper struct to store accumulated forces
#[derive(Default)]
struct ForceAccumulator {
    gravity: Vec3,
    electromagnetic: Vec3,
    strong: Vec3,
}

pub fn apply_forces(
    mut query: Query<(Entity, &Transform, &mut Acceleration, &Particle)>,
    time: Res<Time>,
) {
    let mut force_map: HashMap<Entity, ForceAccumulator> = HashMap::new();

    // Calculate gravitational forces
    for [(entity_a, transform_a, _acceleration_a, particle_a), (entity_b, transform_b, _acceleration_b, particle_b)] in
        query.iter_combinations()
    {
        let delta = transform_b.translation - transform_a.translation;
        let distance = delta.length();
        let direction = delta.normalize();

        // Gravitational force
        let grav_force =
            GRAVITATIONAL_CONSTANT * particle_a.mass * particle_b.mass
                / distance.powi(2);
        let grav_change = direction * grav_force;

        // Electromagnetic force
        let em_force = -COULOMB_CONSTANT
            * particle_a.charge.charge()
            * particle_b.charge.charge()
            / distance.powi(2);
        let em_change = direction * em_force;

        // Strong force
        let force = (-distance * RANGE_CONSTANT).exp()
            * (distance - EQUILIBRIUM_DISTANCE);
        let strong_force = if distance < EQUILIBRIUM_DISTANCE {
            // Apply damping factor only during repulsion (when particles are too close)
            STRONG_FORCE_CONSTANT * force * 1.0 // Adjust this damping factor as needed
        } else {
            // Normal attractive force when particles are further than equilibrium
            STRONG_FORCE_CONSTANT * force
        };
        let strong_change = direction * strong_force;

        // Accumulate forces for both particles
        force_map.entry(entity_a).or_default().gravity += grav_change;
        force_map.entry(entity_b).or_default().gravity -= grav_change;

        force_map.entry(entity_a).or_default().electromagnetic += em_change;
        force_map.entry(entity_b).or_default().electromagnetic -= em_change;

        force_map.entry(entity_a).or_default().strong += strong_change;
        force_map.entry(entity_b).or_default().strong -= strong_change;
    }

    // Apply accumulated forces
    for (entity, _transform, mut acceleration, particle) in query.iter_mut() {
        if let Some(forces) = force_map.get(&entity) {
            let total_force =
                forces.gravity + forces.electromagnetic + forces.strong;
            let change = total_force * time.delta_secs();

            // TODO: Handle this better by applying acceleration from the force -> then resulting in a velocity
            // Apply maximum change limit
            let max_change_per_frame = 10.0;
            let change = if change.length() > max_change_per_frame {
                change.normalize() * max_change_per_frame
            } else {
                change
            };

            acceleration.value = change / particle.mass;
        }
    }
}

const NUCLEUS_FORMATION_DISTANCE: f32 = 12.0;

pub fn detect_atoms(
    mut commands: Commands,
    particle_query: Query<(Entity, &Transform, &Particle)>,
    mut atoms: Query<
        (Entity, &mut Atom, &mut Transform, &mut AtomHitbox),
        Without<Particle>,
    >,
) {
    // Track which atoms are still valid
    let mut active_atoms = std::collections::HashSet::new();

    // First, find nuclei (clusters of protons and neutrons)
    let mut potential_nuclei: Vec<Vec<(Entity, Vec3, &Particle)>> = Vec::new();

    // Group nearby protons and neutrons
    for (entity_a, transform_a, particle_a) in particle_query.iter() {
        if !matches!(particle_a.kind, Kind::Proton | Kind::Neutron) {
            continue;
        }

        let pos_a = transform_a.translation;
        let mut found_cluster = false;

        // Try to add to existing cluster
        for cluster in &mut potential_nuclei {
            let cluster_center = cluster
                .iter()
                .map(|(_, pos, _)| *pos)
                .reduce(|a, b| a + b)
                .unwrap()
                / cluster.len() as f32;

            if (pos_a - cluster_center).length() < NUCLEUS_FORMATION_DISTANCE {
                cluster.push((entity_a, pos_a, particle_a));
                found_cluster = true;
                break;
            }
        }

        // Create new cluster if needed
        if !found_cluster {
            potential_nuclei.push(vec![(entity_a, pos_a, particle_a)]);
        }
    }

    // For each potential nucleus, update or create atoms
    for nucleus in potential_nuclei {
        if nucleus.len() < 2 {
            continue;
        }

        let nucleus_center = nucleus
            .iter()
            .map(|(_, pos, _)| *pos)
            .reduce(|a, b| a + b)
            .unwrap()
            / nucleus.len() as f32;

        let proton_count = nucleus
            .iter()
            .filter(|(_, _, p)| p.kind == Kind::Proton)
            .count() as u32;

        if proton_count == 0 || proton_count > Element::iter().count() as u32 {
            info!(
                "Invalid proton count: {} > {}",
                proton_count,
                Element::iter().count()
            );
            continue;
        }

        let neutron_count = nucleus
            .iter()
            .filter(|(_, _, p)| p.kind == Kind::Neutron)
            .count() as u32;

        let constituent_particles: Vec<Entity> =
            nucleus.iter().map(|(e, _, _)| *e).collect();

        // Try to find an existing atom with overlapping constituents
        let mut updated_existing = false;
        let mut should_be_selected = false;

        // First check if any of the constituent particles were part of a selected atom
        for (_, atom, _, hitbox) in atoms.iter() {
            if hitbox.selected
                && atom
                    .constituent_particles
                    .iter()
                    .any(|p| constituent_particles.contains(p))
            {
                should_be_selected = true;
                break;
            }
        }

        // Now update or create the atom with the correct selected state
        for (atom_entity, mut atom, mut transform, mut hitbox) in
            atoms.iter_mut()
        {
            if atom
                .constituent_particles
                .iter()
                .any(|p| constituent_particles.contains(p))
            {
                // Update existing atom
                if let Ok(new_atom) = Atom::new(
                    proton_count,
                    neutron_count,
                    0,
                    constituent_particles.clone(),
                ) {
                    *atom = new_atom;
                    transform.translation = nucleus_center;
                    transform.scale = Vec3::splat(atom.radius());
                    hitbox.selected = should_be_selected; // Set the correct selected state
                    active_atoms.insert(atom_entity);
                    updated_existing = true;
                    break;
                }
            }
        }

        // Create new atom if no existing one was updated
        if !updated_existing {
            if let Ok(atom) =
                Atom::new(proton_count, neutron_count, 0, constituent_particles)
            {
                let transform = Transform::from_translation(nucleus_center)
                    .with_scale(Vec3::splat(atom.radius()));
                let entity = commands
                    .spawn((
                        atom,
                        transform,
                        AtomHitbox {
                            selected: should_be_selected,
                        }, // Set the correct selected state
                    ))
                    .id();
                active_atoms.insert(entity);
            }
        }
    }

    // Remove atoms that are no longer valid
    for (entity, _atom, _transform, _hitbox) in atoms.iter() {
        if !active_atoms.contains(&entity) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn maintain_atom_hitbox_colour(
    mut query: Query<
        (&Atom, &mut MeshMaterial3d<StandardMaterial>, &AtomHitbox),
        Changed<Atom>,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (atom, material, hitbox) in query.iter_mut() {
        if let Some(material) = materials.get_mut(&material.0) {
            let alpha = if hitbox.selected {
                50.0 / 255.0
            } else {
                20.0 / 255.0
            };
            material.base_color = atom.element.color().with_alpha(alpha);
        }
    }
}

pub fn spawn_atom_hitbox(
    mut commands: Commands,
    query: Query<(Entity, &Atom, &AtomHitbox), Added<Atom>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, atom, hitbox) in query.iter() {
        commands
            .entity(entity)
            .insert((
                Mesh3d(meshes.add(Sphere::new(1.0).mesh().ico(10).unwrap())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: match hitbox.selected {
                        true => atom.element.color().with_alpha(50.0 / 255.0),
                        false => atom.element.color().with_alpha(20.0 / 255.0),
                    },
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                })),
            ))
            .observe(over_atom)
            .observe(out_atom)
            .observe(click_atom);
    }
}

fn over_atom(
    trigger: Trigger<Pointer<Over>>,
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &AtomHitbox)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok((material, hitbox)) = query.get_mut(trigger.entity()) {
        if hitbox.selected {
            return;
        }

        if let Some(material) = materials.get_mut(&material.0) {
            material.base_color.set_alpha(50.0 / 255.0);
        }
    }
}

fn out_atom(
    trigger: Trigger<Pointer<Out>>,
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &AtomHitbox)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok((material, hitbox)) = query.get_mut(trigger.entity()) {
        if hitbox.selected {
            return;
        }

        if let Some(material) = materials.get_mut(&material.0) {
            material.base_color.set_alpha(20.0 / 255.0);
        }
    }
}

fn click_atom(
    trigger: Trigger<Pointer<Click>>,
    mut query: Query<(
        Entity,
        &mut MeshMaterial3d<StandardMaterial>,
        &mut AtomHitbox,
        &Atom,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // only if left click
    if trigger.button != PointerButton::Primary {
        return;
    }

    if let Ok((entity, material_handle, mut hitbox, atom)) =
        query.get_mut(trigger.entity())
    {
        hitbox.selected = !hitbox.selected;

        if let Some(material) = materials.get_mut(&material_handle.0) {
            match hitbox.selected {
                true => {
                    material.base_color =
                        atom.element.color().with_alpha(50.0 / 255.0);
                    // deselect all other atoms
                    for (
                        other_entity,
                        other_material_handle,
                        mut other_hitbox,
                        other_atom,
                    ) in query.iter_mut()
                    {
                        if other_entity == entity {
                            continue;
                        }
                        if other_hitbox.selected {
                            other_hitbox.selected = false;
                            if let Some(material) =
                                materials.get_mut(&other_material_handle.0)
                            {
                                material.base_color = other_atom
                                    .element
                                    .color()
                                    .with_alpha(20.0 / 255.0);
                            }
                        }
                    }
                }
                false => {
                    material.base_color =
                        atom.element.color().with_alpha(20.0 / 255.0);
                }
            }
        }
    }
}

// system that writes information about the currently selected atom to the UI
pub fn write_atom_info(query: Query<(&Atom, &AtomHitbox)>) {
    for (atom, hitbox) in query.iter() {
        if hitbox.selected {
            info!("Selected atom: {}", atom);
        }
    }
}
