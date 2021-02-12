mod mp3_loader;
mod ogg_loader;
mod wav_loader;

use bevy_reflect::TypeUuid;
use kira::sound::Sound;

#[cfg(feature = "mp3")]
pub use mp3_loader::Mp3Loader;
#[cfg(feature = "vorbis")]
pub use ogg_loader::OggLoader;
#[cfg(feature = "wav")]
pub use wav_loader::WavLoader;

/// A source of audio data
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "7a14806a-672b-443b-8d16-4f18afefa463"]
pub struct AudioSource {
    pub sound: Sound,
}
