use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    bird::Bird,
    pipe::{BottomPipe, TopPipe},
    Collider,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_system);
    }
}

fn collision_system(
    bird: Query<(&Transform, &Collider), With<Bird>>,
    pipe: Query<(&Transform, &Collider), Or<(With<BottomPipe>, With<TopPipe>)>>,
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
                // TODO: Implement game over
                info!("Collided with pipe");
            }
        }
    }
}

