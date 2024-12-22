use bevy::prelude::*;

use crate::{Acceleration, Debug, Velocity};

pub fn apply_acceleration(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.apply_acceleration(acceleration, time.delta_secs());
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_secs();
    }
}

pub fn debug_acceleration(mut gizmos: Gizmos, query: Query<(&Transform, &Acceleration, &Debug)>) {
    for (transform, acceleration, debug) in query.iter() {
        if !debug.0 {
            continue;
        }
        draw_gizmo(
            &mut gizmos,
            transform,
            acceleration.value,
            Color::srgb_u8(255, 0, 0),
        );
    }
}

pub fn debug_velocity(mut gizmos: Gizmos, query: Query<(&Transform, &Velocity, &Debug)>) {
    for (transform, velocity, debug) in query.iter() {
        if !debug.0 {
            continue;
        }
        draw_gizmo(
            &mut gizmos,
            transform,
            velocity.value,
            Color::srgb_u8(0, 255, 0),
        );
    }
}

fn draw_gizmo(gizmos: &mut Gizmos, transform: &Transform, change: Vec3, color: Color) {
    gizmos.line(
        transform.translation,
        transform.translation + change * 10.0,
        color,
    );
}
