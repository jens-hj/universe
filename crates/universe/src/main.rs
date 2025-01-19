use bevy::prelude::*;
use hud::HudPlugin;
use mechanics::MechanicsPlugin;
use view::ViewPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(Time::<Fixed>::from_hz(100.0));

    // Bevy plugins
    app.add_plugins((DefaultPlugins, MeshPickingPlugin));

    // Internal plugins
    app.add_plugins((MechanicsPlugin, ViewPlugin, HudPlugin));

    app.run();
}
