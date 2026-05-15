#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::{c_char, c_int, c_void};

use crate::types::Boolean;

use crate::doom::d_mode;
use crate::doom::m_fixed::{FRACBITS, FRACUNIT};
use crate::doom::sounds::{MusicInfo, S_InitSfxLinks, S_music, S_sfx, SfxInfo, NUMMUSIC, NUMSFX};
use crate::doom::tables::{finesine, ANGLETOFINESHIFT};
use crate::doom::z_zone::PU_STATIC;

const S_CLIPPING_DIST: c_int = 1200 * FRACUNIT;
const S_CLOSE_DIST: c_int = 200 * FRACUNIT;
const S_ATTENUATOR: c_int = (S_CLIPPING_DIST - S_CLOSE_DIST) >> FRACBITS;
const S_STEREO_SWING: c_int = 96 * FRACUNIT;
const NORM_PITCH: c_int = 128;
const NORM_PRIORITY: c_int = 64;
const NORM_SEP: c_int = 128;

const MAXPLAYERS: usize = 4;

const mus_None: c_int = 0;
const mus_e1m1: c_int = 1;
const mus_e1m2: c_int = 2;
const mus_e1m3: c_int = 3;
const mus_e1m4: c_int = 4;
const mus_e1m5: c_int = 5;
const mus_e1m6: c_int = 6;
const mus_e1m7: c_int = 7;
const mus_e1m8: c_int = 8;
const mus_e1m9: c_int = 9;
const mus_e2m1: c_int = 10;
const mus_e2m2: c_int = 11;
const mus_e2m3: c_int = 12;
const mus_e2m4: c_int = 13;
const mus_e2m5: c_int = 14;
const mus_e2m6: c_int = 15;
const mus_e2m7: c_int = 16;
const mus_e2m8: c_int = 17;
const mus_e2m9: c_int = 18;
const mus_e3m1: c_int = 19;
const mus_e3m2: c_int = 20;
const mus_e3m3: c_int = 21;
const mus_e3m4: c_int = 22;
const mus_e3m5: c_int = 23;
const mus_e3m6: c_int = 24;
const mus_e3m7: c_int = 25;
const mus_e3m8: c_int = 26;
const mus_e3m9: c_int = 27;
const mus_inter: c_int = 28;
const mus_intro: c_int = 29;
const mus_bunny: c_int = 30;
const mus_victor: c_int = 31;
const mus_introa: c_int = 32;
const mus_runnin: c_int = 33;
const mus_stalks: c_int = 34;
const mus_countd: c_int = 35;
const mus_betwee: c_int = 36;
const mus_doom: c_int = 37;
const mus_the_da: c_int = 38;
const mus_shawn: c_int = 39;
const mus_ddtblu: c_int = 40;
const mus_in_cit: c_int = 41;
const mus_dead: c_int = 42;
const mus_stlks2: c_int = 43;
const mus_theda2: c_int = 44;
const mus_doom2: c_int = 45;
const mus_ddtbl2: c_int = 46;
const mus_runni2: c_int = 47;
const mus_dead2: c_int = 48;
const mus_stlks3: c_int = 49;
const mus_romero: c_int = 50;
const mus_shawn2: c_int = 51;
const mus_messag: c_int = 52;
const mus_count2: c_int = 53;
const mus_ddtbl3: c_int = 54;
const mus_ampie: c_int = 55;
const mus_theda3: c_int = 56;
const mus_adrian: c_int = 57;
const mus_messg2: c_int = 58;
const mus_romer2: c_int = 59;
const mus_tense: c_int = 60;
const mus_shawn3: c_int = 61;
const mus_openin: c_int = 62;
const mus_evil: c_int = 63;
const mus_ultima: c_int = 64;
const mus_read_m: c_int = 65;
const mus_dm2ttl: c_int = 66;
const mus_dm2int: c_int = 67;

const SNDDEVICE_ADLIB: c_int = 2;
const SNDDEVICE_SB: c_int = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct channel_t {
    pub sfxinfo: *mut SfxInfo,
    pub origin: *mut MobjStub,
    pub handle: c_int,
}

#[repr(C)]
pub struct MobjStub {
    pub x: c_int,
    pub y: c_int,
    pub angle: u32,
}

#[repr(C)]
struct PlayerStub {
    mo: *mut MobjStub,
}

