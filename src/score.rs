use bevy::prelude::*;

use crate::{bird::Bird, ui::ScoreUI, pipe::BottomPipe, sound::SoundEvents, state::AppState};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score { value: 0 })
            .add_systems(Update, udpate_score.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
struct Scored;

#[derive(Resource)]
pub struct Score {
    pub value: i32,
}

fn udpate_score(
    mut commands: Commands,
    mut score: ResMut<Score>,
    bird: Query<&Transform, With<Bird>>,
    pipes: Query<(&Transform, Entity), (With<BottomPipe>, Without<Scored>)>,
    mut score_ui: Query<&mut Text, With<ScoreUI>>,
    mut sound_events: EventWriter<SoundEvents>,
) {
    for bird in bird.iter() {
        for (pipe, entity) in pipes.iter() {
            if bird.translation.x > pipe.translation.x {
                score.value += 1;
                commands.entity(entity).insert(Scored);
                sound_events.send(SoundEvents::Score);
            }
        }
    }

    let mut text = score_ui.single_mut();
    text.sections[0].value = format!("{}", score.value);
}
