use bevy::prelude::*;

use crate::{
    sound::SoundEvents,
    state::{AppEvents, AppState},
    Collider, Velocity,
};

const GRAVITY: f32 = -9.8;
const JUMP: f32 = 45.0;
const BIRD_SIZE: Vec2 = Vec2::new(34.0, 24.0);

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BirdAssets::default())
            .add_systems(
                Startup,
                (load_bird_image, spawn_bird)
                    .chain()
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(Update, bird_jump.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct Bird;

#[derive(Resource, Default)]
pub struct BirdAssets {
    pub yellow: Option<Handle<Image>>,
    pub blue: Option<Handle<Image>>,
    pub red: Option<Handle<Image>>,
}

fn load_bird_image(asset_server: Res<AssetServer>, mut birds: ResMut<BirdAssets>) {
    birds.yellow = Some(asset_server.load("sprites/birds/yellow_bird/yellowbird-midflap.png"));
    birds.blue = Some(asset_server.load("sprites/birds/blue_bird/bluebird-midflap.png"));
    birds.red = Some(asset_server.load("sprites/birds/red_bird/redbird-midflap.png"));
}

fn spawn_bird(mut commands: Commands, bird_assets: Res<BirdAssets>) {
    let texture = bird_assets.yellow.as_ref().expect("Embedded in the binary");
    commands.spawn((
        SpriteBundle {
            texture: texture.clone(),
            transform: Transform::from_xyz(-50.0, -49.0, 3.0),
            ..default()
        },
        Velocity {
            value: Vec2::new(0.0, 0.0),
        },
        Collider {
            // size: Vec2::new(image.width() as f32, image.height() as f32),
            size: BIRD_SIZE,
        },
        Bird,
    ));
}

fn bird_jump(
    mut query: Query<(&mut Transform, &mut Velocity), With<Bird>>,
    mut event_set: ParamSet<(EventReader<AppEvents>, EventWriter<SoundEvents>)>,
    time: Res<Time>,
) {
    let gravity_scale = 150.0;
    for (mut transform, mut velocity) in query.iter_mut() {
        velocity.value.y += GRAVITY * gravity_scale * time.delta_seconds();

        if event_set.p0().read().any(|e| e == &AppEvents::Tap) {
            velocity.value.y = (JUMP * -2.0 * (GRAVITY * gravity_scale)).sqrt();
            event_set.p1().send(SoundEvents::Flap);
        }

        transform.translation.y += velocity.value.y * time.delta_seconds();
    }
}