extern "C" {
    static mut gamemode: c_int;
    static mut gameepisode: c_int;
    static mut gamemap: c_int;
    static mut consoleplayer: c_int;
    static mut players: [PlayerStub; MAXPLAYERS];
    static mut snd_musicdevice: c_int;

    fn I_PrecacheSounds(sounds: *mut SfxInfo, num_sounds: c_int);
    fn I_ShutdownSound();
    fn I_ShutdownMusic();
    fn I_SoundIsPlaying(handle: c_int) -> c_int;
    fn I_StopSound(handle: c_int);
    fn I_GetSfxLumpNum(sfxinfo: *mut SfxInfo) -> c_int;
    fn I_StartSound(sfxinfo: *mut SfxInfo, channel: c_int, vol: c_int, sep: c_int) -> c_int;
    fn I_UpdateSound();
    fn I_UpdateSoundParams(handle: c_int, vol: c_int, sep: c_int);
    fn I_SetMusicVolume(volume: c_int);
    fn I_PauseSong();
    fn I_ResumeSong();
    fn I_RegisterSong(data: *mut c_void, len: c_int) -> *mut c_void;
    fn I_UnRegisterSong(handle: *mut c_void);
    fn I_PlaySong(handle: *mut c_void, looping: c_int);
    fn I_StopSong();
    fn I_MusicIsPlaying() -> c_int;
    fn I_AtExit(func: extern "C" fn(), run_on_error: Boolean);
    fn Z_Malloc(size: c_int, tag: c_int, user: *mut c_void) -> *mut c_void;
    fn R_PointToAngle2(x1: c_int, y1: c_int, x2: c_int, y2: c_int) -> u32;
    fn FixedMul(a: c_int, b: c_int) -> c_int;
    fn W_GetNumForName(name: *const c_char) -> c_int;
    fn W_CacheLumpNum(num: c_int, tag: c_int) -> *mut c_void;
    fn W_ReleaseLumpNum(num: c_int);
    fn W_LumpLength(num: c_int) -> c_int;
}

use crate::c_write;

static mut channels: *mut channel_t = std::ptr::null_mut();

#[no_mangle]
pub static mut sfxVolume: c_int = 8;

#[no_mangle]
pub static mut musicVolume: c_int = 8;

#[no_mangle]
pub static mut snd_channels: c_int = 8;

#[no_mangle]
pub static mut snd_SfxVolume: c_int = 8;

#[no_mangle]
pub static mut snd_MusicVolume: c_int = 8;

#[no_mangle]
pub static mut mus_paused: c_int = 0;

static mut mus_playing: *mut MusicInfo = std::ptr::null_mut();

#[no_mangle]
pub extern "C" fn S_Init(sfx_volume: c_int, music_volume: c_int) {
    unsafe {
        S_InitSfxLinks();

        I_PrecacheSounds(S_sfx.as_mut_ptr(), NUMSFX as c_int);

        S_SetSfxVolume(sfx_volume);
        S_SetMusicVolume(music_volume);

        channels = Z_Malloc(
            (snd_channels as usize * std::mem::size_of::<channel_t>()) as c_int,
            PU_STATIC,
            std::ptr::null_mut(),
        ) as *mut channel_t;

        for i in 0..snd_channels {
            (*channels.offset(i as isize)).sfxinfo = std::ptr::null_mut();
        }

        mus_paused = 0;

        for i in 1..NUMSFX {
            (*S_sfx.as_mut_ptr().add(i)).lumpnum = -1;
            (*S_sfx.as_mut_ptr().add(i)).usefulness = -1;
        }

        I_AtExit(S_Shutdown, Boolean::TRUE);
    }
}

#[no_mangle]
pub extern "C" fn S_Shutdown() {
    unsafe {
        I_ShutdownSound();
        I_ShutdownMusic();
    }
}

unsafe fn S_StopChannel(cnum: c_int) {
    let c = &mut *channels.offset(cnum as isize);

    if !c.sfxinfo.is_null() {
        if I_SoundIsPlaying(c.handle) != 0 {
            I_StopSound(c.handle);
        }

        for i in 0..snd_channels {
            if cnum != i
                && !channels.offset(i as isize).is_null()
                && (*channels.offset(i as isize)).sfxinfo == c.sfxinfo
            {
                break;
            }
        }

        (*c.sfxinfo).usefulness -= 1;
        c.sfxinfo = std::ptr::null_mut();
    }
}

#[no_mangle]
pub extern "C" fn S_Start() {
    unsafe {
        for cnum in 0..snd_channels {
            if !(*channels.offset(cnum as isize)).sfxinfo.is_null() {
                S_StopChannel(cnum);
            }
        }

        mus_paused = 0;

        let mnum: c_int;

        if gamemode == d_mode::commercial {
            mnum = mus_runnin + gamemap - 1;
        } else {
            let spmus: [c_int; 9] = [
                mus_e3m4, mus_e3m2, mus_e3m3, mus_e1m5, mus_e2m7, mus_e2m4, mus_e2m6, mus_e2m5,
                mus_e1m9,
            ];

            if gameepisode < 4 {
                mnum = mus_e1m1 + (gameepisode - 1) * 9 + gamemap - 1;
            } else {
                mnum = spmus[(gamemap - 1) as usize];
            }
        }

        S_ChangeMusic(mnum, 1);
    }
}

