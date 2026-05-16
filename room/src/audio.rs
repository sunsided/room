use std::cell::RefCell;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use rodio::buffer::SamplesBuffer;
use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player, Source};

thread_local! {
    pub(crate) static AUDIO: RefCell<Option<AudioState>> = const { RefCell::new(None) };
}

pub(crate) fn decode_doom_sfx(data: &[u8]) -> Option<(u32, Vec<f32>)> {
    if data.len() < 8 {
        return None;
    }
    let sample_rate = u16::from_le_bytes([data[2], data[3]]) as u32;
    let num_samples = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
    if sample_rate == 0 || num_samples == 0 || data.len() < 8 + num_samples {
        return None;
    }
    let samples = data[8..8 + num_samples]
        .iter()
        .map(|&b| (b as f32 - 128.0) / 128.0)
        .collect();
    Some((sample_rate, samples))
}

pub(crate) fn gains_from(vol: i32, sep: i32) -> (f32, f32) {
    let v = vol.clamp(0, 127) as f32;
    let s = sep.clamp(0, 254) as f32;
    (v * (254.0 - s) / (127.0 * 254.0), v * s / (127.0 * 254.0))
}

pub(crate) struct PanState {
    pub(crate) l_gain: AtomicU32,
    pub(crate) r_gain: AtomicU32,
}

impl PanState {
    pub(crate) fn new(vol: i32, sep: i32) -> Self {
        let (l, r) = gains_from(vol, sep);
        Self {
            l_gain: AtomicU32::new(l.to_bits()),
            r_gain: AtomicU32::new(r.to_bits()),
        }
    }

    pub(crate) fn update(&self, vol: i32, sep: i32) {
        let (l, r) = gains_from(vol, sep);
        self.l_gain.store(l.to_bits(), Ordering::Relaxed);
        self.r_gain.store(r.to_bits(), Ordering::Relaxed);
    }
}

pub(crate) struct PannedSource<S> {
    inner: S,
    pan: Arc<PanState>,
    buffered: Option<f32>,
}

impl<S: Source> PannedSource<S> {
    pub(crate) fn new(inner: S, pan: Arc<PanState>) -> Self {
        Self { inner, pan, buffered: None }
    }
}

impl<S: Source> Iterator for PannedSource<S> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        match self.buffered.take() {
            None => {
                // Left sample: read mono, buffer raw value for the upcoming right.
                let s = self.inner.next()?;
                let l = s * f32::from_bits(self.pan.l_gain.load(Ordering::Relaxed));
                self.buffered = Some(s);
                Some(l)
            }
            Some(s) => {
                // Right sample: use buffered mono value.
                Some(s * f32::from_bits(self.pan.r_gain.load(Ordering::Relaxed)))
            }
        }
    }
}

impl<S: Source> Source for PannedSource<S> {
    fn current_span_len(&self) -> Option<usize> {
        self.inner.current_span_len().map(|n| n * 2)
    }
    fn channels(&self) -> rodio::ChannelCount {
        std::num::NonZero::new(2).unwrap()
    }
    fn sample_rate(&self) -> rodio::SampleRate {
        self.inner.sample_rate()
    }
    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
    }
}

struct ChannelState {
    player: Option<Player>,
    pan: Arc<PanState>,
}

impl ChannelState {
    fn new() -> Self {
        Self {
            player: None,
            pan: Arc::new(PanState::new(127, 127)),
        }
    }
}

pub(crate) struct AudioState {
    _device_sink: MixerDeviceSink,
    mixer: rodio::mixer::Mixer,
    channels: Box<[ChannelState; 8]>,
}

impl AudioState {
    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let device_sink = DeviceSinkBuilder::open_default_sink()?;
        let mixer = device_sink.mixer().clone();
        let channels = Box::new(std::array::from_fn(|_| ChannelState::new()));
        Ok(Self { _device_sink: device_sink, mixer, channels })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_sfx_silence_sample() {
        // 8-byte header: format=3, rate=11025 (0x2B11 LE), count=1, then byte 128
        let data: &[u8] = &[3, 0, 0x11, 0x2B, 1, 0, 0, 0, 128];
        let (rate, samples) = decode_doom_sfx(data).unwrap();
        assert_eq!(rate, 11025);
        assert_eq!(samples.len(), 1);
        assert!((samples[0] - 0.0_f32).abs() < 1e-4);
    }

    #[test]
    fn decode_sfx_full_positive() {
        // byte 255 should decode to (255-128)/128 = 0.9921875
        let data: &[u8] = &[3, 0, 0x11, 0x2B, 1, 0, 0, 0, 255];
        let (_, samples) = decode_doom_sfx(data).unwrap();
        assert!((samples[0] - (127.0_f32 / 128.0)).abs() < 1e-4);
    }

    #[test]
    fn decode_sfx_full_negative() {
        // byte 0 should decode to (0-128)/128 = -1.0
        let data: &[u8] = &[3, 0, 0x11, 0x2B, 1, 0, 0, 0, 0];
        let (_, samples) = decode_doom_sfx(data).unwrap();
        assert!((samples[0] - (-1.0_f32)).abs() < 1e-4);
    }

    #[test]
    fn decode_sfx_too_short_returns_none() {
        assert!(decode_doom_sfx(&[3, 0, 0, 0]).is_none());
    }

