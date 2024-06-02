use bevy::prelude::*;
#[derive(Component)]
struct Movable;

const SYM_HEIGHT: f32 = 122.;
const SYM_COUNT: f32 = 8.;
const SPEED: f32 = 8.;
pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_systems(Update, tick)
        .run();
}

fn init(commands: Commands, asset_server: Res<AssetServer>) {
    show_sprite(commands, asset_server)
}

fn show_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("../assets/scorpion.png"),
        ..default()
    });
    let mut i: f32 = 0.;
    while i < SYM_COUNT {
        i += 1.;
        let sprite_path = format!("../assets/{}.png", i);
        println!("{sprite_path}");
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0., SYM_HEIGHT * (i - SYM_COUNT + 1.), 0.), //-500, 0, 500
                texture: asset_server.load(sprite_path),
                ..default()
            },
            Movable,
        ));
    }
}

fn tick(time: Res<Time>, mut sprite_position: Query<(&Movable, &mut Transform)>) {
    for (_, mut transform) in &mut sprite_position {
        transform.translation.y -= SYM_HEIGHT * time.delta_seconds() * SPEED;
        if transform.translation.y < -1. * (SYM_COUNT / 2.) * SYM_HEIGHT {
            transform.translation.y += SYM_HEIGHT * SYM_COUNT;
        }
    }
}
