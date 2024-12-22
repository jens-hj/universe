use bevy::prelude::*;
use bevy_blendy_cameras::BlendyCamerasPlugin;

use crate::{init_particles, setup_view, spawn_particles, toggle_debug};

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            brightness: 300.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::srgb_u8(30, 30, 46)))
        .add_plugins(BlendyCamerasPlugin)
        .add_systems(Startup, (setup_view, spawn_particles))
        .add_systems(Update, (init_particles, toggle_debug));
    }
}