    #[test]
    fn decode_sfx_zero_samples_returns_none() {
        let data: &[u8] = &[3, 0, 0x11, 0x2B, 0, 0, 0, 0];
        assert!(decode_doom_sfx(data).is_none());
    }

    #[test]
    fn decode_sfx_zero_rate_returns_none() {
        // sample_rate=0 (bytes 2-3 are 0x00, 0x00), 1 sample
        let data: &[u8] = &[3, 0, 0x00, 0x00, 1, 0, 0, 0, 128];
        assert!(decode_doom_sfx(data).is_none());
    }

    #[test]
    fn decode_sfx_truncated_returns_none() {
        // claims 4 samples but only has 2 bytes of PCM
        let data: &[u8] = &[3, 0, 0x11, 0x2B, 4, 0, 0, 0, 128, 192];
        assert!(decode_doom_sfx(data).is_none());
    }

    #[test]
    fn gains_center() {
        let (l, r) = gains_from(127, 127);
        assert!((l - 0.5_f32).abs() < 1e-4, "l={l}");
        assert!((r - 0.5_f32).abs() < 1e-4, "r={r}");
    }

    #[test]
    fn gains_hard_left() {
        let (l, r) = gains_from(127, 0);
        assert!((l - 1.0_f32).abs() < 1e-4, "l={l}");
        assert!((r - 0.0_f32).abs() < 1e-4, "r={r}");
    }

    #[test]
    fn gains_hard_right() {
        let (l, r) = gains_from(127, 254);
        assert!((l - 0.0_f32).abs() < 1e-4, "l={l}");
        assert!((r - 1.0_f32).abs() < 1e-4, "r={r}");
    }

    #[test]
    fn gains_clamp_vol() {
        let (l, r) = gains_from(200, 127); // vol clamped to 127
        assert!((l - 0.5_f32).abs() < 1e-4, "l={l}");
        assert!((r - 0.5_f32).abs() < 1e-4, "r={r}");
    }

    #[test]
    fn pan_state_update() {
        let pan = PanState::new(127, 127);
        pan.update(127, 0); // hard left
        let l = f32::from_bits(pan.l_gain.load(Ordering::Relaxed));
        let r = f32::from_bits(pan.r_gain.load(Ordering::Relaxed));
        assert!((l - 1.0_f32).abs() < 1e-4, "l={l}");
        assert!((r - 0.0_f32).abs() < 1e-4, "r={r}");
    }

    #[test]
    fn panned_source_center_two_samples() {
        use std::num::NonZero;
        let buf = SamplesBuffer::new(NonZero::new(1).unwrap(), NonZero::new(11025).unwrap(), vec![1.0_f32, 0.5_f32]);
        let pan = Arc::new(PanState::new(127, 127)); // center: l=r=1.0
        let mut src = PannedSource::new(buf, Arc::clone(&pan));

        let l1 = src.next().unwrap();
        let r1 = src.next().unwrap();
        assert!((l1 - 0.5_f32).abs() < 1e-4, "l1={l1}");
        assert!((r1 - 0.5_f32).abs() < 1e-4, "r1={r1}");

        let l2 = src.next().unwrap();
        let r2 = src.next().unwrap();
        assert!((l2 - 0.25_f32).abs() < 1e-4, "l2={l2}");
        assert!((r2 - 0.25_f32).abs() < 1e-4, "r2={r2}");

        assert!(src.next().is_none());
    }

    #[test]
    fn panned_source_hard_left() {
        use std::num::NonZero;
        // vol=127, sep=0: l=2.0, r=0.0
        let buf = SamplesBuffer::new(NonZero::new(1).unwrap(), NonZero::new(11025).unwrap(), vec![0.5_f32]);
        let pan = Arc::new(PanState::new(127, 0));
        let mut src = PannedSource::new(buf, pan);
        let l = src.next().unwrap();
        let r = src.next().unwrap();
        assert!((l - 0.5_f32).abs() < 1e-4, "l={l}");  // 0.5 * 1.0
        assert!((r - 0.0_f32).abs() < 1e-4, "r={r}");  // 0.5 * 0.0
    }

    #[test]
    fn panned_source_channels_is_2() {
        use std::num::NonZero;
        use rodio::Source;
        let buf = SamplesBuffer::new(NonZero::new(1).unwrap(), NonZero::new(11025).unwrap(), vec![0.0_f32]);
        let pan = Arc::new(PanState::new(127, 127));
        let src = PannedSource::new(buf, pan);
        assert_eq!(src.channels().get(), 2);
    }

    #[test]
    fn panned_source_live_pan_update() {
        use std::num::NonZero;
        // Start centered, then update to hard left mid-stream
        let buf = SamplesBuffer::new(NonZero::new(1).unwrap(), NonZero::new(11025).unwrap(), vec![1.0_f32, 1.0_f32]);
        let pan = Arc::new(PanState::new(127, 127));
        let mut src = PannedSource::new(buf, Arc::clone(&pan));

        // Consume first stereo pair at center
        let _ = src.next(); // L1
        let _ = src.next(); // R1

        // Update to hard left before second pair
        pan.update(127, 0);
        let l2 = src.next().unwrap();
        let r2 = src.next().unwrap();
        assert!((l2 - 1.0_f32).abs() < 1e-4, "l2={l2}");
        assert!((r2 - 0.0_f32).abs() < 1e-4, "r2={r2}");
    }
}
