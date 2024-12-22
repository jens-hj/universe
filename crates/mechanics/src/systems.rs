use bevy::prelude::*;
use dynamics::Acceleration;
use std::collections::HashMap;

use crate::particle::Particle;

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
