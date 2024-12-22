use bevy::prelude::*;
use bevy_blendy_cameras::{FlyCameraController, OrbitCameraController};
use dynamics::Debug;
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
        Transform::from_translation(Vec3::new(0.0, 10.0, 10.0)).looking_at(Vec3::ZERO, Vec3::Y),
        DirectionalLight {
            illuminance: 5000.0,
            ..default()
        },
    ));

    // spawn camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
        OrbitCameraController::default(),
        FlyCameraController {
            is_enabled: false,
            ..default()
        },
    ));
}

pub fn spawn_particles(mut commands: Commands) {
    let kinds = [Kind::Proton, Kind::Neutron, Kind::Electron];

    // generate 5 random positions for each kind in {Proton, Neutron, Electron}
    // in total 15 particles
    // in the range of [-10, 10]
    const PARTICLE_COUNT: usize = 200;
    const SPAWN_RADIUS: f32 = 500.0;
    let particles = (0..PARTICLE_COUNT).map(|_| {
        (
            Vec3::new(
                rand::random::<f32>() * SPAWN_RADIUS - SPAWN_RADIUS / 2.0,
                rand::random::<f32>() * SPAWN_RADIUS - SPAWN_RADIUS / 2.0,
                rand::random::<f32>() * SPAWN_RADIUS - SPAWN_RADIUS / 2.0,
            ),
            // randomly choose one of the three kinds
            kinds[rand::random::<usize>() % kinds.len()],
        )
    });

    for (position, kind) in particles {
        let particle = match kind {
            Kind::Proton => Particle::proton(),
            Kind::Neutron => Particle::neutron(),
            Kind::Electron => Particle::electron(),
            _ => unreachable!(),
        };
        let particle_entity = commands
            .spawn((particle, Transform::from_translation(position)))
            .id();

        if [Kind::Proton, Kind::Neutron].contains(&kind) {
            commands.entity(particle_entity).insert(Debug(false));
        }
    }

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
                base_color: particle.get_color(),
                ..default()
            })),
            Mesh3d(meshes.add(Sphere::new(particle.radius).mesh().ico(10).unwrap())),
        ));
    }
}

// system that toggles debug on pressing U
pub fn toggle_debug(mut query: Query<&mut Debug>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyU) {
        for mut debug in query.iter_mut() {
            debug.0 = !debug.0;
        }
    }
}
