use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    background::Floor,
    bird::Bird,
    pipe::{BottomPipe, TopPipe},
    sound::SoundEvents,
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
    floor: Query<(&Transform, &Collider), With<Floor>>,
    mut event_set: ParamSet<(EventWriter<AppEvents>, EventWriter<SoundEvents>)>,
) {
    for (bird_transform, bird_collider) in bird.iter() {
        // Screen Bound
        if bird_transform.translation.x < -144.0
            || bird_transform.translation.x > 144.0
            || bird_transform.translation.y < -256.0
            || bird_transform.translation.y > 256.0
        {
            for mut bird_visibility in bird_query.iter_mut() {
                *bird_visibility = Visibility::Hidden;
                event_set.p0().send(AppEvents::Collision);
                event_set.p1().send(SoundEvents::Hit);
            }
        }

        // Pipe Collisions
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
                    event_set.p0().send(AppEvents::Collision);
                    event_set.p1().send(SoundEvents::Hit);
                }
            }
        }
        // Floor Collisions
        for (floor_transform, floor_collider) in floor.iter() {
            if collide(
                bird_transform.translation,
                bird_collider.size,
                floor_transform.translation,
                floor_collider.size,
            )
            .is_some()
            {
                for mut bird_visibility in bird_query.iter_mut() {
                    *bird_visibility = Visibility::Hidden;
                    event_set.p0().send(AppEvents::Collision);
                    event_set.p1().send(SoundEvents::Hit);
                }
            }
        }
    }
}
