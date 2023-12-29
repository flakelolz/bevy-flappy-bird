use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

pub struct SoundPlugin;
impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEvents>()
            .insert_resource(SoundAssets::default())
            .add_systems(Startup, load_sound_assets)
            .add_systems(Update, play_sound);
    }
}

#[derive(Event)]
pub enum SoundEvents {
    Flap,
    Score,
    Hit,
}

// TODO: Add swoosh and die sounds
#[derive(Resource, Default)]
struct SoundAssets {
    score: Option<Handle<AudioSource>>,
    flap: Option<Handle<AudioSource>>,
    hit: Option<Handle<AudioSource>>,
}

fn load_sound_assets(asset_server: Res<AssetServer>, mut sounds: ResMut<SoundAssets>) {
    sounds.score = Some(asset_server.load("audio/point.ogg"));
    sounds.flap = Some(asset_server.load("audio/wing.ogg"));
    sounds.hit = Some(asset_server.load("audio/hit.ogg"));
}

fn play_sound(
    mut commands: Commands,
    sound: Res<SoundAssets>,
    mut events: EventReader<SoundEvents>,
) {
    for event in events.read() {
        // Play sounds everytime a respective event is received
        match event {
            SoundEvents::Flap => {
                commands.spawn(AudioBundle {
                    source: sound.flap.clone().expect("Embedded"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::new_relative(0.1),
                        ..default()
                    },
                });
            }
            SoundEvents::Score => {
                commands.spawn(AudioBundle {
                    source: sound.score.clone().expect("Embedded"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::new_relative(0.1),
                        ..default()
                    },
                });
            }

            SoundEvents::Hit => {
                commands.spawn(AudioBundle {
                    source: sound.hit.clone().expect("Embedded"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::new_relative(0.1),
                        ..default()
                    },
                });
            }
        }
    }
}
