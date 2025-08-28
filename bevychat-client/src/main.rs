use bevy::prelude::*;

use crate::socials::SocialsPlugin;

mod consts;
mod module_bindings;
mod socials;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins.set(create_window_plugin()), SocialsPlugin))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .run();
}

fn create_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Chat".to_string(),
            ..default()
        }),
        ..default()
    }
}
