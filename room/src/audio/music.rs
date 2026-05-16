use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use rodio::Player;

pub(crate) struct MusicHandle {
    pub(crate) midi_bytes: Vec<u8>,
}

pub(crate) struct MusicState {
    pub(crate) sound_font: Option<Arc<rustysynth::SoundFont>>,
    pub(crate) player: Option<Player>,
    pub(crate) volume: Arc<AtomicU32>,
}

impl MusicState {
    pub(crate) fn new() -> Self {
        Self {
            sound_font: None,
            player: None,
            volume: Arc::new(AtomicU32::new(1.0f32.to_bits())),
        }
    }
}
