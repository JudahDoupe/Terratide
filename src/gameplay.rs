use bevy::audio::CpalSample;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Clone)]
enum Element {
    Earth,
    Fire,
    Water,
}

#[derive(Clone)]
enum Player {
    None,
    Player1,
    Player2,
}

#[derive(PartialEq)]
struct Coordinate {
    row: i32,
    col: i32,
}

#[derive(Component)]
struct Tile {
    element: Element,
    player: Player,
    coord: Coordinate,
}

#[derive(Resource)]
struct Turn {
    player: Player,
    source: Option<Coordinate>,
    destination: Option<Coordinate>,
}

pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Turn {
            player: Player::Player1,
            source: None,
            destination: None,
        })
        .add_systems(Startup, (setup_board, setup_camera))
        .add_systems(Update, advance_tile);
    }
}

const NUM_ROWS: i32 = 9;
const NUM_COLS: i32 = 5;

fn setup_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((Camera2d { ..default() },));
}

fn setup_board(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let tile_width = window.width() / NUM_COLS as f32;
    let tile_height = window.height() / NUM_ROWS as f32;

    for col in 0..NUM_COLS {
        for row in 0..NUM_ROWS {
            let element = {
                if col == 0 || col == NUM_COLS - 1 {
                    Element::Fire
                } else if col == 1 || col == NUM_COLS - 2 {
                    Element::Earth
                } else {
                    Element::Water
                }
            };
            let player = if row < 3 {
                Player::Player1
            } else if row >= (NUM_ROWS - 3) {
                Player::Player2
            } else {
                Player::None
            };

            match player {
                Player::None => println!("none"),
                Player::Player1 => println!("plaeyer 1"),
                Player::Player2 => println!("player 2"),
            };

            commands.spawn((
                Tile {
                    element: element.clone(),
                    coord: Coordinate { row: row, col: col },
                    player: player.clone(),
                },
                Sprite {
                    image: assets.load("sprites/tile.png"),
                    color: Color::hsl(
                        match element {
                            Element::Earth => 130.0,
                            Element::Fire => 0.0,
                            Element::Water => 225.0,
                        },
                        match player {
                            Player::None => 0.2,
                            Player::Player1 => 1.0,
                            Player::Player2 => 1.0,
                        },
                        match player {
                            Player::None => 0.5,
                            Player::Player1 => 0.5,
                            Player::Player2 => 0.3,
                        },
                    ),
                    custom_size: Some(Vec2 {
                        x: tile_width,
                        y: tile_height,
                    }),
                    ..default()
                },
                Transform::from_xyz(
                    (col as f32 * tile_width) + (tile_width / 2.0) - (window.width() / 2.0),
                    (row as f32 * tile_height) + (tile_height / 2.0) - (window.height() / 2.0),
                    0.0,
                ),
            ));
        }
    }
}

fn advance_tile(player_move: Res<Turn>, mut tiles: Query<&mut Tile>) {
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
