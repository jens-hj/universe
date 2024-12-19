use bevy::prelude::*;

use crate::particle::Particle;

const GRAVITATIONAL_CONSTANT: f32 = 6.67430e-11;

pub fn update_mechanics(mut query: Query<(Entity, &mut Transform, &Particle)>, time: Res<Time>) {
    let mut changes: Vec<(Entity, Vec3)> = Vec::new();

    for [(entity_a, transform_a, particle_a), (entity_b, transform_b, particle_b)] in
        query.iter_combinations()
    {
        if particle_a.kind == particle_b.kind {
            continue;
        }

        let delta = transform_b.translation - transform_a.translation;
        let distance = delta.length();
        let direction = delta.normalize();
        let force = GRAVITATIONAL_CONSTANT * particle_a.mass * particle_b.mass / distance.powi(2);

        changes.push((entity_a, direction * force * time.delta_secs()));
        changes.push((entity_b, -direction * force * time.delta_secs()));
    }

    for (entity, change) in changes {
        if let Ok((_, mut transform, _)) = query.get_mut(entity) {
            transform.translation += change;
        }
    }
}
