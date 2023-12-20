use bevy::prelude::*;
use rand::Rng;

use crate::Velocity;

const PIPE: (f32, f32) = (52.0, 320.0);
const GAP: f32 = 50.0;

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PipeTimer {
            spawn: Timer::from_seconds(1.7, TimerMode::Repeating),
        })
        .add_systems(Update, move_pipes)
        .add_systems(Update, spawn_pipes);
    }
}

#[derive(Resource)]
struct PipeTimer {
    spawn: Timer,
}

#[derive(Component)]
struct TopPipe;

#[derive(Component)]
struct BottomPipe;

fn spawn_pipes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut timer: ResMut<PipeTimer>,
    time: Res<Time>,
) {
    let texture: Handle<Image> = asset_server.load("sprites/pipes/pipe-green.png");
    let window = window_query.single();
    let pipe_position = (window.height() / 2.0) - (PIPE.1 / 2.0);
    let positions = [0.0, 100.0, -100.0, 125.0, -125.0];

    if timer.spawn.tick(time.delta()).just_finished() {
        let rng = rand::thread_rng().gen_range(0..positions.len());
        let variance = positions[rng];

        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        window.width() / 2.0,
                        (pipe_position + GAP) + variance,
                        0.0,
                    ),
                    rotation: Quat::from_rotation_x(180.0_f32.to_radians()),
                    ..default()
                },
                ..default()
            },
            Velocity {
                value: Vec2::new(100.0, 0.0),
            },
            TopPipe,
        ));

        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_xyz(
                    window.width() / 2.0,
                    ((pipe_position * -1.0) - GAP) + variance,
                    0.0,
                ),
                ..default()
            },
            Velocity {
                value: Vec2::new(100.0, 0.0),
            },
            BottomPipe,
        ));
    }
}

fn move_pipes(
    mut query: Query<(&mut Transform, &Velocity), Or<(With<BottomPipe>, With<TopPipe>)>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x -= velocity.value.x * time.delta_seconds();
    }
}

