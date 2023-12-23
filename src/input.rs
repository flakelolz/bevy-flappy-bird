use bevy::prelude::*;

use crate::state::AppEvents;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, input);
    }
}

fn input(
    mut event_writer: EventWriter<AppEvents>,
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    if keys.any_just_pressed([KeyCode::Space, KeyCode::Up]) || mouse.just_pressed(MouseButton::Left)
    {
        event_writer.send(AppEvents::Tap);
    }
}
