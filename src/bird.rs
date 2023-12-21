use bevy::prelude::*;

use crate::Velocity;

const GRAVITY: f32 = -9.8;
const JUMP: f32 = 45.0;
pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bird)
            .add_systems(Update, bird_jump);
    }
}

#[derive(Component)]
pub struct Bird;

fn spawn_bird(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("sprites/yellow_bird/yellowbird-midflap.png");

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            ..default()
        },
        Velocity {
            value: Vec2::new(0.0, 0.0),
        },
        Bird,
    ));
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

