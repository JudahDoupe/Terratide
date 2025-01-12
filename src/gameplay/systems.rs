use super::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

//Setup

const NUM_ROWS: i32 = 8;
const NUM_COLS: i32 = 6;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d { ..default() },));
}

pub fn setup_tiles(mut commands: Commands, assets: Res<AssetServer>) {
    for col in 0..NUM_COLS {
        for row in 0..NUM_ROWS {
            commands
                .spawn((
                    Tile {
                        coord: Coordinate { row: row, col: col },
                        element: rand::random(),
                        player: match row {
                            r if r < NUM_ROWS / 2 => Player::Player1,
                            r if r >= NUM_ROWS / 2 => Player::Player2,
                            _ => Player::None,
                        },
                        interactable: Interactable::None,
                    },
                    Bounds {
                        top: 0.0,
                        bottom: 0.0,
                        left: 0.0,
                        right: 0.0,
                    },
                    Sprite {
                        image: assets.load("sprites/elementTile.png"),
                        ..default()
                    },
                ))
                .with_children(|parent| {
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
    mut tile_query: Query<(&mut Sprite, &mut Transform, &mut Bounds, &Tile)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let tile_size = tile_size(window_query);

    for (mut sprite, mut transform, mut bounds, tile) in tile_query.iter_mut() {
        sprite.color = match tile.element {
            Element::Fire => Color::hsl(22.0, 0.89, 0.44),
            Element::Earth => Color::hsl(85.0, 0.23, 0.48),
            Element::Water => Color::hsl(175.0, 1.0, 0.25),
        };
        sprite.custom_size = Some(
            Vec2::new(tile_size, tile_size)
                * match tile.interactable {
                    Interactable::None => 0.8,
                    Interactable::Clickable => 1.0,
                    Interactable::Active => 0.6,
                },
        );
        transform.translation = tile_position(&tile.coord, tile_size, 1.0);
        bounds.top = transform.translation.y + tile_size / 2.0;
        bounds.bottom = transform.translation.y - tile_size / 2.0;
        bounds.left = transform.translation.x - tile_size / 2.0;
        bounds.right = transform.translation.x + tile_size / 2.0;
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

pub fn update_tile(turn: Res<Turn>, mut q_tiles: Query<&mut Tile>) {
    for mut dst in q_tiles.iter_mut() {
        dst.interactable = match &turn.src_tile {
            Some(src)
                if src.coord.is_neighbor(&dst.coord) && src.element.can_attack(&dst.element) =>
            {
                Interactable::Clickable
            }
            Some(src) if src.coord == dst.coord => Interactable::Active,
            None if turn.player == dst.player => Interactable::Clickable,
            _ => Interactable::None,
        };
    }
}

pub fn advance_tile(
    mut ev_advance_tile: EventReader<AdvanceTileEvent>,
    mut q_tiles: Query<&mut Tile>,
    mut turn: ResMut<Turn>,
) {
    for ev in ev_advance_tile.read() {
        if let Ok(mut tile) = q_tiles.get_mut(ev.0) {
            if let Some(src) = &turn.src_tile {
                tile.element = src.element.clone();
                tile.player = src.player.clone();
            }
            turn.player = match turn.player {
                Player::None => Player::None,
                Player::Player1 => Player::Player2,
                Player::Player2 => Player::Player1,
            };
            turn.src_tile = None;
        }
    }
}

pub fn read_input(
    buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut turn: ResMut<Turn>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<&Transform, With<Camera2d>>,
    q_tiles: Query<(Entity, &Bounds, &Tile)>,
    mut ev_advance_tile: EventWriter<AdvanceTileEvent>,
) {
    let window = q_windows.single();
    let camera = q_camera.single();
    let mut pos = None;

    for finger in touches.iter() {
        if touches.just_released(finger.id()) {
            pos = Some(window_to_world(finger.position(), &window, &camera));
        }
    }
    if buttons.just_released(MouseButton::Left) {
        if let Some(p) = window.cursor_position() {
            pos = Some(window_to_world(p, &window, &camera));
        }
    }

    if let Some(p) = pos {
        for (entity, bounds, tile) in q_tiles.iter() {
            if tile.interactable != Interactable::None
                && bounds.left < p.x
                && p.x < bounds.right
                && bounds.bottom < p.y
                && p.y < bounds.top
            {
                match turn.src_tile.clone() {
                    None => turn.src_tile = Some(tile.clone()),
                    Some(t) if t.coord == tile.coord => turn.src_tile = None,
                    _ => {
                        ev_advance_tile.send(AdvanceTileEvent(entity));
                        ()
                    }
                }
            }
        }
    }
}

fn window_to_world(position: Vec2, window: &Window, camera: &Transform) -> Vec2 {
    let norm = Vec3::new(
        position.x - window.width() / 2.,
        -(position.y - window.height() / 2.),
        0.0,
    );
    (*camera * norm).xy()
}
