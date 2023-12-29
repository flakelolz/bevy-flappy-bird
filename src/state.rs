use bevy::prelude::*;

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_event::<AppEvents>()
            .insert_resource(StateTimer(Timer::from_seconds(1.0, TimerMode::Once)))
            .add_systems(Update, state_machine);
    }
}

#[derive(States, PartialEq, Eq, Clone, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
    Restart,
}

#[derive(Event, PartialEq)]
pub enum AppEvents {
    Collision,
    Tap,
    Restarted,
}

#[derive(Resource)]
struct StateTimer(Timer);

// State logic driven by state events received
fn state_machine(
    mut commands: Commands,
    states: Res<State<AppState>>,
    mut event_reader: EventReader<AppEvents>,
    mut timer: ResMut<StateTimer>,
    time: Res<Time>,
) {
    if states.as_ref() == &AppState::MainMenu && event_reader.read().any(|e| e == &AppEvents::Tap) {
        commands.insert_resource(NextState(Some(AppState::InGame)));
    }

    if states.as_ref() == &AppState::InGame
        && event_reader.read().any(|e| e == &AppEvents::Collision)
    {
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }

    if states.as_ref() == &AppState::GameOver {
        timer.0.tick(time.delta());

        if timer.0.finished() && event_reader.read().any(|e| e == &AppEvents::Tap) {
            timer.0.reset();
            commands.insert_resource(NextState(Some(AppState::Restart)));
        }
    }

    if states.as_ref() == &AppState::Restart
        && event_reader.read().any(|e| e == &AppEvents::Restarted)
    {
        commands.insert_resource(NextState(Some(AppState::MainMenu)));
    }
}
