use bevy::{prelude::*, window::EnabledButtons};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

mod background;
mod bird;
mod collision;
mod input;
mod pipe;
mod restart;
mod score;
mod sound;
mod state;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bird".into(),
                        resolution: (288., 512.).into(),
                        resizable: false,
                        enabled_buttons: EnabledButtons {
                            minimize: false,
                            maximize: false,
                            close: true,
                        },
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        ))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(bird::BirdPlugin)
        .add_plugins(pipe::PipePlugin)
        .add_plugins(background::BackgroundPlugin)
        .add_plugins(collision::CollisionPlugin)
        .add_plugins(score::ScorePlugin)
        .add_plugins(ui::MenuPlugin)
        .add_plugins(state::StatePlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(restart::RestartPlugin)
        .add_plugins(sound::SoundPlugin)
        .run();
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec2,
}

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

