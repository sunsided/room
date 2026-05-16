#![allow(non_upper_case_globals, non_snake_case)]

use crate::doom::m_config::M_BindVariable;
use crate::doom::sounds::SfxInfo;
use crate::doom::w_wad::{W_CacheLumpNum, W_CheckNumForName, W_LumpLength};
use crate::doom::z_zone::PU_CACHE;
use std::ffi::{c_char, c_int, c_uint, c_void};

use crate::types::Boolean;

#[no_mangle]
pub static mut snd_musicdevice: c_int = 3;

#[no_mangle]
pub static mut snd_sfxdevice: c_int = 3;

#[no_mangle]
pub static mut snd_samplerate: c_int = 44100;

#[no_mangle]
pub static mut snd_cachesize: c_int = 64 * 1024 * 1024;

#[no_mangle]
pub static mut snd_maxslicetime_ms: c_int = 28;

#[no_mangle]
pub static mut snd_sbport: c_int = 0;

#[no_mangle]
pub static mut snd_sbirq: c_int = 0;

#[no_mangle]
pub static mut snd_sbdma: c_int = 0;

#[no_mangle]
pub static mut snd_mport: c_int = 0;

#[no_mangle]
pub static mut snd_pitchshift: c_int = 0;

static mut EMPTY_CSTR: [c_char; 1] = [0];

#[no_mangle]
pub static mut snd_musiccmd: *mut c_char = unsafe { EMPTY_CSTR.as_mut_ptr() };

const fn c_bytes(s: &[u8]) -> &'static [c_char] {
    unsafe { &*(s as *const [u8] as *const [c_char]) }
}

#[no_mangle]
pub extern "C" fn I_InitSound(_use_sfx_prefix: Boolean) {
    crate::audio::AUDIO.with_borrow_mut(|audio| {
        if audio.is_none() {
            match crate::audio::AudioState::new() {
                Ok(state) => {
                    log::info!("Audio initialised");
                    *audio = Some(state);
                }
                Err(e) => log::warn!("Audio init failed (running silent): {e}"),
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn I_ShutdownSound() {
    crate::audio::AUDIO.with_borrow_mut(|audio| *audio = None);
}

#[no_mangle]
pub extern "C" fn I_GetSfxLumpNum(sfxinfo: *mut c_void) -> c_int {
    if sfxinfo.is_null() {
        return -1;
    }
    unsafe {
        let mut sfx = sfxinfo as *const SfxInfo;
        let mut depth = 0usize;
        while !(*sfx).link.is_null() && depth < 64 {
            sfx = (*sfx).link;
            depth += 1;
        }
        let mut lump_name = [0 as c_char; 9];
        lump_name[0] = b'D' as c_char;
        lump_name[1] = b'S' as c_char;
        for (i, &c) in (*sfx).name.iter().take(6).enumerate() {
            if c == 0 {
                break;
            }
            lump_name[2 + i] = c;
        }
        W_CheckNumForName(lump_name.as_ptr())
    }
}

#[no_mangle]
pub extern "C" fn I_UpdateSound() {}

#[no_mangle]
pub extern "C" fn I_UpdateSoundParams(channel: c_int, vol: c_int, sep: c_int) {
    if channel < 0 || channel >= 8 {
        return;
    }
    crate::audio::AUDIO.with_borrow(|audio| {
        if let Some(a) = audio.as_ref() {
            a.update_sound_params(channel as usize, vol, sep);
        }
    });
}

#[no_mangle]
pub extern "C" fn I_StartSound(
    sfxinfo: *mut c_void,
    channel: c_int,
    vol: c_int,
    sep: c_int,
) -> c_int {
    if sfxinfo.is_null() || channel < 0 || channel >= 8 {
        return -1;
    }
    unsafe {
        let sfx = sfxinfo as *const SfxInfo;
        let lumpnum = (*sfx).lumpnum;
        if lumpnum < 0 {
            return -1;
        }
        let lump_len = W_LumpLength(lumpnum as c_uint);
        if lump_len <= 0 {
            return -1;
        }
        let ptr = W_CacheLumpNum(lumpnum, PU_CACHE);
        if ptr.is_null() {
            return -1;
        }
        let data = std::slice::from_raw_parts(ptr as *const u8, lump_len as usize);
        let mut result = -1;
        crate::audio::AUDIO.with_borrow_mut(|audio| {
            if let Some(a) = audio.as_mut() {
                if a.start_sound(data, vol, sep, channel as usize) {
                    result = channel;
                }
            }
        });
        result
    }
}

#[no_mangle]
pub extern "C" fn I_StopSound(channel: c_int) {
    if channel < 0 || channel >= 8 {
        return;
    }
    crate::audio::AUDIO.with_borrow_mut(|audio| {
        if let Some(a) = audio.as_mut() {
            a.stop_sound(channel as usize);
        }
    });
}

#[no_mangle]
pub extern "C" fn I_SoundIsPlaying(channel: c_int) -> c_int {
    if channel < 0 || channel >= 8 {
        return 0;
    }
    let mut playing = 0;
    crate::audio::AUDIO.with_borrow(|audio| {
        if let Some(a) = audio.as_ref() {
            playing = a.is_playing(channel as usize) as c_int;
        }
    });
    playing
}

#[no_mangle]
pub extern "C" fn I_PrecacheSounds(_sounds: *mut c_void, _num_sounds: c_int) {}

#[no_mangle]
pub extern "C" fn I_InitMusic() {}

#[no_mangle]
pub extern "C" fn I_ShutdownMusic() {}

#[no_mangle]
pub extern "C" fn I_SetMusicVolume(_volume: c_int) {}

#[no_mangle]
pub extern "C" fn I_PauseSong() {}

#[no_mangle]
pub extern "C" fn I_ResumeSong() {}

#[no_mangle]
pub extern "C" fn I_RegisterSong(_data: *mut c_void, _len: c_int) -> *mut c_void {
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn I_UnRegisterSong(_handle: *mut c_void) {}

#[no_mangle]
pub extern "C" fn I_PlaySong(_handle: *mut c_void, _looping: c_int) {}

#[no_mangle]
pub extern "C" fn I_StopSong() {}

#[no_mangle]
pub extern "C" fn I_MusicIsPlaying() -> c_int {
    0
}


#[no_mangle]
pub extern "C" fn I_BindSoundVariables() {
    unsafe {
        M_BindVariable(
            c_bytes(b"snd_musicdevice\0").as_ptr() as *mut c_char,
            &mut snd_musicdevice as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_sfxdevice\0").as_ptr() as *mut c_char,
            &mut snd_sfxdevice as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_sbport\0").as_ptr() as *mut c_char,
            &mut snd_sbport as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_sbirq\0").as_ptr() as *mut c_char,
            &mut snd_sbirq as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_sbdma\0").as_ptr() as *mut c_char,
            &mut snd_sbdma as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_mport\0").as_ptr() as *mut c_char,
            &mut snd_mport as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_maxslicetime_ms\0").as_ptr() as *mut c_char,
            &mut snd_maxslicetime_ms as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_musiccmd\0").as_ptr() as *mut c_char,
            &mut snd_musiccmd as *mut *mut c_char as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_samplerate\0").as_ptr() as *mut c_char,
            &mut snd_samplerate as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_cachesize\0").as_ptr() as *mut c_char,
            &mut snd_cachesize as *mut c_int as *mut c_void,
        );
    }
}
