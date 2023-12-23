use bevy::prelude::*;

mod background;
mod bird;
mod collision;
mod input;
mod menu;
mod pipe;
mod restart;
mod score;
mod state;

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
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(bird::BirdPlugin)
        .add_plugins(pipe::PipePlugin)
        .add_plugins(background::BackgroundPlugin)
        .add_plugins(collision::CollisionPlugin)
        .add_plugins(score::ScorePlugin)
        .add_plugins(menu::MenuPlugin)
        .add_plugins(state::StatePlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(restart::RestartPlugin)
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
