use bevy::prelude::*;
use rand::Rng;

use crate::{
    background::{Background, BackgroundAssets},
    bird::{AnimationIndices, Bird},
    pipe::{BottomPipe, PipeAssets, RandomPipe, TopPipe},
    score::Score,
    state::{AppEvents, AppState},
    ui::GameOver,
    Velocity,
};
pub struct RestartPlugin;

impl Plugin for RestartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                randomize_bird,
                randomize_pipes,
                randomize_background,
                restart,
            )
                .chain()
                .run_if(in_state(AppState::Restart)),
        );
    }
}

fn restart(
    mut commands: Commands,
    mut pipe_query: Query<Entity, Or<(With<BottomPipe>, With<TopPipe>)>>,
    mut bird_query: Query<
        (
            &mut Transform,
            &mut Velocity,
            &mut Visibility,
            &mut TextureAtlasSprite,
            &AnimationIndices,
        ),
        With<Bird>,
    >,
    mut game_over: Query<&mut Visibility, (With<GameOver>, Without<Bird>)>,
    states: Res<State<AppState>>,
    mut events: EventWriter<AppEvents>,
    mut score: ResMut<Score>,
) {
    if states.as_ref() == &AppState::Restart {
        if let Ok((
            mut bird_transform,
            mut bird_velocity,
            mut bird_visibility,
            mut bird_sprite,
            index,
        )) = bird_query.get_single_mut()
        {
            *bird_transform = Transform::from_xyz(-50.0, -49.0, 3.0);
            *bird_velocity = Velocity::default();
            *bird_visibility = Visibility::Visible;
            bird_sprite.index = index.first;
        }
        for pipe in pipe_query.iter_mut() {
            commands.entity(pipe).despawn();
        }

        if let Ok(mut game_over) = game_over.get_single_mut() {
            *game_over = Visibility::Hidden;
        }

        score.value = 0;
        events.send(AppEvents::Restarted);
    }
}

fn randomize_bird(mut query: Query<&mut AnimationIndices, With<Bird>>) {
    let rng = rand::thread_rng().gen_range(0..3);
    for mut index in &mut query {
        match rng {
            0 => {
                index.first = 0;
                index.last = 3;
            }
            1 => {
                index.first = 4;
                index.last = 7;
            }
            _ => {
                index.first = 8;
                index.last = 11;
            }
        }
    }
}

fn randomize_pipes(assets: Res<PipeAssets>, mut randomize: ResMut<RandomPipe>) {
    let rng = rand::thread_rng().gen_range(0..2);
    randomize.as_mut().texture = match rng {
        0 => Some(assets.as_ref().green.clone().expect("Embedded")),
        _ => Some(assets.as_ref().red.clone().expect("Embedded")),
    }
}

fn randomize_background(
    mut query: Query<&mut Handle<Image>, With<Background>>,
    assets: Res<BackgroundAssets>,
) {
    let rng = rand::thread_rng().gen_range(0..2);
    if let Ok(mut sprite) = query.get_single_mut() {
        match rng {
            0 => {
                *sprite = assets
                    .day
                    .clone()
                    .expect("Image should be loaded by this point");
            }
            _ => {
                *sprite = assets
                    .night
                    .clone()
                    .expect("Image should be loaded by this point");
            }
        }
    }
}
