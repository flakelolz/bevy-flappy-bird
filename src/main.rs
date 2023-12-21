use bevy::prelude::*;

mod background;
mod bird;
mod pipe;
mod score;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bird".into(),
                        resolution: (288., 512.).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(bird::BirdPlugin)
        .add_plugins(pipe::PipePlugin)
        .add_plugins(background::BackgroundPlugin)
        .add_plugins(score::ScorePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec2,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
