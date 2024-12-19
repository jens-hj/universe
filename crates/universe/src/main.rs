use bevy::prelude::*;
use mechanics::MechanicsPlugin;
use view::ViewPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, MechanicsPlugin, ViewPlugin));

    app.run();
}
