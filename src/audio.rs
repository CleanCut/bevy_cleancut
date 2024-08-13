use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use rand::{thread_rng, Rng};

/// Create a tuple of components to pass to commands.spawn(...) representing a nicely-named sound
/// effect that will play once at a slightly random speed and then despawn itself.
pub fn play_sound<S: Into<String>>(
    commands: &mut Commands,
    asset_server: &AssetServer,
    filename: S,
    volume: f32,
) {
    let filename = filename.into();
    // Turn a filename like "sound.wav" into a name like "Sound"
    let name = filename.split_terminator(".").next().unwrap();
    let mut chars = name.chars();
    let name_titlecase = match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    } + " Sound";
    commands.spawn((
        Name::new(name_titlecase),
        AudioBundle {
            source: asset_server.load(filename),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::new(volume),
                speed: thread_rng().gen_range(1.0..1.2),
                ..Default::default()
            },
        },
    ));
}
