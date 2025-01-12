use bevy::prelude::*;
use components::*;
use systems::*;

pub mod components;
pub mod systems;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Turn {
            player: Player::Player1,
            source: None,
            destination: None,
        })
        .insert_resource(ClearColor(Color::hsl(37.0, 0.65, 0.68)))
        .add_systems(Startup, (setup_tiles, setup_camera))
        .add_systems(Update, advance_tile)
        .add_systems(
            Update,
            (
                systems::update_tile_sprite,
                systems::update_player_tile_sprite,
            )
                .chain(),
        );
    }
}
