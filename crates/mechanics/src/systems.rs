use bevy::prelude::*;
use dynamics::Acceleration;
use std::collections::HashMap;

use crate::particle::Particle;
use crate::{element, Atom, AtomHitbox, Kind};

const GRAVITATIONAL_CONSTANT: f32 = 50000.0;
const COULOMB_CONSTANT: f32 = 70000.0;
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

        // if distance < 0.5 {
        //     continue;
        // }

        let direction = delta.normalize();

        // Gravitational force
        let grav_force =
            GRAVITATIONAL_CONSTANT * particle_a.mass * particle_b.mass / distance.powi(2);
        let grav_change = direction * grav_force;
        // info!(
        //     "[{:?}, {:?}] grav_change: {} with force {}",
        //     particle_a.kind, particle_b.kind, grav_change, grav_force
        // );

        // Electromagnetic force
        let em_force = -COULOMB_CONSTANT * particle_a.charge.charge() * particle_b.charge.charge()
            / distance.powi(2);
        let em_change = direction * em_force;
        // info!(
        //     "[{:?}, {:?}] em_change: {} with force {}",
        //     particle_a.kind, particle_b.kind, em_change, em_force
        // );

        // Strong force
        let force = (-distance * RANGE_CONSTANT).exp() * (distance - EQUILIBRIUM_DISTANCE);
        let strong_force = if distance < EQUILIBRIUM_DISTANCE {
            // Apply damping factor only during repulsion (when particles are too close)
            STRONG_FORCE_CONSTANT * force * 1.0 // Adjust this damping factor as needed
        } else {
            // Normal attractive force when particles are further than equilibrium
            STRONG_FORCE_CONSTANT * force
        };
        let strong_change = direction * strong_force;
        // info!(
        //     "[{:?}, {:?}] strong_change: {} with force {}",
        //     particle_a.kind, particle_b.kind, strong_change, strong_force
        // );

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
            let total_force = forces.gravity + forces.electromagnetic + forces.strong;
            let change = total_force * time.delta_secs();

            // TODO: Handle this better by applying acceleration from the force -> then resulting in a velocity
            // Apply maximum change limit
            let max_change_per_frame = 10.0;
            let change = if change.length() > max_change_per_frame {
                change.normalize() * max_change_per_frame
            } else {
                change
            };

            // Force = mass * acceleration => acceleration = force / mass
            acceleration.value = change / particle.mass;
            // info!("acceleration: {}", acceleration.value);
            // transform.translation += final_change;
            // transform.translation += change;
        }
    }
}

pub fn detect_atoms(
    mut commands: Commands,
    particle_query: Query<(Entity, &Transform, &Particle)>,
    mut atoms: Query<(Entity, &mut Atom)>,
) {
    // temporarily clear atoms
    for (entity, atom) in atoms.iter_mut() {
        info!("{}", *atom);
        commands.entity(entity).despawn_recursive();
    }

    // Constants for atom detection
    const NUCLEUS_FORMATION_DISTANCE: f32 = 8.0; // Distance for protons/neutrons to form nucleus

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

    // For each potential nucleus, look for orbiting electrons
    for nucleus in potential_nuclei {
        if nucleus.len() < 2 {
            // Require at least 2 nucleons
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

        if proton_count == 0 || proton_count > element::MAX_ATOMIC_NUMBER as u32 {
            continue;
        }

        let neutron_count = nucleus
            .iter()
            .filter(|(_, _, p)| p.kind == Kind::Neutron)
            .count() as u32;

        let constituent_particles = nucleus.iter().map(|(e, _, _)| *e).collect::<Vec<_>>();
        // constituent_particles.extend(orbiting_electrons.iter());
        let atom = match Atom::new(
            nucleus_center,
            proton_count,
            neutron_count,
            0,
            constituent_particles,
        ) {
            Ok(atom) => atom,
            Err(e) => {
                error!("Failed to create atom: {}", e);
                continue;
            }
        };
        commands.spawn(atom);
        // }
    }
}

pub fn atom_hitbox(
    mut commands: Commands,
    query: Query<&Atom, Added<Atom>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_hitbox: Query<Entity, With<AtomHitbox>>,
) {
    // clear all previous hitboxes
    for entity in query_hitbox.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for atom in query.iter() {
        commands.spawn((
            Transform::from_translation(atom.nucleus_center),
            Mesh3d(meshes.add(Sphere::new(atom.radius()).mesh().ico(10).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba_u8(205, 214, 244, 20),
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
            AtomHitbox,
        ));
    }
}
