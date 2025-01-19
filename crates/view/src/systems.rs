use bevy::prelude::*;
use bevy_blendy_cameras::{FlyCameraController, OrbitCameraController};
use bevy_dynamics::{Damping, Debug};
use mechanics::particle::{GetColor, Kind, Particle};

use crate::ParticleView;

pub fn setup_view(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn lighting
    // - directional light
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 10.0, 10.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        DirectionalLight {
            illuminance: 5000.0,
            ..default()
        },
    ));

    // spawn camera
    commands.spawn((
        Camera {
            order: 0,
            ..default()
        },
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 100.0, 0.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        OrbitCameraController::default(),
        FlyCameraController {
            is_enabled: false,
            ..default()
        },
    ));
}

pub fn spawn_particles(mut commands: Commands) {
    const SPAWN_RADIUS: f32 = 300.0;

    // Define count for each particle type
    let particle_counts = [
        (Kind::Proton, 118),
        (Kind::Neutron, 157),
        // (Kind::Electron, 50),  // Commented out as per original
    ];

    // Spawn particles for each kind
    for (kind, count) in particle_counts {
        for _ in 0..count {
            let position = Vec3::new(
                rand::random::<f32>() * SPAWN_RADIUS - SPAWN_RADIUS / 2.0,
                rand::random::<f32>() * SPAWN_RADIUS - SPAWN_RADIUS / 2.0,
                rand::random::<f32>() * SPAWN_RADIUS - SPAWN_RADIUS / 2.0,
            );

            let particle = match kind {
                Kind::Proton => Particle::proton(),
                Kind::Neutron => Particle::neutron(),
                Kind::Electron => Particle::electron(),
                _ => unreachable!(),
            };
            let particle_entity = commands
                .spawn((
                    particle,
                    Transform::from_translation(position),
                    Damping::new(2.0),
                ))
                .id();

            if [Kind::Proton, Kind::Neutron].contains(&kind) {
                commands.entity(particle_entity).insert(Debug {
                    acceleration: false,
                    velocity: false,
                });
            }
        }
    }

    // // let kinds = [Kind::Proton, Kind::Neutron, Kind::Electron];
    // let kinds = [Kind::Proton, Kind::Neutron];

    // // generate 5 random positions for each kind in {Proton, Neutron, Electron}
    // // in total 15 particles
    // // in the range of [-10, 10]
    // const PARTICLE_COUNT: usize = 225;
    // const SPAWN_RADIUS: f32 = 200.0;
    // let particles = (0..PARTICLE_COUNT).map(|_| {
    //     (
    //         Vec3::new(
    //             rand::random::<f32>() * SPAWN_RADIUS - SPAWN_RADIUS / 2.0,
    //             rand::random::<f32>() * SPAWN_RADIUS - SPAWN_RADIUS / 2.0,
    //             rand::random::<f32>() * SPAWN_RADIUS - SPAWN_RADIUS / 2.0,
    //         ),
    //         // randomly choose one of the three kinds
    //         kinds[rand::random::<usize>() % kinds.len()],
    //     )
    // });

    // for (position, kind) in particles {
    //     let particle = match kind {
    //         Kind::Proton => Particle::proton(),
    //         Kind::Neutron => Particle::neutron(),
    //         Kind::Electron => Particle::electron(),
    //         _ => unreachable!(),
    //     };
    //     let particle_entity = commands
    //         .spawn((particle, Transform::from_translation(position)))
    //         .id();

    //     if [Kind::Proton, Kind::Neutron].contains(&kind) {
    //         commands.entity(particle_entity).insert(Debug(false));
    //     }
    // }

    // let particles = [
    //     (Vec3::new(5.0, 5.0, 5.0), Kind::Proton),
    //     (Vec3::new(-5.0, 5.0, -5.0), Kind::Proton),
    //     (Vec3::new(5.0, -5.0, -5.0), Kind::Proton),
    //     (Vec3::new(-5.0, -5.0, 5.0), Kind::Proton),
    //     (Vec3::new(-5.0, 5.0, 5.0), Kind::Neutron),
    //     (Vec3::new(5.0, 5.0, -5.0), Kind::Neutron),
    //     (Vec3::new(5.0, -5.0, 5.0), Kind::Neutron),
    //     (Vec3::new(-5.0, -5.0, -5.0), Kind::Neutron),
    //     (Vec3::new(10.0, -10.0, 10.0), Kind::Electron),
    //     (Vec3::new(-10.0, -10.0, -10.0), Kind::Electron),
    //     (Vec3::new(10.0, 10.0, -10.0), Kind::Electron),
    //     (Vec3::new(-10.0, 10.0, 10.0), Kind::Electron),
    // ];

    // for (position, kind) in particles {
    //     let particle = match kind {
    //         Kind::Proton => Particle::proton(),
    //         Kind::Neutron => Particle::neutron(),
    //         Kind::Electron => Particle::electron(),
    //         _ => unreachable!(),
    //     };
    //     commands.spawn((particle, Transform::from_translation(position)));
    // }
}

pub fn init_particles(
    mut commands: Commands,
    query: Query<(Entity, &Particle), Added<Particle>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, particle) in query.iter() {
        commands.entity(entity).insert((
            ParticleView,
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: particle.get_color().with_alpha(0.5),
                ..default()
            })),
            Mesh3d(
                meshes
                    .add(Sphere::new(particle.radius).mesh().ico(10).unwrap()),
            ),
        ));
    }
}

// system that toggles debug on pressing U
pub fn toggle_debug(
    mut query: Query<&mut Debug>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyU) {
        for mut debug in query.iter_mut() {
            debug.acceleration = !debug.acceleration;
            debug.velocity = !debug.velocity;
        }
    }
}
