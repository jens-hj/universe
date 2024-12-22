use bevy::prelude::*;

pub fn update_hud(mut query: Query<&mut Text>) {
    for mut text in query.iter_mut() {
        // text.sections[0].value = "Hello, world!".to_string();
    }
}
