use bevy::prelude::*;

use crate::{
    bird::Bird,
    ui::GameOver,
    pipe::{BottomPipe, TopPipe},
    score::Score,
    state::{AppEvents, AppState},
};
pub struct RestartPlugin;

impl Plugin for RestartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, restart);
    }
}

fn restart(
    mut commands: Commands,
    mut pipe_query: Query<Entity, Or<(With<BottomPipe>, With<TopPipe>)>>,
    mut bird_query: Query<(&mut Transform, &mut Visibility), With<Bird>>,
    mut game_over: Query<&mut Visibility, (With<GameOver>, Without<Bird>)>,
    states: Res<State<AppState>>,
    mut events: EventWriter<AppEvents>,
    mut score: ResMut<Score>,
) {
    if states.as_ref() == &AppState::Restart {
        if let Ok((mut bird_transform, mut bird_visibility)) = bird_query.get_single_mut() {
            *bird_transform = Transform::from_xyz(-50.0, -49.0, 3.0);
            *bird_visibility = Visibility::Visible;
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
