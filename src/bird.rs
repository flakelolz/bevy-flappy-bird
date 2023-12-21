use bevy::prelude::*;

use crate::{Collider, Velocity};

const GRAVITY: f32 = -9.8;
const JUMP: f32 = 45.0;
const BIRD_SIZE: Vec2 = Vec2::new(34.0, 24.0);

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BirdAssets {
            yellow: None,
            blue: None,
            red: None,
        })
        .add_systems(PreStartup, load_bird_image)
        .add_systems(Startup, spawn_bird)
        .add_systems(Update, bird_jump);
    }
}

#[derive(Component)]
pub struct Bird;

#[derive(Resource)]
struct BirdAssets {
    yellow: Option<Handle<Image>>,
    blue: Option<Handle<Image>>,
    red: Option<Handle<Image>>,
}

fn load_bird_image(asset_server: Res<AssetServer>, mut birds: ResMut<BirdAssets>) {
    let yellow_texture: Handle<Image> =
        asset_server.load("sprites/birds/yellow_bird/yellowbird-midflap.png");

    let blue_texture: Handle<Image> =
        asset_server.load("sprites/birds/blue_bird/bluebird-midflap.png");

    let red_texture: Handle<Image> =
        asset_server.load("sprites/birds/red_bird/redbird-midflap.png");

    birds.yellow = Some(yellow_texture);
    birds.blue = Some(blue_texture);
    birds.red = Some(red_texture);
}

fn spawn_bird(mut commands: Commands, bird_assets: Res<BirdAssets>) {
    if let Some(texture) = bird_assets.yellow.as_ref() {
        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 3.0),
                ..default()
            },
            Velocity {
                value: Vec2::new(0.0, 0.0),
            },
            Collider {
                // size: Vec2::new(image.width() as f32, image.height() as f32),
                size: BIRD_SIZE * 0.95,
            },
            Bird,
        ));
    }
}

fn bird_jump(
    mut query: Query<(&mut Transform, &mut Velocity), With<Bird>>,
    key: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    let gravity_scale = 150.0;
    for (mut transform, mut velocity) in query.iter_mut() {
        velocity.value.y += GRAVITY * gravity_scale * time.delta_seconds();

        if key.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
            velocity.value.y = (JUMP * -2.0 * (GRAVITY * gravity_scale)).sqrt();
        }

        transform.translation.y += velocity.value.y * time.delta_seconds();
    }
}

