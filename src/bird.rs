use std::time::Duration;

use bevy::prelude::*;

use crate::{
    sound::SoundEvents,
    state::{AppEvents, AppState},
    Collider, Velocity,
};

const GRAVITY: f32 = -9.8;
const JUMP: f32 = 45.0;
const BIRD_SIZE: Vec2 = Vec2::new(34.0, 24.0);
const ASCENDING: f32 = 200.0;
const FALLING: f32 = -250.0;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BirdAssets::default())
            .add_systems(
                Startup,
                (load_bird_atlas, spawn_bird)
                    .chain()
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(
                Update,
                animate_bird.run_if(not(in_state(AppState::Restart))),
            )
            .add_systems(Update, bird_jump.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Resource, Default)]
pub struct BirdAssets {
    pub birds_atlas: Option<Handle<TextureAtlas>>,
}

#[derive(Component)]
pub struct Bird;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn load_bird_atlas(
    asset_server: Res<AssetServer>,
    mut bird_assets: ResMut<BirdAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/birds/flappybird-sheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle.clone(),
        Vec2::new(34.0, 24.0),
        4,
        4,
        None,
        None,
    );

    bird_assets.birds_atlas = Some(texture_atlases.add(texture_atlas));
}

fn spawn_bird(mut commands: Commands, bird_assets: ResMut<BirdAssets>) {
    let animation_indices = AnimationIndices { first: 0, last: 3 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: bird_assets
                .birds_atlas
                .clone()
                .expect("Already loaded and embedded in the binary"),
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_xyz(-50.0, -49.0, 3.0),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Velocity::default(),
        Collider { size: BIRD_SIZE },
        Bird,
    ));
}

fn animate_bird(
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Velocity,
    )>,
    time: Res<Time>,
) {
    for (indices, mut timer, mut sprite, velocity) in &mut query {
        timer.tick(time.delta());

        if velocity.value.y > ASCENDING {
            timer.set_duration(Duration::from_secs_f32(0.5));
        } else {
            timer.set_duration(Duration::from_secs_f32(0.1));
        }

        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        } else if velocity.value.y < FALLING {
            sprite.index = indices.last - 1;
        }
    }
}

fn bird_jump(
    mut query: Query<(&mut Transform, &mut Velocity), With<Bird>>,
    mut event_set: ParamSet<(EventReader<AppEvents>, EventWriter<SoundEvents>)>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    time: Res<Time>,
) {
    let window = window_query
        .get_single()
        .expect("Window should exist at this point");
    let gravity_scale = 150.0;
    for (mut transform, mut velocity) in &mut query {
        velocity.value.y += GRAVITY * gravity_scale * time.delta_seconds();

        if event_set.p0().read().any(|e| e == &AppEvents::Tap)
            && transform.translation.y < window.height() / 2.0
        {
            transform.rotation = Quat::from_rotation_z(0.35);
            velocity.value.y = (JUMP * -2.0 * (GRAVITY * gravity_scale)).sqrt();
            event_set.p1().send(SoundEvents::Flap);
        }

        if velocity.value.y < FALLING {
            transform.rotation = transform.rotation.lerp(Quat::from_rotation_z(-1.0), 0.08);
        }

        transform.translation.y += velocity.value.y * time.delta_seconds();
    }
}
