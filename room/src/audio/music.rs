use std::io::{BufReader, Cursor};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;
use rodio::{Player, Source};
use rustysynth::{MidiFile, MidiFileSequencer, SoundFont, Synthesizer, SynthesizerSettings};

const BLOCK_SIZE: usize = 512;
const SAMPLE_RATE: i32 = 44100;

pub(crate) struct MusicHandle {
    pub(crate) midi_bytes: Vec<u8>,
}

// ---------------------------------------------------------------------------
// MusicSource — rodio Source that renders MIDI via rustysynth
// ---------------------------------------------------------------------------

pub(crate) struct MusicSource {
    sequencer: MidiFileSequencer,
    buf_l: Vec<f32>,
    buf_r: Vec<f32>,
    pos: usize,
    finished: bool,
    volume: Arc<AtomicU32>,
}

impl MusicSource {
    pub(crate) fn new(
        sound_font: &Arc<SoundFont>,
        midi_bytes: &[u8],
        looping: bool,
        volume: Arc<AtomicU32>,
    ) -> Option<Self> {
        let settings = SynthesizerSettings::new(SAMPLE_RATE);
        let synthesizer = Synthesizer::new(sound_font, &settings).ok()?;
        let midi_file = Arc::new(MidiFile::new(&mut Cursor::new(midi_bytes)).ok()?);
        let mut sequencer = MidiFileSequencer::new(synthesizer);
        sequencer.play(&midi_file, looping);
        Some(Self {
            sequencer,
            buf_l: vec![0.0f32; BLOCK_SIZE],
            buf_r: vec![0.0f32; BLOCK_SIZE],
            pos: BLOCK_SIZE * 2, // triggers render on first next()
            finished: false,
            volume,
        })
    }
}

impl Iterator for MusicSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.pos >= BLOCK_SIZE * 2 {
            if self.finished {
                return None;
            }
            self.sequencer.render(&mut self.buf_l, &mut self.buf_r);
            self.pos = 0;
            if self.sequencer.end_of_sequence() {
                self.finished = true;
            }
        }
        let vol = f32::from_bits(self.volume.load(Ordering::Relaxed));
        let sample = if self.pos % 2 == 0 {
            self.buf_l[self.pos / 2] * vol
        } else {
            self.buf_r[self.pos / 2] * vol
        };
        self.pos += 1;
        Some(sample)
    }
}

