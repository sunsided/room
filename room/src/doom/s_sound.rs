#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::{c_char, c_int, c_uint, c_void};

use crate::types::Boolean;

use crate::doom::d_mode;
use crate::doom::m_fixed::{FRACBITS, FRACUNIT};
use crate::doom::sounds::{
    Mus, MusicInfo, S_InitSfxLinks, S_music, S_sfx, SfxInfo, NUMMUSIC, NUMSFX,
};
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
    // thinker_t prefix (3 pointers × 8 bytes = 24 bytes on 64-bit)
    _thinker_prev: *mut c_void,
    _thinker_next: *mut c_void,
    _thinker_fn: *mut c_void,
    // mobj_t positional fields
    pub x: c_int,
    pub y: c_int,
    _z: c_int,
    _pad0: u32,
    _snext: *mut c_void,
    _sprev: *mut c_void,
    pub angle: u32,
}

// doomstat.rs
use crate::doom::doomstat::gamemode;

// g_game.rs
use crate::doom::g_game::{consoleplayer, gameepisode, gamemap, players};

// i_sound.rs
use crate::doom::i_sound::{
    I_GetSfxLumpNum, I_MusicIsPlaying, I_PauseSong, I_PlaySong, I_PrecacheSounds, I_RegisterSong,
    I_ResumeSong, I_SetMusicVolume, I_ShutdownMusic, I_ShutdownSound, I_SoundIsPlaying,
    I_StartSound, I_StopSong, I_StopSound, I_UnRegisterSong, I_UpdateSound, I_UpdateSoundParams,
};

// i_system.rs
use crate::doom::i_system::I_AtExit;

// i_sound.rs
use crate::doom::i_sound::snd_musicdevice;

// z_zone.rs
use crate::doom::z_zone::Z_Malloc;

// r_main.rs
use crate::doom::r_main::R_PointToAngle2;

// m_fixed.rs
use crate::doom::m_fixed::FixedMul;

// w_wad.rs
use crate::doom::w_wad::{W_CacheLumpNum, W_GetNumForName, W_LumpLength, W_ReleaseLumpNum};

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

        I_PrecacheSounds(
            std::ptr::addr_of_mut!(S_sfx[0]) as *mut c_void,
            NUMSFX as c_int,
        );

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
            (*std::ptr::addr_of_mut!(S_sfx[0]).add(i)).lumpnum = -1;
            (*std::ptr::addr_of_mut!(S_sfx[0]).add(i)).usefulness = -1;
        }

        I_AtExit(S_Shutdown, Boolean::TRUE);
    }
}

#[no_mangle]
pub extern "C" fn S_Shutdown() {
    I_ShutdownSound();
    I_ShutdownMusic();
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
            mnum = Mus::Runnin as c_int + gamemap - 1;
        } else {
            let spmus: [c_int; 9] = [
                Mus::E3m4 as c_int,
                Mus::E3m2 as c_int,
                Mus::E3m3 as c_int,
                Mus::E1m5 as c_int,
                Mus::E2m7 as c_int,
                Mus::E2m4 as c_int,
                Mus::E2m6 as c_int,
                Mus::E2m5 as c_int,
                Mus::E1m9 as c_int,
            ];

            if gameepisode < 4 {
                mnum = Mus::E1m1 as c_int + (gameepisode - 1) * 9 + gamemap - 1;
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

        let sfx = &mut *std::ptr::addr_of_mut!(S_sfx[0]).offset(sfx_id as isize);

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

        let player_mo =
            (*std::ptr::addr_of!(players[0]).offset(consoleplayer as isize)).mo as *mut MobjStub;
        if !origin.is_null() && origin != player_mo {
            let listener = player_mo;
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
            sfx.lumpnum = I_GetSfxLumpNum(sfx as *mut SfxInfo as *mut c_void);
        }

        (*channels.offset(cnum as isize)).handle =
            I_StartSound(sfx as *mut SfxInfo as *mut c_void, cnum, volume, sep);
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
    if !(0..=127).contains(&volume) {
        return;
    }

    I_SetMusicVolume(volume);
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

        if musicnum == Mus::Intro as c_int
            && (snd_musicdevice == SNDDEVICE_ADLIB || snd_musicdevice == SNDDEVICE_SB)
        {
            musicnum = Mus::Introa as c_int;
        }

        if musicnum <= Mus::None as c_int || musicnum >= NUMMUSIC as c_int {
            return;
        }

        let music = &mut *std::ptr::addr_of_mut!(S_music[0]).offset(musicnum as isize);

        if mus_playing == music {
            return;
        }

        S_StopMusic();

        if music.lumpnum == 0 {
            let mut namebuf: [c_char; 9] = [0; 9];
            let name_str = std::ffi::CStr::from_ptr(music.name).to_string_lossy();
            c_write!(namebuf, "d_{}", name_str);
            music.lumpnum = W_GetNumForName(namebuf.as_ptr() as *const c_char);
        }

        music.data = W_CacheLumpNum(music.lumpnum, PU_STATIC);

        let len = W_LumpLength(music.lumpnum as c_uint);
        let handle = I_RegisterSong(music.data, len);
        music.handle = handle;
        I_PlaySong(handle, looping);

        mus_playing = music;
    }
}

#[no_mangle]
pub extern "C" fn S_MusicPlaying() -> c_int {
    I_MusicIsPlaying()
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
