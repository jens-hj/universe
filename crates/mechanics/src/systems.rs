use bevy::prelude::*;

use crate::particle::Particle;

// const GRAVITATIONAL_CONSTANT: f32 = 6.67430e-11;
const GRAVITATIONAL_CONSTANT: f32 = 10.0;

pub fn gravity(mut query: Query<(Entity, &mut Transform, &Particle)>, time: Res<Time>) {
    let mut changes: Vec<(Entity, Vec3)> = Vec::new();

    for [(entity_a, transform_a, particle_a), (entity_b, transform_b, particle_b)] in
        query.iter_combinations()
    {
        if entity_a == entity_b {
            continue;
        }

        let delta = transform_b.translation - transform_a.translation;
        let distance = delta.length();

        if distance < 0.5 {
            continue;
        }

        let direction = delta.normalize();
        let force = GRAVITATIONAL_CONSTANT * particle_a.mass * particle_b.mass / distance.powi(2);

        let change = direction * force * time.delta_secs();
        // make sure to clamp the change such that
        // TODO: this needs a lot more thought
        // if change.length() > distance - particle_a.radius - particle_b.radius {
        //     change = direction * (distance - particle_a.radius - particle_b.radius);
        //     info!("clamping change {:?}", change);
        // }

        changes.push((entity_a, change));
        changes.push((entity_b, -change));
    }

    for (entity, change) in changes {
        if let Ok((_, mut transform, _)) = query.get_mut(entity) {
            transform.translation += change;
        }
    }
}

// pub fn weak_interaction(mut query: Query<(Entity, &mut Transform, &Particle)>, time: Res<Time>) {
//     let mut changes: Vec<(Entity, Vec3)> = Vec::new();

//     for (entity, transform, particle) in query.iter() {
//         changes.push((entity, Vec3::ZERO));
//     }
// }

pub fn strong_interaction(mut query: Query<(Entity, &mut Transform, &Particle)>, time: Res<Time>) {
    let mut changes: Vec<(Entity, Vec3)> = Vec::new();

    // Constants for the strong force
    const STRONG_FORCE_CONSTANT: f32 = 100.0; // Much stronger than gravity
    const RANGE_CONSTANT: f32 = 2.0; // Controls how quickly the force falls off
    const EQUILIBRIUM_DISTANCE: f32 = 1.0; // Distance where attractive and repulsive forces balance
    const MAX_FORCE: f32 = 1000.0; // Add maximum force limit

    for [(entity_a, transform_a, _particle_a), (entity_b, transform_b, _particle_b)] in
        query.iter_combinations()
    {
        if entity_a == entity_b {
            continue;
        }

        let delta = transform_b.translation - transform_a.translation;
        let distance = delta.length();

        // // Skip if particles are overlapping
        // if distance < particle_a.radius + particle_b.radius {
        //     continue;
        // }

        let direction = delta.normalize();

        // Modified force equation that becomes repulsive at close range
        let force = STRONG_FORCE_CONSTANT
            * ((-distance * RANGE_CONSTANT).exp() * (distance - EQUILIBRIUM_DISTANCE));

        // Clamp the force to prevent extreme values
        let force = force.clamp(-MAX_FORCE, MAX_FORCE);

        let change = direction * force * time.delta_secs();

        // Limit the maximum position change per frame
        let max_change_per_frame = 1.0; // Adjust this value as needed
        let change = if change.length() > max_change_per_frame {
            change.normalize() * max_change_per_frame
        } else {
            change
        };

        changes.push((entity_a, change));
        changes.push((entity_b, -change));
    }

    for (entity, change) in changes {
        if let Ok((_, mut transform, _)) = query.get_mut(entity) {
            transform.translation += change;
        }
    }
}

pub fn electromagnetic_interaction(
    mut query: Query<(Entity, &mut Transform, &Particle)>,
    time: Res<Time>,
) {
    let mut changes: Vec<(Entity, Vec3)> = Vec::new();

    // Coulomb's constant (scaled for game purposes)
    const COULOMB_CONSTANT: f32 = 50.0;

    for [(entity_a, transform_a, particle_a), (entity_b, transform_b, particle_b)] in
        query.iter_combinations()
    {
        if entity_a == entity_b {
            continue;
        }

        let delta = transform_b.translation - transform_a.translation;
        let distance = delta.length();

        // // Skip if particles are overlapping
        // if distance < particle_a.radius + particle_b.radius {
        //     continue;
        // }

        let direction = delta.normalize();

        // Coulomb's law: F = k * (q1 * q2) / r²
        // Note: Unlike gravity, this can be repulsive (positive) or attractive (negative)
        let force = COULOMB_CONSTANT * particle_a.charge.value() * particle_b.charge.value()
            / distance.powi(2);

        let change = direction * force * time.delta_secs();

        changes.push((entity_a, change));
        changes.push((entity_b, -change));
    }

    for (entity, change) in changes {
        if let Ok((_, mut transform, _)) = query.get_mut(entity) {
            transform.translation += change;
        }
    }
}
