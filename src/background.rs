use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background)
            .add_systems(Startup, spawn_base);
    }
}

fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("sprites/background/background-day.png");

    commands.spawn(SpriteBundle {
        texture,
        ..default()
    });
}

fn spawn_base(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let window = window_query.single();
    let texture: Handle<Image> = asset_server.load("sprites/base/base.png");

    commands.spawn(SpriteBundle {
        texture,
        transform: Transform {
            translation: Vec3::new(0.0, -(window.height() / 2.0), 2.0),
            ..default()
        },
        ..default()
    });
}
