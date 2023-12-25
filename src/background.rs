use bevy::prelude::*;

use crate::Collider;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BackgroundAssets::default())
            .add_systems(Startup, (load_background_texture, spawn_background).chain())
            .add_systems(Startup, spawn_base);
    }
}

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Floor;

#[derive(Resource, Default)]
pub struct BackgroundAssets {
    pub day: Option<Handle<Image>>,
    pub night: Option<Handle<Image>>,
}

fn load_background_texture(
    asset_server: Res<AssetServer>,
    mut background_assets: ResMut<BackgroundAssets>,
) {
    background_assets.day = Some(asset_server.load("sprites/backgrounds/background-day.png"));
    background_assets.night = Some(asset_server.load("sprites/backgrounds/background-night.png"));
}

fn spawn_background(mut commands: Commands, assets: Res<BackgroundAssets>) {
    let texture = assets.day.as_ref().expect("Embedded in the binary");

    commands.spawn((
        SpriteBundle {
            texture: texture.clone(),
            ..default()
        },
        Background,
    ));
}

fn spawn_base(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let window = window_query.single();
    let texture: Handle<Image> = asset_server.load("sprites/base/base.png");

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3::new(0.0, -(window.height() / 2.0), 2.0),
                ..default()
            },
            ..default()
        },
        Collider {
            size: Vec2::new(336.0, 112.0),
        },
        Floor,
    ));
}
