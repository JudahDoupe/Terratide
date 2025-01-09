use bevy::prelude::*;

fn main() {
    App::new()
    .add_systems(Startup, setup_board)
    .add_systems(Update, (sanity, count_tiles))
    .run();
}

fn sanity() {
    println!("running");
}

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Element(String);

fn setup_board(mut commands: Commands){
    commands.spawn((Tile, Element("Fire".to_string())));
    commands.spawn((Tile, Element("Earth".to_string())));
    commands.spawn((Tile, Element("Water".to_string())));
    commands.spawn((Tile, Element("Fire".to_string())));
}

fn count_tiles(query: Query<&Element, With<Tile>>){
    for element in &query{
        println!("tile is {}", element.0);
    }
}