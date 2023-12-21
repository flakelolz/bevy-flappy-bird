use bevy::prelude::*;

use crate::{bird::Bird, pipe::BottomPipe};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score { value: 0 })
            .add_systems(Update, udpate_score);
    }
}

#[derive(Component)]
struct Scored;

#[derive(Resource)]
struct Score {
    value: i32,
}

fn udpate_score(
    mut commands: Commands,
    mut score: ResMut<Score>,
    bird: Query<&Transform, With<Bird>>,
    pipes: Query<(&Transform, Entity), (With<BottomPipe>, Without<Scored>)>,
) {
    for bird in bird.iter() {
        for (pipe, entity) in pipes.iter() {
            if bird.translation.x > pipe.translation.x {
                score.value += 1;
                println!("Score: {}", score.value);
                commands.entity(entity).insert(Scored);
            }
        }
    }
}

