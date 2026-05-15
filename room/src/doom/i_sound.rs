#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::{c_char, c_int, c_void};

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
pub extern "C" fn I_InitSound(_use_sfx_prefix: Boolean) {}

#[no_mangle]
pub extern "C" fn I_ShutdownSound() {}

#[no_mangle]
pub extern "C" fn I_GetSfxLumpNum(_sfxinfo: *mut c_void) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn I_UpdateSound() {}

#[no_mangle]
pub extern "C" fn I_UpdateSoundParams(_channel: c_int, _vol: c_int, _sep: c_int) {}

#[no_mangle]
pub extern "C" fn I_StartSound(
    _sfxinfo: *mut c_void,
    _channel: c_int,
    _vol: c_int,
    _sep: c_int,
) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn I_StopSound(_channel: c_int) {}

#[no_mangle]
pub extern "C" fn I_SoundIsPlaying(_channel: c_int) -> c_int {
    0
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

extern "C" {
    fn M_BindVariable(name: *const c_char, variable: *mut c_void);
}

#[no_mangle]
pub extern "C" fn I_BindSoundVariables() {
    unsafe {
        M_BindVariable(
            c_bytes(b"snd_musicdevice\0").as_ptr(),
            &mut snd_musicdevice as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_sfxdevice\0").as_ptr(),
            &mut snd_sfxdevice as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_sbport\0").as_ptr(),
            &mut snd_sbport as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_sbirq\0").as_ptr(),
            &mut snd_sbirq as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_sbdma\0").as_ptr(),
            &mut snd_sbdma as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_mport\0").as_ptr(),
            &mut snd_mport as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_maxslicetime_ms\0").as_ptr(),
            &mut snd_maxslicetime_ms as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_musiccmd\0").as_ptr(),
            &mut snd_musiccmd as *mut *mut c_char as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_samplerate\0").as_ptr(),
            &mut snd_samplerate as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            c_bytes(b"snd_cachesize\0").as_ptr(),
            &mut snd_cachesize as *mut c_int as *mut c_void,
        );
    }
}
