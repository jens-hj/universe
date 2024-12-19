use bevy::{color::palettes, prelude::*};
use bevy_rts_camera::{Ground, RtsCamera, RtsCameraControls};
use mechanics::{particle::Particle, proton::Proton};

use crate::ParticleView;

pub fn setup_view(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        RtsCamera::default(),
        RtsCameraControls::default(), // Optional
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(80.0, 80.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: palettes::basic::GREEN.into(),
            ..default()
        })),
        Ground,
    ));
}

pub fn spawn_particles(mut commands: Commands) {
    let positions = vec![Vec3::new(-50.0, 0.0, 0.0), Vec3::new(50.0, 0.0, 0.0)];

    for position in positions {
        commands.spawn((
            Particle::proton(),
            Proton,
            Transform::from_translation(position),
        ));
    }
}

pub fn init_protons(
    mut commands: Commands,
    query: Query<Entity, Added<Proton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert((
            ParticleView,
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: palettes::basic::RED.into(),
                ..default()
            })),
            Mesh3d(meshes.add(Sphere::default().mesh().ico(5).unwrap())),
        ));
    }
}