impl Source for MusicSource {
    fn current_span_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> rodio::ChannelCount {
        std::num::NonZero::new(2u16).unwrap()
    }
    fn sample_rate(&self) -> rodio::SampleRate {
        std::num::NonZero::new(SAMPLE_RATE as u32).unwrap()
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

// ---------------------------------------------------------------------------
// MusicState — SF2 loading and playback control
// ---------------------------------------------------------------------------

pub(crate) struct MusicState {
    pub(crate) sound_font: Option<Arc<SoundFont>>,
    player: Option<Player>,
    volume: Arc<AtomicU32>,
}

impl MusicState {
    pub(crate) fn new() -> Self {
        Self {
            sound_font: None,
            player: None,
            volume: Arc::new(AtomicU32::new(1.0f32.to_bits())),
        }
    }

    pub(crate) fn load_sound_font(&mut self, path: &std::path::Path) {
        match std::fs::File::open(path) {
            Ok(f) => {
                let mut reader = BufReader::new(f);
                match SoundFont::new(&mut reader) {
                    Ok(sf) => {
                        log::info!("Soundfont loaded: {}", path.display());
                        self.sound_font = Some(Arc::new(sf));
                    }
                    Err(e) => log::warn!("Soundfont parse error ({}): {e}", path.display()),
                }
            }
            Err(e) => log::warn!("Soundfont not found ({}): {e}", path.display()),
        }
    }

    pub(crate) fn play(
        &mut self,
        midi_bytes: &[u8],
        looping: bool,
        mixer: &rodio::mixer::Mixer,
    ) {
        let Some(sf) = self.sound_font.as_ref() else {
            return;
        };
        let Some(source) =
            MusicSource::new(sf, midi_bytes, looping, Arc::clone(&self.volume))
        else {
            log::warn!("I_PlaySong: failed to create MusicSource");
            return;
        };
        self.player = None;
        let player = Player::connect_new(mixer);
        player.append(source);
        self.player = Some(player);
    }

    pub(crate) fn stop(&mut self) {
        self.player = None;
    }

    pub(crate) fn set_volume(&self, vol: i32) {
        let gain = (vol.clamp(0, 127) as f32 / 127.0).to_bits();
        self.volume.store(gain, Ordering::Relaxed);
    }

    pub(crate) fn pause(&self) {
        if let Some(p) = &self.player {
            p.pause();
        }
    }

    pub(crate) fn resume(&self) {
        if let Some(p) = &self.player {
            p.play();
        }
    }

    pub(crate) fn is_playing(&self) -> bool {
        self.player.as_ref().map_or(false, |p| !p.empty())
    }
}

// ---------------------------------------------------------------------------
// MUS-to-MIDI converter
// ---------------------------------------------------------------------------

const MUS_MAGIC: &[u8; 4] = b"MUS\x1a";
const MIDI_TEMPO: u32 = 500_000; // µs per beat (120 BPM) — yields 140 ticks/sec = MUS tick rate
const MIDI_PPQ: u16 = 70;          // ticks per beat; 1 tick = 1 MUS tick (1/140 s)

fn mus_to_midi_channel(mus_ch: u8) -> u8 {
    if mus_ch == 15 { 9 }              // percussion
    else if mus_ch >= 9 { mus_ch + 1 } // skip MIDI ch 9
    else { mus_ch }
}

fn write_var_len(buf: &mut Vec<u8>, mut val: u32) {
    let b0 = (val & 0x7F) as u8; val >>= 7;
    if val == 0 { buf.push(b0); return; }
    let b1 = (val & 0x7F) as u8; val >>= 7;
    if val == 0 { buf.extend_from_slice(&[b1 | 0x80, b0]); return; }
    let b2 = (val & 0x7F) as u8; val >>= 7;
    if val == 0 { buf.extend_from_slice(&[b2 | 0x80, b1 | 0x80, b0]); return; }
    let b3 = (val & 0x7F) as u8;
    buf.extend_from_slice(&[b3 | 0x80, b2 | 0x80, b1 | 0x80, b0]);
}

fn read_delay(score: &[u8], pos: &mut usize) -> Option<u32> {
    let mut delay = 0u32;
    loop {
        if *pos >= score.len() { return None; }
        let b = score[*pos]; *pos += 1;
        delay = delay.saturating_mul(128).saturating_add((b & 0x7F) as u32);
        if b & 0x80 == 0 { return Some(delay); }
    }
}

pub(crate) fn mus2midi(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 17 || &data[0..4] != MUS_MAGIC {
        return None;
    }
    let score_len = u16::from_le_bytes([data[4], data[5]]) as usize;
    let score_start = u16::from_le_bytes([data[6], data[7]]) as usize;
    if score_start >= data.len() || score_start + score_len > data.len() {
        return None;
    }
    let score = &data[score_start..score_start + score_len];
    let mut pos = 0usize;
    let mut track: Vec<u8> = Vec::new();

    // Tempo meta-event at delta 0
    track.extend_from_slice(&[
        0x00, 0xFF, 0x51, 0x03,
        ((MIDI_TEMPO >> 16) & 0xFF) as u8,
        ((MIDI_TEMPO >> 8) & 0xFF) as u8,
        (MIDI_TEMPO & 0xFF) as u8,
    ]);

    let mut delta: u32 = 0;
    let mut last_vol = [100u8; 16]; // default velocity per MUS channel

    loop {
        if pos >= score.len() {
            return None; // truncated score
        }
        let ev = score[pos]; pos += 1;
        let mus_ch = ev & 0x0F;
        let ev_type = (ev >> 4) & 0x07;
        let is_last = ev >> 7 != 0;
        let mid_ch = mus_to_midi_channel(mus_ch);

        // Collect MIDI bytes for this event (empty = no event emitted)
        let mut midi_ev: Vec<u8> = Vec::new();

        match ev_type {
            0 => { // note off
                if pos >= score.len() { return None; }
                let note = score[pos] & 0x7F; pos += 1;
                midi_ev.extend_from_slice(&[0x80 | mid_ch, note, 0x40]);
            }
            1 => { // note on
                if pos >= score.len() { return None; }
                let raw = score[pos]; pos += 1;
                if raw & 0x80 != 0 {
                    if pos >= score.len() { return None; }
                    last_vol[mus_ch as usize] = score[pos] & 0x7F; pos += 1;
                }
                let note = raw & 0x7F;
                let vol = last_vol[mus_ch as usize];
                midi_ev.extend_from_slice(&[0x90 | mid_ch, note, vol]);
            }
            2 => { // pitch bend
                if pos >= score.len() { return None; }
                let b = score[pos] as u32; pos += 1;
                // MUS: 0=down, 128=center, 255=up. MIDI: 14-bit, center=0x2000.
                let bend = (b * 64) as u16;
                midi_ev.extend_from_slice(&[
                    0xE0 | mid_ch,
                    (bend & 0x7F) as u8,
                    (bend >> 7) as u8,
                ]);
            }
            3 => { // system event
                if pos >= score.len() { return None; }
                let ctrl = score[pos]; pos += 1;
                let cc: Option<u8> = match ctrl {
                    10 => Some(120), // all sounds off
                    11 => Some(123), // all notes off
                    12 => Some(126), // mono
                    13 => Some(127), // poly
                    14 => Some(121), // reset all controllers
                    _ => None,
                };
                if let Some(cc) = cc {
                    midi_ev.extend_from_slice(&[0xB0 | mid_ch, cc, 0]);
                }
            }
            4 => { // change controller
                if pos + 1 >= score.len() { return None; }
                let ctrl = score[pos]; pos += 1;
                let val = score[pos] & 0x7F; pos += 1;
                match ctrl {
                    0 => midi_ev.extend_from_slice(&[0xC0 | mid_ch, val]),     // program
                    1 => midi_ev.extend_from_slice(&[0xB0 | mid_ch, 0, val]),  // bank select
                    2 => midi_ev.extend_from_slice(&[0xB0 | mid_ch, 1, val]),  // modulation
                    3 => midi_ev.extend_from_slice(&[0xB0 | mid_ch, 7, val]),  // volume
                    4 => midi_ev.extend_from_slice(&[0xB0 | mid_ch, 10, val]), // pan
                    5 => midi_ev.extend_from_slice(&[0xB0 | mid_ch, 11, val]), // expression
                    6 => midi_ev.extend_from_slice(&[0xB0 | mid_ch, 91, val]), // reverb
                    7 => midi_ev.extend_from_slice(&[0xB0 | mid_ch, 93, val]), // chorus
                    8 => midi_ev.extend_from_slice(&[0xB0 | mid_ch, 64, val]), // sustain
                    9 => midi_ev.extend_from_slice(&[0xB0 | mid_ch, 67, val]), // soft pedal
                    _ => {} // unknown controller, skip
                }
            }
            5 => {}         // measure end — no payload, skip
            6 | 7 => break, // score end
            _ => break,     // unknown type, stop
        }

        if !midi_ev.is_empty() {
            write_var_len(&mut track, delta);
            track.extend(midi_ev);
            delta = 0;
        }

        if is_last {
            delta += read_delay(score, &mut pos)?;
        }
    }

    // End of track
    write_var_len(&mut track, delta);
    track.extend_from_slice(&[0xFF, 0x2F, 0x00]);

    // Assemble MIDI file
    let mut out = Vec::with_capacity(22 + track.len());
    out.extend_from_slice(b"MThd");
    out.extend_from_slice(&6u32.to_be_bytes());
    out.extend_from_slice(&0u16.to_be_bytes()); // format 0
    out.extend_from_slice(&1u16.to_be_bytes()); // 1 track
    out.extend_from_slice(&MIDI_PPQ.to_be_bytes());
    out.extend_from_slice(b"MTrk");
    out.extend_from_slice(&(track.len() as u32).to_be_bytes());
    out.extend(track);
    Some(out)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_mus() -> Vec<u8> {
        let mut d = vec![0u8; 17];
        d[0..4].copy_from_slice(b"MUS\x1a");
        d[4] = 1;   // score_len
        d[6] = 16;  // score_start
        d[16] = 0x60; // score end event: is_last=0, type=6, ch=0
        d
    }

    #[test]
    fn mus2midi_rejects_bad_magic() {
        let data = b"MIDI\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        assert!(mus2midi(data).is_none());
    }

    #[test]
    fn mus2midi_rejects_truncated() {
        assert!(mus2midi(b"MUS").is_none());
    }

    #[test]
    fn mus2midi_produces_valid_smf_header() {
        let out = mus2midi(&minimal_mus()).unwrap();
        // MThd
        assert_eq!(&out[0..4], b"MThd");
        // Header chunk length = 6
        assert_eq!(&out[4..8], &[0, 0, 0, 6]);
        // Format 0
        assert_eq!(&out[8..10], &[0, 0]);
        // 1 track
        assert_eq!(&out[10..12], &[0, 1]);
        // PPQ = 70 = 0x0046
        assert_eq!(&out[12..14], &[0, 70]);
        // MTrk
        assert_eq!(&out[14..18], b"MTrk");
    }

    #[test]
    fn mus2midi_score_end_produces_eot_meta() {
        let out = mus2midi(&minimal_mus()).unwrap();
        // Track ends with delta=0, FF 2F 00
        let last4 = &out[out.len() - 4..];
        assert_eq!(last4, &[0x00, 0xFF, 0x2F, 0x00]);
    }

    #[test]
    fn mus2midi_note_on_encoded_correctly() {
        // MUS: note-on ch0, note=60 (0x3C), vol=100 (0x64), then score end.
        // Event byte 0x10 = is_last=0, type=1 (note on), ch=0.
        // Next byte: 0x80|60 = 0xBC -> has_vol=1, note=60.
        // Vol byte: 100.
        // Score end: 0x60.
        let mut mus = vec![0u8; 20];
        mus[0..4].copy_from_slice(b"MUS\x1a");
        mus[4] = 4;   // score_len
        mus[6] = 16;  // score_start
        mus[16] = 0x10; // note on, ch 0, not last
        mus[17] = 0x80 | 60; // has_vol=1, note=60
        mus[18] = 100;        // volume
        mus[19] = 0x60;       // score end
        let out = mus2midi(&mus).unwrap();
        // Find note-on byte 0x90 (ch0 note on) after the tempo meta event.
        // Tempo meta is 7 bytes at track start (after MThd+MTrk headers = 22 bytes).
        // Note-on event: delta=0 (0x00), status=0x90, note=0x3C, vel=0x64.
        let track_body = &out[22..]; // skip MThd (14 bytes) + MTrk header (8 bytes)
        // Tempo: 00 FF 51 03 0F 42 40 (7 bytes)
        assert_eq!(&track_body[0..7], &[0x00, 0xFF, 0x51, 0x03, 0x07, 0xA1, 0x20]);
        // Note on: delta=0, ch=0
        assert_eq!(&track_body[7..11], &[0x00, 0x90, 0x3C, 0x64]);
    }

    #[test]
    fn mus2midi_percussion_maps_to_midi_channel_9() {
        // MUS channel 15 -> MIDI channel 9.
        // Note-on event on ch 15: ev = is_last=0 | type=1<<4 | ch=15 = 0b0_001_1111 = 0x1F
        let mut mus = vec![0u8; 20];
        mus[0..4].copy_from_slice(b"MUS\x1a");
        mus[4] = 4;
        mus[6] = 16;
        mus[16] = 0x1F; // note on, ch 15 (percussion)
        mus[17] = 0x80 | 35; // note=35 (bass drum), has_vol=1
        mus[18] = 100;
        mus[19] = 0x60; // score end
        let out = mus2midi(&mus).unwrap();
        let track_body = &out[22..];
        // Note-on on MIDI channel 9: 0x90 | 9 = 0x99
        assert_eq!(track_body[8], 0x99);
    }

    #[test]
    fn mus2midi_truncated_score_returns_none() {
        // Valid header but score has a note-on byte with no following note byte.
        // score_start=16, event 0x10 = note-on ch0, but score ends before the note byte.
        let mut mus = vec![0u8; 17];
        mus[0..4].copy_from_slice(b"MUS\x1a");
        mus[4] = 1;   // score_len = 1
        mus[6] = 16;  // score_start
        mus[16] = 0x10; // note-on event, ch 0 — requires another byte that doesn't exist
        assert!(mus2midi(&mus).is_none());
    }
}
