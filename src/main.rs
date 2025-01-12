use bevy::prelude::*;
use bevy::window::*;
mod gameplay;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(500., 900.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(gameplay::GameplayPlugin)
        .run();
}
