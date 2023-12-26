use bevy::prelude::*;

use crate::{bird::Bird, pipe::BottomPipe, sound::SoundEvents, state::AppState, ui::ScoreUI};

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
    bird_query: Query<&Transform, With<Bird>>,
    pipes: Query<(&Transform, Entity), (With<BottomPipe>, Without<Scored>)>,
    mut score_ui: Query<&mut Text, With<ScoreUI>>,
    mut sound_events: EventWriter<SoundEvents>,
) {
    let bird = bird_query.get_single().expect("A bird should always exist");

    // Every time the bird's middle point is greater than the pipe's middle point, tag it as scored
    // and increase the score counter
    for (pipe, entity) in pipes.iter() {
        if bird.translation.x > pipe.translation.x {
            score.value += 1;
            commands.entity(entity).insert(Scored);
            sound_events.send(SoundEvents::Score);
        }
    }

    // Update the score UI
    let mut text = score_ui.single_mut();
    text.sections[0].value = format!("{}", score.value);
}

