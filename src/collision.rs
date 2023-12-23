use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    bird::Bird,
    pipe::{BottomPipe, TopPipe},
    state::{AppEvents, AppState},
    Collider,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_system.run_if(in_state(AppState::InGame)));
    }
}

fn collision_system(
    bird: Query<(&Transform, &Collider), With<Bird>>,
    mut bird_query: Query<&mut Visibility, With<Bird>>,
    pipe: Query<(&Transform, &Collider), Or<(With<BottomPipe>, With<TopPipe>)>>,
    mut state: EventWriter<AppEvents>,
) {
    for (bird_transform, bird_collider) in bird.iter() {
        for (pipe_transform, pipe_collider) in pipe.iter() {
            if collide(
                bird_transform.translation,
                bird_collider.size,
                pipe_transform.translation,
                pipe_collider.size,
            )
            .is_some()
            {
                for mut bird_visibility in bird_query.iter_mut() {
                    *bird_visibility = Visibility::Hidden;
                    state.send(AppEvents::Collision);
                }
            }
        }
    }
}
