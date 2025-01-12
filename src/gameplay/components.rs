use bevy::prelude::*;

#[derive(Clone)]
pub enum Element {
    Earth,
    Fire,
    Water,
}

#[derive(Clone)]
pub enum Player {
    None,
    Player1,
    Player2,
}

#[derive(PartialEq)]
pub struct Coordinate {
    pub row: i32,
    pub col: i32,
}

#[derive(Component)]
pub struct Tile {
    pub element: Element,
    pub player: Player,
    pub coord: Coordinate,
}

#[derive(Component)]
pub struct PlayerTile;

#[derive(Resource)]
pub struct Turn {
    pub player: Player,
    pub source: Option<Coordinate>,
    pub destination: Option<Coordinate>,
}