#[no_mangle]
pub extern "C" fn S_StopSound(origin: *mut MobjStub) {
    unsafe {
        for cnum in 0..snd_channels {
            let ch = *channels.offset(cnum as isize);
            if !ch.sfxinfo.is_null() && ch.origin == origin {
                S_StopChannel(cnum);
                break;
            }
        }
    }
}

unsafe fn S_GetChannel(origin: *mut MobjStub, sfxinfo: *mut SfxInfo) -> c_int {
    let mut cnum: c_int = 0;

    for cnum_search in 0..snd_channels {
        let ch = *channels.offset(cnum_search as isize);
        if ch.sfxinfo.is_null() {
            cnum = cnum_search;
            break;
        } else if !origin.is_null() && ch.origin == origin {
            S_StopChannel(cnum_search);
            cnum = cnum_search;
            break;
        }
    }

    if cnum == snd_channels {
        for cnum_search in 0..snd_channels {
            let ch = *channels.offset(cnum_search as isize);
            if !ch.sfxinfo.is_null() && (*ch.sfxinfo).priority >= (*sfxinfo).priority {
                cnum = cnum_search;
                break;
            }
        }

        if cnum == snd_channels {
            return -1;
        } else {
            S_StopChannel(cnum);
        }
    }

    let c = &mut *channels.offset(cnum as isize);
    c.sfxinfo = sfxinfo;
    c.origin = origin;

    cnum
}

unsafe fn S_AdjustSoundParams(
    listener: *mut MobjStub,
    source: *mut MobjStub,
    vol: *mut c_int,
    sep: *mut c_int,
) -> c_int {
    let adx = (*listener).x.wrapping_sub((*source).x).wrapping_abs();
    let ady = (*listener).y.wrapping_sub((*source).y).wrapping_abs();

    let approx_dist = adx
        .wrapping_add(ady)
        .wrapping_sub((if adx < ady { adx } else { ady }) >> 1);

    if gamemap != 8 && approx_dist > S_CLIPPING_DIST {
        return 0;
    }

    let mut angle = R_PointToAngle2((*listener).x, (*listener).y, (*source).x, (*source).y);

    if angle > (*listener).angle {
        angle -= (*listener).angle;
    } else {
        angle = angle.wrapping_add(0xffffffff_u32 - (*listener).angle);
    }

    angle >>= ANGLETOFINESHIFT;

    *sep = 128 - (FixedMul(S_STEREO_SWING, finesine[angle as usize]) >> FRACBITS);

    if approx_dist < S_CLOSE_DIST {
        *vol = snd_SfxVolume;
    } else if gamemap == 8 {
        let mut clipped_dist = approx_dist;
        if clipped_dist > S_CLIPPING_DIST {
            clipped_dist = S_CLIPPING_DIST;
        }

        *vol = 15
            + ((snd_SfxVolume - 15) * ((S_CLIPPING_DIST - clipped_dist) >> FRACBITS))
                / S_ATTENUATOR;
    } else {
        *vol = (snd_SfxVolume * ((S_CLIPPING_DIST - approx_dist) >> FRACBITS)) / S_ATTENUATOR;
    }

    (*vol > 0) as c_int
}

#[no_mangle]
pub extern "C" fn S_StartSound(origin_p: *mut c_void, sfx_id: c_int) {
    unsafe {
        let origin = origin_p as *mut MobjStub;
        let mut volume = snd_SfxVolume;

        if sfx_id < 1 || sfx_id > NUMSFX as c_int {
            // Bogus sound id — skip
            return;
        }

        let sfx = &mut *S_sfx.as_mut_ptr().offset(sfx_id as isize);

        if !sfx.link.is_null() {
            volume += sfx.volume;

            if volume < 1 {
                return;
            }

            if volume > snd_SfxVolume {
                volume = snd_SfxVolume;
            }
        }

        let mut sep: c_int = NORM_SEP;

        if !origin.is_null() && origin != (*players.as_ptr().offset(consoleplayer as isize)).mo {
            let listener = (*players.as_ptr().offset(consoleplayer as isize)).mo;
            let rc = S_AdjustSoundParams(listener, origin, &mut volume, &mut sep);

            if (*origin).x == (*listener).x && (*origin).y == (*listener).y {
                sep = NORM_SEP;
            }

            if rc == 0 {
                return;
            }
        }

        S_StopSound(origin);

        let cnum = S_GetChannel(origin, sfx);

        if cnum < 0 {
            return;
        }

        if sfx.usefulness < 0 {
            sfx.usefulness = 1;
        }
        sfx.usefulness += 1;

        if sfx.lumpnum < 0 {
            sfx.lumpnum = I_GetSfxLumpNum(sfx);
        }

        (*channels.offset(cnum as isize)).handle = I_StartSound(sfx, cnum, volume, sep);
    }
}

