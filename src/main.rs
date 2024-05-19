use bevy::prelude::*;
fn main() {
    println!("Hello, world!");
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (add_hero, show_sprite))
    .add_systems(Update, (yoyo, (update_hero, greet_people).chain()))
    .run();
}

fn yoyo() {
    println!("yoyoyo");
}

#[derive(Component)]
struct SuperHero;

#[derive(Component)]
struct Name(String);

fn add_hero(mut commands: Commands) {
    commands.spawn((SuperHero, Name("Bat Man".to_string())));    
    commands.spawn((SuperHero, Name("Super Man".to_string())));    
    commands.spawn((SuperHero, Name("ShaktiMaan".to_string())));    
}

fn greet_people(query: Query<&Name, With<SuperHero>>) {
    for name in &query {
        println!("Yo {}!", name.0);
    }
}

fn update_hero(mut query: Query<&mut Name, With<SuperHero>>) {
    for mut name in &mut query {
        if name.0 == "ShaktiMaan" {
            name.0 = "GangaDhar".to_string();
            break; // We don’t need to change any other names
        }
        else if name.0 == "Super Man" {
            name.0 = "Clark Kent".to_string();
            break;
        }
    }
}

fn show_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("../assets/abc.png"),
        ..default()
    });
}