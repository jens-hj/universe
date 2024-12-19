use bevy::prelude::*;
use bevy_rts_camera::RtsCameraPlugin;

use crate::{init_protons, setup_view};

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RtsCameraPlugin)
            .add_systems(Startup, setup_view)
            .add_systems(Update, init_protons);
    }
}