#[no_mangle]
pub extern "C" fn S_PauseSound() {
    unsafe {
        if !mus_playing.is_null() && mus_paused == 0 {
            I_PauseSong();
            mus_paused = 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn S_ResumeSound() {
    unsafe {
        if !mus_playing.is_null() && mus_paused != 0 {
            I_ResumeSong();
            mus_paused = 0;
        }
    }
}

#[no_mangle]
pub extern "C" fn S_UpdateSounds(listener: *mut MobjStub) {
    unsafe {
        I_UpdateSound();

        for cnum in 0..snd_channels {
            let c = &mut *channels.offset(cnum as isize);
            let sfx = c.sfxinfo;

            if !sfx.is_null() {
                if I_SoundIsPlaying(c.handle) != 0 {
                    let mut volume = snd_SfxVolume;
                    let mut sep = NORM_SEP;

                    if !(*sfx).link.is_null() {
                        volume += (*sfx).volume;
                        if volume < 1 {
                            S_StopChannel(cnum);
                            continue;
                        } else if volume > snd_SfxVolume {
                            volume = snd_SfxVolume;
                        }
                    }

                    if !c.origin.is_null() && listener != c.origin {
                        let audible =
                            S_AdjustSoundParams(listener, c.origin, &mut volume, &mut sep);

                        if audible == 0 {
                            S_StopChannel(cnum);
                        } else {
                            I_UpdateSoundParams(c.handle, volume, sep);
                        }
                    }
                } else {
                    S_StopChannel(cnum);
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn S_SetMusicVolume(volume: c_int) {
    unsafe {
        if !(0..=127).contains(&volume) {
            return;
        }

        I_SetMusicVolume(volume);
    }
}

#[no_mangle]
pub extern "C" fn S_SetSfxVolume(volume: c_int) {
    unsafe {
        if !(0..=127).contains(&volume) {
            return;
        }

        snd_SfxVolume = volume;
    }
}

#[no_mangle]
pub extern "C" fn S_StartMusic(m_id: c_int) {
    S_ChangeMusic(m_id, 0);
}

#[no_mangle]
pub extern "C" fn S_ChangeMusic(musicnum: c_int, looping: c_int) {
    unsafe {
        let mut musicnum = musicnum;

        if musicnum == mus_intro
            && (snd_musicdevice == SNDDEVICE_ADLIB || snd_musicdevice == SNDDEVICE_SB)
        {
            musicnum = mus_introa;
        }

        if musicnum <= mus_None || musicnum >= NUMMUSIC as c_int {
            return;
        }

        let music = &mut *S_music.as_mut_ptr().offset(musicnum as isize);

        if mus_playing == music {
            return;
        }

        S_StopMusic();

        if music.lumpnum == 0 {
            let mut namebuf: [c_char; 9] = [0; 9];
            let name_str = std::ffi::CStr::from_ptr(music.name).to_string_lossy();
            c_write!(namebuf, "d_{}", name_str);
            music.lumpnum = W_GetNumForName(namebuf.as_ptr());
        }

        music.data = W_CacheLumpNum(music.lumpnum, PU_STATIC);

        let len = W_LumpLength(music.lumpnum);
        let handle = I_RegisterSong(music.data, len);
        music.handle = handle;
        I_PlaySong(handle, looping);

        mus_playing = music;
    }
}

#[no_mangle]
pub extern "C" fn S_MusicPlaying() -> c_int {
    unsafe { I_MusicIsPlaying() }
}

#[no_mangle]
pub extern "C" fn S_StopMusic() {
    unsafe {
        if !mus_playing.is_null() {
            let m = &mut *mus_playing;

            if mus_paused != 0 {
                I_ResumeSong();
            }

            I_StopSong();
            I_UnRegisterSong(m.handle);
            W_ReleaseLumpNum(m.lumpnum);
            m.data = std::ptr::null_mut();
            mus_playing = std::ptr::null_mut();
        }
    }
}
