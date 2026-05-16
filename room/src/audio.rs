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
    if num_samples == 0 || data.len() < 8 + num_samples {
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
    (v * (254.0 - s) / (127.0 * 127.0), v * s / (127.0 * 127.0))
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

pub(crate) struct AudioState;

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
    fn decode_sfx_truncated_returns_none() {
        // claims 4 samples but only has 2 bytes of PCM
        let data: &[u8] = &[3, 0, 0x11, 0x2B, 4, 0, 0, 0, 128, 192];
        assert!(decode_doom_sfx(data).is_none());
    }

    #[test]
    fn gains_center() {
        let (l, r) = gains_from(127, 127);
        assert!((l - 1.0_f32).abs() < 1e-4, "l={l}");
        assert!((r - 1.0_f32).abs() < 1e-4, "r={r}");
    }

    #[test]
    fn gains_hard_left() {
        let (l, r) = gains_from(127, 0);
        assert!((l - 2.0_f32).abs() < 1e-4, "l={l}");
        assert!((r - 0.0_f32).abs() < 1e-4, "r={r}");
    }

    #[test]
    fn gains_hard_right() {
        let (l, r) = gains_from(127, 254);
        assert!((l - 0.0_f32).abs() < 1e-4, "l={l}");
        assert!((r - 2.0_f32).abs() < 1e-4, "r={r}");
    }

    #[test]
    fn gains_clamp_vol() {
        let (l, r) = gains_from(200, 127); // vol clamped to 127
        assert!((l - 1.0_f32).abs() < 1e-4, "l={l}");
        assert!((r - 1.0_f32).abs() < 1e-4, "r={r}");
    }

    #[test]
    fn pan_state_update() {
        let pan = PanState::new(127, 127);
        pan.update(127, 0); // hard left
        let l = f32::from_bits(pan.l_gain.load(Ordering::Relaxed));
        let r = f32::from_bits(pan.r_gain.load(Ordering::Relaxed));
        assert!((l - 2.0_f32).abs() < 1e-4, "l={l}");
        assert!((r - 0.0_f32).abs() < 1e-4, "r={r}");
    }
}
