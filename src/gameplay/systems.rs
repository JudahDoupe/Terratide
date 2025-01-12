use super::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

//Setup

const NUM_ROWS: i32 = 8;
const NUM_COLS: i32 = 6;

pub fn setup_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    commands.spawn((Camera2d { ..default() },));
}

pub fn setup_tiles(mut commands: Commands, assets: Res<AssetServer>) {
    for col in 0..NUM_COLS {
        for row in 0..NUM_ROWS {
            commands
                .spawn((
                    Tile {
                        coord: Coordinate { row: row, col: col },
                        element: if col == 0 || col == NUM_COLS - 1 {
                            Element::Fire
                        } else if col == 1 || col == NUM_COLS - 2 {
                            Element::Earth
                        } else {
                            Element::Water
                        },
                        player: if row < 3 {
                            Player::Player1
                        } else if row >= (NUM_ROWS - 3) {
                            Player::Player2
                        } else {
                            Player::None
                        },
                    },
                    Sprite {
                        image: assets.load("sprites/elementTile.png"),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Create the child entity (player)
                    parent.spawn((
                        PlayerTile,
                        Sprite {
                            image: assets.load("sprites/playerTile.png"),
                            ..default()
                        },
                    ));
                });
        }
    }
}

//Visualization

pub fn update_tile_sprite(
    mut tile_query: Query<(&mut Sprite, &mut Transform, &Tile)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let tile_size = tile_size(window_query);

    for (mut sprite, mut transform, tile) in tile_query.iter_mut() {
        sprite.color = match tile.element {
            Element::Fire => Color::hsl(22.0, 0.89, 0.44),
            Element::Earth => Color::hsl(85.0, 0.23, 0.48),
            Element::Water => Color::hsl(175.0, 1.0, 0.25),
        };
        sprite.custom_size = Some(Vec2::new(tile_size, tile_size));
        transform.translation = tile_position(&tile.coord, tile_size, 1.0);
    }
}

pub fn update_player_tile_sprite(
    mut player_tile_query: Query<(&mut Sprite, &mut Transform, &Parent), With<PlayerTile>>,
    parent_tile_query: Query<&Tile>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let tile_size = tile_size(window_query);

    for (mut sprite, mut transform, parent) in player_tile_query.iter_mut() {
        if let Ok(tile) = parent_tile_query.get(parent.get()) {
            sprite.color = match tile.player {
                Player::Player1 => Color::hsl(37.0, 0.62, 0.96),
                Player::Player2 => Color::hsl(0.0, 0.0, 0.2),
                Player::None => Color::hsla(0.0, 0.0, 0.0, 0.0),
            };
            sprite.custom_size = Some(Vec2::new(tile_size, tile_size));
            transform.translation = Vec3::new(0.0, 0.0, -1.0);
        }
    }
}

fn tile_position(coord: &Coordinate, tile_size: f32, z: f32) -> Vec3 {
    let tile_offset = tile_size / 2.0;
    let h_offset = tile_offset - tile_size * (NUM_COLS as f32 / 2.0);
    let v_offset = tile_offset - tile_size * (NUM_ROWS as f32 / 2.0);
    Vec3::new(
        (coord.col as f32 * tile_size) + h_offset,
        (coord.row as f32 * tile_size) + v_offset,
        z,
    )
}

fn tile_size(window_query: Query<&Window, With<PrimaryWindow>>) -> f32 {
    let window = window_query.get_single().unwrap();
    let tile_width = window.width() / NUM_COLS as f32;
    let tile_height = window.height() / NUM_ROWS as f32;
    tile_width.min(tile_height)
}

// Gameplay

pub fn advance_tile(player_move: Res<Turn>, mut tiles: Query<&mut Tile>) {
    match (&player_move.source, &player_move.destination) {
        (Some(src), Some(dst)) => {
            let mut src_element = Element::Water;
            let mut src_player = Player::None;
            for tile in &mut tiles {
                if tile.coord == *src {
                    src_element = tile.element.clone();
                    src_player = tile.player.clone();
                }
            }
            for mut tile in &mut tiles {
                if tile.coord == *dst {
                    tile.element = src_element.clone();
                    tile.player = src_player.clone();
                }
            }
        }
        _ => {}
    };
}
