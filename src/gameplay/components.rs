use bevy::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Element {
    Earth,
    Fire,
    Water,
}

impl Element {
    pub fn can_attack(&self, other: &Element) -> bool {
        match self {
            Element::Earth => *other == Element::Water,
            Element::Fire => *other == Element::Earth,
            Element::Water => *other == Element::Fire,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Player {
    None,
    Player1,
    Player2,
}

#[derive(Clone, PartialEq)]
pub enum Interactable {
    None,
    Clickable,
    Active,
}

#[derive(PartialEq, Clone)]
pub struct Coordinate {
    pub row: i32,
    pub col: i32,
}

impl Coordinate {
    pub fn is_neighbor(&self, coord2: &Coordinate) -> bool {
        if self.col == coord2.col {
            return self.row == coord2.row + 1 || self.row == coord2.row - 1;
        }
        if self.row == coord2.row {
            return self.col == coord2.col + 1 || self.col == coord2.col - 1;
        }
        return false;
    }
}

#[derive(Component, Clone)]
pub struct Bounds {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Component, Clone)]
pub struct Tile {
    pub element: Element,
    pub player: Player,
    pub coord: Coordinate,
    pub interactable: Interactable,
}

#[derive(Component)]
pub struct PlayerTile;

#[derive(Resource)]
pub struct Turn {
    pub player: Player,
    pub src_tile: Option<Tile>,
}

#[derive(Event)]
pub struct AdvanceTileEvent(pub Entity);
