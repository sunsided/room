pub(crate) mod music;
pub(crate) mod sfx;

use std::cell::RefCell;
use std::sync::Arc;

use rodio::buffer::SamplesBuffer;
use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player};

pub(crate) use sfx::{PanState, PannedSource, decode_doom_sfx, gains_from};

thread_local! {
    pub(crate) static AUDIO: RefCell<Option<AudioState>> = const { RefCell::new(None) };
}

pub(crate) struct AudioState {
    pub(crate) _device_sink: MixerDeviceSink,
    pub(crate) mixer: rodio::mixer::Mixer,
    channels: Box<[sfx::ChannelState; 8]>,
    pub(crate) music: music::MusicState,
}

impl AudioState {
    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let device_sink = DeviceSinkBuilder::open_default_sink()?;
        let mixer = device_sink.mixer().clone();
        let channels = Box::new(std::array::from_fn(|_| sfx::ChannelState::new()));
        Ok(Self {
            _device_sink: device_sink,
            mixer,
            channels,
            music: music::MusicState::new(),
        })
    }

    pub(crate) fn start_sound(&mut self, data: &[u8], vol: i32, sep: i32, channel: usize) -> bool {
        if channel >= 8 { return false; }
        let Some((sample_rate, samples)) = decode_doom_sfx(data) else { return false; };
        let pan = Arc::clone(&self.channels[channel].pan);
        pan.update(vol, sep);
        let buf = SamplesBuffer::new(
            std::num::NonZero::new(1u16).unwrap(),
            std::num::NonZero::new(sample_rate).unwrap(),
            samples,
        );
        let source = PannedSource::new(buf, pan);
        let player = Player::connect_new(&self.mixer);
        player.append(source);
        self.channels[channel].player = Some(player);
        true
    }

    pub(crate) fn stop_sound(&mut self, channel: usize) {
        if channel >= 8 { return; }
        self.channels[channel].player = None;
    }

    pub(crate) fn update_sound_params(&self, channel: usize, vol: i32, sep: i32) {
        if channel >= 8 { return; }
        self.channels[channel].pan.update(vol, sep);
    }

    pub(crate) fn is_playing(&self, channel: usize) -> bool {
        if channel >= 8 { return false; }
        self.channels[channel].player.as_ref().map_or(false, |p| !p.empty())
    }
}
