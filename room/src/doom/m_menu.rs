//! Rust port of vendor/doomgeneric/m_menu.c.
//!
//! DOOM selection menu, options, episode etc.

#![allow(non_upper_case_globals, non_snake_case)]

use std::ffi::{c_char, c_int, c_void};
use std::ptr;

use crate::types::Boolean;

use super::d_event::event_t;
use super::d_mode;
use super::d_player::M_Menu_SetPlayerMessage;
use super::doomstat::{gamemission, gamemode, gameversion};
use crate::{c_write, i_error};

fn logical_gamemission() -> c_int {
    unsafe {
        if gamemission == d_mode::pack_chex {
            d_mode::doom
        } else if gamemission == d_mode::pack_hacx {
            d_mode::doom2
        } else {
            gamemission
        }
    }
}

const LINEHEIGHT: c_int = 16;
const SKULLXOFF: c_int = -32;

const HU_FONTSTART: c_int = b'!' as c_int;
const HU_FONTEND: c_int = b'_' as c_int;
const HU_FONTSIZE: usize = (HU_FONTEND - HU_FONTSTART + 1) as usize;

use crate::doom::i_video::{SCREENHEIGHT, SCREENWIDTH};

const SAVESTRINGSIZE: usize = 24;
const MAXPLAYERS: usize = 4;

const EV_KEYDOWN: c_int = 0;
const EV_KEYUP: c_int = 1;
const EV_MOUSE: c_int = 2;
const EV_JOYSTICK: c_int = 3;
const EV_QUIT: c_int = 4;

const KEY_ESCAPE: c_int = 27;
const KEY_ENTER: c_int = 13;
const KEY_BACKSPACE: c_int = 0x7f;
const KEY_PAUSE: c_int = 0xff;
const KEY_CAPSLOCK: c_int = 0x80 + 0x3a;
const KEY_NUMLOCK: c_int = 0x80 + 0x45;
const KEY_SCRLCK: c_int = 0x80 + 0x46;

const fn mi(
    status: i16,
    name: &[u8],
    routine: Option<extern "C" fn(c_int)>,
    alpha: u8,
) -> menuitem_t {
    let mut n = [0i8; 10];
    let mut i = 0;
    while i < name.len() && i < 10 {
        n[i] = name[i] as c_char;
        i += 1;
    }
    menuitem_t {
        status,
        name: n,
        routine,
        alphaKey: alpha as c_char,
    }
}

const fn make_gamma_msg(s: &str) -> [c_char; 26] {
    let bytes = s.as_bytes();
    let mut arr = [0i8; 26];
    let mut i = 0;
    while i < bytes.len() {
        arr[i] = bytes[i] as c_char;
        i += 1;
    }
    arr
}

#[repr(C)]
pub struct menuitem_t {
    pub status: i16,
    pub name: [c_char; 10],
    pub routine: Option<extern "C" fn(c_int)>,
    pub alphaKey: c_char,
}

#[repr(C)]
pub struct menu_t {
    pub numitems: i16,
    pub prevMenu: *mut menu_t,
    pub menuitems: *mut menuitem_t,
    pub routine: Option<extern "C" fn()>,
    pub x: i16,
    pub y: i16,
    pub lastOn: i16,
}

const _: () = assert!(
    std::mem::size_of::<menuitem_t>() == 32,
    "menuitem_t size mismatch"
);
const _: () = assert!(std::mem::size_of::<menu_t>() == 40, "menu_t size mismatch");

#[repr(C)]
struct patch_stub {
    width: i16,
    height: i16,
}

extern "C" {
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut c_void) -> usize;
    fn fclose(stream: *mut c_void) -> c_int;
    fn toupper(c: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

use crate::doom::am_map::automapactive;
use crate::doom::d_loop::gametic;
use crate::doom::d_main::devparm;
use crate::doom::dstrings::{doom1_endmsg, doom2_endmsg};
use crate::doom::g_game::{
    demoplayback, gamestate, netgame, testcontrols, usergame, G_DeferedInitNew, G_LoadGame,
    G_SaveGame, G_ScreenShot,
};
use crate::doom::hu_stuff::{chat_on, hu_font, message_dontfuckwithme};
use crate::doom::i_input::vanilla_keyboard_mapping;
use crate::doom::i_system::I_Quit;
use crate::doom::i_timer::{I_GetTime, I_WaitVBL};
use crate::doom::i_video::{usegamma, I_SetPalette};
use crate::doom::m_misc::M_StringCopy;
use crate::doom::p_saveg::P_SaveGameFile;
use crate::doom::r_main::R_SetViewSize;
use crate::doom::s_sound::{S_SetMusicVolume, S_SetSfxVolume, S_StartSound};
use crate::doom::sounds::Sfx;
use crate::doom::v_video::{patch_t, V_DrawPatchDirect};
use crate::doom::w_wad::W_CacheLumpName;

#[no_mangle]
pub static mut mouseSensitivity: c_int = 5;
#[no_mangle]
pub static mut showMessages: c_int = 1;
#[no_mangle]
pub static mut detailLevel: c_int = 0;
#[no_mangle]
pub static mut screenblocks: c_int = 10;

static mut screenSize: c_int = 0;
static mut quickSaveSlot: c_int = -1;
static mut messageToPrint: c_int = 0;
static mut messageString: *mut c_char = ptr::null_mut();
static mut messx: c_int = 0;
static mut messy: c_int = 0;
static mut messageLastMenuActive: c_int = 0;
static mut messageNeedsInput: c_int = 0;
static mut messageRoutine: Option<extern "C" fn(c_int)> = None;

#[no_mangle]
pub static mut gammamsg: [[c_char; 26]; 5] = [
    make_gamma_msg("Gamma correction OFF"),
    make_gamma_msg("Gamma correction level 1"),
    make_gamma_msg("Gamma correction level 2"),
    make_gamma_msg("Gamma correction level 3"),
    make_gamma_msg("Gamma correction level 4"),
];

static mut saveStringEnter: c_int = 0;
static mut saveSlot: c_int = 0;
static mut saveCharIndex: c_int = 0;
static mut saveOldString: [c_char; SAVESTRINGSIZE] = [0; SAVESTRINGSIZE];

#[no_mangle]
pub static mut inhelpscreens: c_int = 0;
#[no_mangle]
pub static mut menuactive: c_int = 0;

static mut savegamestrings: [[c_char; SAVESTRINGSIZE]; 10] = [[0; SAVESTRINGSIZE]; 10];
static mut endstring: [c_char; 160] = [0; 160];

static mut itemOn: i16 = 0;
static mut skullAnimCounter: i16 = 0;
static mut whichSkull: i16 = 0;

static mut skullName: [*const c_char; 2] = [c"M_SKULL1".as_ptr(), c"M_SKULL2".as_ptr()];

#[no_mangle]
pub static mut currentMenu: *mut menu_t = ptr::null_mut();

const newgame: usize = 0;
const options: usize = 1;
const loadgame: usize = 2;
const savegame: usize = 3;
const readthis: usize = 4;
const quitdoom: usize = 5;
const main_end: usize = 6;

const ep1: usize = 0;
const ep2: usize = 1;
const ep3: usize = 2;
const ep4: usize = 3;
const ep_end: usize = 4;

const killthings: usize = 0;
const toorough: usize = 1;
const hurtme: usize = 2;
const violence: usize = 3;
const nightmare: usize = 4;
const newg_end: usize = 5;

const endgame: usize = 0;
const messages: usize = 1;
const detail: usize = 2;
const scrnsize: usize = 3;
const mousesens: usize = 5;
const soundvol: usize = 7;
const opt_end: usize = 8;

const sfx_vol: usize = 0;
const music_vol: usize = 2;
const sound_end: usize = 4;

const load_end: usize = 6;

const read1_end: usize = 1;
const read2_end: usize = 1;

static mut MainMenu: [menuitem_t; 6] = [
    mi(1, b"M_NGAME\0\0\0", Some(M_NewGame), b'n'),
    mi(1, b"M_OPTION\0\0", Some(M_Options), b'o'),
    mi(1, b"M_LOADG\0\0\0", Some(M_LoadGame), b'l'),
    mi(1, b"M_SAVEG\0\0\0", Some(M_SaveGame), b's'),
    mi(1, b"M_RDTHIS\0\0", Some(M_ReadThis), b'r'),
    mi(1, b"M_QUITG\0\0\0", Some(M_QuitDOOM), b'q'),
];

static mut EpisodeMenu: [menuitem_t; 4] = [
    mi(1, b"M_EPI1\0\0\0\0", Some(M_Episode), b'k'),
    mi(1, b"M_EPI2\0\0\0\0", Some(M_Episode), b't'),
    mi(1, b"M_EPI3\0\0\0\0", Some(M_Episode), b'i'),
    mi(1, b"M_EPI4\0\0\0\0", Some(M_Episode), b't'),
];

static mut NewGameMenu: [menuitem_t; 5] = [
    mi(1, b"M_JKILL\0\0\0", Some(M_ChooseSkill), b'i'),
    mi(1, b"M_ROUGH\0\0\0", Some(M_ChooseSkill), b'h'),
    mi(1, b"M_HURT\0\0\0\0", Some(M_ChooseSkill), b'h'),
    mi(1, b"M_ULTRA\0\0\0", Some(M_ChooseSkill), b'u'),
    mi(1, b"M_NMARE\0\0\0", Some(M_ChooseSkill), b'n'),
];

static mut OptionsMenu: [menuitem_t; 8] = [
    mi(1, b"M_ENDGAM\0\0", Some(M_EndGame), b'e'),
    mi(1, b"M_MESSG\0\0\0", Some(M_ChangeMessages), b'm'),
    mi(1, b"M_DETAIL\0\0", Some(M_ChangeDetail), b'g'),
    mi(2, b"M_SCRNSZ\0\0", Some(M_SizeDisplay), b's'),
    mi(-1, b"", None, 0),
    mi(2, b"M_MSENS\0\0\0", Some(M_ChangeSensitivity), b'm'),
    mi(-1, b"", None, 0),
    mi(1, b"M_SVOL\0\0\0\0", Some(M_Sound), b's'),
];

static mut ReadMenu1: [menuitem_t; 1] = [mi(1, b"", Some(M_ReadThis2), 0)];

static mut ReadMenu2: [menuitem_t; 1] = [mi(1, b"", Some(M_FinishReadThis), 0)];

static mut SoundMenu: [menuitem_t; 4] = [
    mi(2, b"M_SFXVOL\0\0", Some(M_SfxVol), b's'),
    mi(-1, b"", None, 0),
    mi(2, b"M_MUSVOL\0\0", Some(M_MusicVol), b'm'),
    mi(-1, b"", None, 0),
];

static mut LoadMenu: [menuitem_t; 6] = [
    mi(1, b"", Some(M_LoadSelect), b'1'),
    mi(1, b"", Some(M_LoadSelect), b'2'),
    mi(1, b"", Some(M_LoadSelect), b'3'),
    mi(1, b"", Some(M_LoadSelect), b'4'),
    mi(1, b"", Some(M_LoadSelect), b'5'),
    mi(1, b"", Some(M_LoadSelect), b'6'),
];

static mut SaveMenu: [menuitem_t; 6] = [
    mi(1, b"", Some(M_SaveSelect), b'1'),
    mi(1, b"", Some(M_SaveSelect), b'2'),
    mi(1, b"", Some(M_SaveSelect), b'3'),
    mi(1, b"", Some(M_SaveSelect), b'4'),
    mi(1, b"", Some(M_SaveSelect), b'5'),
    mi(1, b"", Some(M_SaveSelect), b'6'),
];

static mut MainDef: menu_t = menu_t {
    numitems: main_end as i16,
    prevMenu: ptr::null_mut(),
    menuitems: ptr::null_mut(),
    routine: Some(M_DrawMainMenu),
    x: 97,
    y: 64,
    lastOn: 0,
};

static mut EpiDef: menu_t = menu_t {
    numitems: ep_end as i16,
    prevMenu: ptr::null_mut(),
    menuitems: ptr::null_mut(),
    routine: Some(M_DrawEpisode),
    x: 48,
    y: 63,
    lastOn: 0,
};

static mut NewDef: menu_t = menu_t {
    numitems: newg_end as i16,
    prevMenu: ptr::null_mut(),
    menuitems: ptr::null_mut(),
    routine: Some(M_DrawNewGame),
    x: 48,
    y: 63,
    lastOn: 2,
};

static mut OptionsDef: menu_t = menu_t {
    numitems: opt_end as i16,
    prevMenu: ptr::null_mut(),
    menuitems: ptr::null_mut(),
    routine: Some(M_DrawOptions),
    x: 60,
    y: 37,
    lastOn: 0,
};

static mut ReadDef1: menu_t = menu_t {
    numitems: read1_end as i16,
    prevMenu: ptr::null_mut(),
    menuitems: ptr::null_mut(),
    routine: Some(M_DrawReadThis1),
    x: 280,
    y: 185,
    lastOn: 0,
};

static mut ReadDef2: menu_t = menu_t {
    numitems: read2_end as i16,
    prevMenu: ptr::null_mut(),
    menuitems: ptr::null_mut(),
    routine: Some(M_DrawReadThis2),
    x: 330,
    y: 175,
    lastOn: 0,
};

static mut SoundDef: menu_t = menu_t {
    numitems: sound_end as i16,
    prevMenu: ptr::null_mut(),
    menuitems: ptr::null_mut(),
    routine: Some(M_DrawSound),
    x: 80,
    y: 64,
    lastOn: 0,
};

static mut LoadDef: menu_t = menu_t {
    numitems: load_end as i16,
    prevMenu: ptr::null_mut(),
    menuitems: ptr::null_mut(),
    routine: Some(M_DrawLoad),
    x: 80,
    y: 54,
    lastOn: 0,
};

static mut SaveDef: menu_t = menu_t {
    numitems: load_end as i16,
    prevMenu: ptr::null_mut(),
    menuitems: ptr::null_mut(),
    routine: Some(M_DrawSave),
    x: 80,
    y: 54,
    lastOn: 0,
};

fn M_ReadSaveStrings() {
    unsafe {
        for i in 0..load_end {
            let name_ptr = P_SaveGameFile(i as c_int);
            let mut name: [c_char; 256] = [0; 256];
            M_StringCopy(name.as_mut_ptr(), name_ptr, name.len());

            let handle = fopen(name.as_ptr() as *const c_char, c"rb".as_ptr());
            if handle.is_null() {
                M_StringCopy(
                    savegamestrings[i].as_mut_ptr(),
                    c"empty slot".as_ptr(),
                    SAVESTRINGSIZE,
                );
                LoadMenu[i].status = 0;
                continue;
            }
            fread(
                savegamestrings[i].as_mut_ptr() as *mut c_void,
                1,
                SAVESTRINGSIZE,
                handle,
            );
            fclose(handle);
            LoadMenu[i].status = 1;
        }
    }
}

extern "C" fn M_DrawLoad() {
    unsafe {
        V_DrawPatchDirect(
            72,
            28,
            W_CacheLumpName(c"M_LOADG".as_ptr(), 0) as *mut patch_t,
        );

        for i in 0..load_end {
            M_DrawSaveLoadBorder(
                LoadDef.x as c_int,
                LoadDef.y as c_int + LINEHEIGHT * i as c_int,
            );
            M_WriteText(
                LoadDef.x as c_int,
                LoadDef.y as c_int + LINEHEIGHT * i as c_int,
                savegamestrings[i].as_mut_ptr(),
            );
        }
    }
}

fn M_DrawSaveLoadBorder(x: c_int, y: c_int) {
    V_DrawPatchDirect(
        x - 8,
        y + 7,
        W_CacheLumpName(c"M_LSLEFT".as_ptr(), 0) as *mut patch_t,
    );

    let mut xi = x;
    for _ in 0..24 {
        V_DrawPatchDirect(
            xi,
            y + 7,
            W_CacheLumpName(c"M_LSCNTR".as_ptr(), 0) as *mut patch_t,
        );
        xi += 8;
    }

    V_DrawPatchDirect(
        xi,
        y + 7,
        W_CacheLumpName(c"M_LSRGHT".as_ptr(), 0) as *mut patch_t,
    );
}

extern "C" fn M_LoadSelect(choice: c_int) {
    unsafe {
        let mut name: [c_char; 256] = [0; 256];
        M_StringCopy(name.as_mut_ptr(), P_SaveGameFile(choice), name.len());
        G_LoadGame(name.as_mut_ptr());
        M_ClearMenus();
    }
}

extern "C" fn M_LoadGame(_choice: c_int) {
    unsafe {
        if netgame != 0 {
            M_StartMessage(
                c"you can't do load while in a net game!\n\npress a key."
                    .as_ptr()
                    .cast_mut(),
                None,
                0,
            );
            return;
        }
        M_SetupNextMenu(&raw mut LoadDef);
    }
    M_ReadSaveStrings();
}

extern "C" fn M_DrawSave() {
    unsafe {
        V_DrawPatchDirect(
            72,
            28,
            W_CacheLumpName(c"M_SAVEG".as_ptr(), 0) as *mut patch_t,
        );
        for i in 0..load_end {
            M_DrawSaveLoadBorder(
                LoadDef.x as c_int,
                LoadDef.y as c_int + LINEHEIGHT * i as c_int,
            );
            M_WriteText(
                LoadDef.x as c_int,
                LoadDef.y as c_int + LINEHEIGHT * i as c_int,
                savegamestrings[i].as_mut_ptr(),
            );
        }

        if saveStringEnter != 0 {
            let w = M_StringWidth(savegamestrings[saveSlot as usize].as_mut_ptr());
            M_WriteText(
                LoadDef.x as c_int + w,
                LoadDef.y as c_int + LINEHEIGHT * saveSlot as c_int,
                c"_".as_ptr().cast_mut(),
            );
        }
    }
}

fn M_DoSave(slot: c_int) {
    unsafe {
        G_SaveGame(slot, savegamestrings[slot as usize].as_ptr());
        M_ClearMenus();
        if quickSaveSlot == -2 {
            quickSaveSlot = slot;
        }
    }
}

extern "C" fn M_SaveSelect(choice: c_int) {
    unsafe {
        saveStringEnter = 1;
        saveSlot = choice;
        M_StringCopy(
            std::ptr::addr_of_mut!(saveOldString[0]),
            savegamestrings[choice as usize].as_ptr(),
            SAVESTRINGSIZE,
        );
        if strcmp(
            savegamestrings[choice as usize].as_ptr(),
            c"empty slot".as_ptr(),
        ) == 0
        {
            savegamestrings[choice as usize][0] = 0;
        }
        saveCharIndex = strlen(savegamestrings[choice as usize].as_ptr()) as c_int;
    }
}

extern "C" fn M_SaveGame(_choice: c_int) {
    const GS_LEVEL: c_int = 0;
    unsafe {
        if usergame == 0 {
            M_StartMessage(
                c"you can't save if you aren't playing!\n\npress a key."
                    .as_ptr()
                    .cast_mut(),
                None,
                0,
            );
            return;
        }
        if gamestate != GS_LEVEL {
            return;
        }
        M_SetupNextMenu(&raw mut SaveDef);
    }
    M_ReadSaveStrings();
}

fn M_QuickSave() {
    const GS_LEVEL: c_int = 0;
    unsafe {
        if usergame == 0 {
            S_StartSound(ptr::null_mut(), Sfx::Oof as c_int);
            return;
        }
        if gamestate != GS_LEVEL {
            return;
        }
        if quickSaveSlot < 0 {
            M_StartControlPanel();
            M_ReadSaveStrings();
            M_SetupNextMenu(&raw mut SaveDef);
            quickSaveSlot = -2;
            return;
        }
        static mut QUICK_SAVE_MSG: [c_char; 80] = [0; 80];
        let slot_str = std::ffi::CStr::from_ptr(savegamestrings[quickSaveSlot as usize].as_ptr())
            .to_string_lossy();
        c_write!(
            QUICK_SAVE_MSG,
            "quicksave over your game named\n\n'{}'?\n\npress y or n.",
            slot_str
        );
        M_StartMessage(
            std::ptr::addr_of_mut!(QUICK_SAVE_MSG[0]),
            Some(M_QuickSaveResponse),
            1,
        );
    }
}

extern "C" fn M_QuickSaveResponse(key: c_int) {
    use super::m_controls::key_menu_confirm;
    unsafe {
        if key == key_menu_confirm {
            M_DoSave(quickSaveSlot);
            S_StartSound(ptr::null_mut(), Sfx::Swtchx as c_int);
        }
    }
}

fn M_QuickLoad() {
    unsafe {
        if netgame != 0 {
            M_StartMessage(
                c"you can't quickload during a netgame!\n\npress a key."
                    .as_ptr()
                    .cast_mut(),
                None,
                0,
            );
            return;
        }
        if quickSaveSlot < 0 {
            M_StartMessage(
                c"you haven't picked a quicksave slot yet!\n\npress a key.".as_ptr() as *mut c_char,
                None,
                0,
            );
            return;
        }
        static mut QUICK_LOAD_MSG: [c_char; 80] = [0; 80];
        let slot_str = std::ffi::CStr::from_ptr(savegamestrings[quickSaveSlot as usize].as_ptr())
            .to_string_lossy();
        c_write!(
            QUICK_LOAD_MSG,
            "do you want to quickload the game named\n\n'{}'?\n\npress y or n.",
            slot_str
        );
        M_StartMessage(
            std::ptr::addr_of_mut!(QUICK_LOAD_MSG[0]),
            Some(M_QuickLoadResponse),
            1,
        );
    }
}

extern "C" fn M_QuickLoadResponse(key: c_int) {
    use super::m_controls::key_menu_confirm;
    unsafe {
        if key == key_menu_confirm {
            M_LoadSelect(quickSaveSlot);
            S_StartSound(ptr::null_mut(), Sfx::Swtchx as c_int);
        }
    }
}

extern "C" fn M_DrawReadThis1() {
    unsafe {
        inhelpscreens = 1;

        let lumpname: *const c_char;
        let skullx: i16;
        let skully: i16;

        match gameversion {
            d_mode::exe_doom_1_666
            | d_mode::exe_doom_1_7
            | d_mode::exe_doom_1_8
            | d_mode::exe_doom_1_9
            | d_mode::exe_hacx => {
                if gamemode == d_mode::commercial {
                    lumpname = c"HELP".as_ptr();
                    skullx = 330;
                    skully = 165;
                } else {
                    lumpname = c"HELP2".as_ptr();
                    skullx = 280;
                    skully = 185;
                }
            }
            d_mode::exe_ultimate | d_mode::exe_chex => {
                lumpname = c"HELP1".as_ptr();
                skullx = 280;
                skully = 185;
            }
            d_mode::exe_final | d_mode::exe_final2 => {
                lumpname = c"HELP".as_ptr();
                skullx = 330;
                skully = 165;
            }
            _ => {
                i_error!("Unhandled game version");
                return;
            }
        }

        V_DrawPatchDirect(0, 0, W_CacheLumpName(lumpname, 0) as *mut patch_t);
        ReadDef1.x = skullx;
        ReadDef1.y = skully;
    }
}

extern "C" fn M_DrawReadThis2() {
    unsafe {
        inhelpscreens = 1;
        V_DrawPatchDirect(0, 0, W_CacheLumpName(c"HELP1".as_ptr(), 0) as *mut patch_t);
    }
}

extern "C" fn M_DrawSound() {
    use super::s_sound::{musicVolume, sfxVolume};
    unsafe {
        V_DrawPatchDirect(
            60,
            38,
            W_CacheLumpName(c"M_SVOL".as_ptr(), 0) as *mut patch_t,
        );

        M_DrawThermo(
            SoundDef.x as c_int,
            SoundDef.y as c_int + LINEHEIGHT * (sfx_vol as c_int + 1),
            16,
            sfxVolume,
        );

        M_DrawThermo(
            SoundDef.x as c_int,
            SoundDef.y as c_int + LINEHEIGHT * (music_vol as c_int + 1),
            16,
            musicVolume,
        );
    }
}

extern "C" fn M_Sound(_choice: c_int) {}

extern "C" fn M_SfxVol(choice: c_int) {
    use super::s_sound::sfxVolume;
    unsafe {
        match choice {
            0 if sfxVolume > 0 => {
                sfxVolume -= 1;
            }
            1 if sfxVolume < 15 => {
                sfxVolume += 1;
            }
            _ => {}
        }
        S_SetSfxVolume(sfxVolume * 8);
    }
}

extern "C" fn M_MusicVol(choice: c_int) {
    use super::s_sound::musicVolume;
    unsafe {
        match choice {
            0 if musicVolume > 0 => {
                musicVolume -= 1;
            }
            1 if musicVolume < 15 => {
                musicVolume += 1;
            }
            _ => {}
        }
        S_SetMusicVolume(musicVolume * 8);
    }
}

extern "C" fn M_DrawMainMenu() {
    V_DrawPatchDirect(
        94,
        2,
        W_CacheLumpName(c"M_DOOM".as_ptr(), 0) as *mut patch_t,
    );
}

extern "C" fn M_DrawNewGame() {
    V_DrawPatchDirect(
        96,
        14,
        W_CacheLumpName(c"M_NEWG".as_ptr(), 0) as *mut patch_t,
    );
    V_DrawPatchDirect(
        54,
        38,
        W_CacheLumpName(c"M_SKILL".as_ptr(), 0) as *mut patch_t,
    );
}

extern "C" fn M_NewGame(_choice: c_int) {
    unsafe {
        if netgame != 0 && demoplayback == 0 {
            M_StartMessage(
                c"you can't start a new game\nwhile in a network game.\n\npress a key.".as_ptr()
                    as *mut c_char,
                None,
                0,
            );
            return;
        }
        if gamemode == d_mode::commercial || gameversion == d_mode::exe_chex {
            M_SetupNextMenu(&raw mut NewDef);
        } else {
            M_SetupNextMenu(&raw mut EpiDef);
        }
    }
}

extern "C" fn M_DrawEpisode() {
    V_DrawPatchDirect(
        54,
        38,
        W_CacheLumpName(c"M_EPISOD".as_ptr(), 0) as *mut patch_t,
    );
}

static mut epi: c_int = 0;

extern "C" fn M_VerifyNightmare(key: c_int) {
    use super::m_controls::key_menu_confirm;
    unsafe {
        if key != key_menu_confirm {
            return;
        }
        G_DeferedInitNew(4, epi + 1, 1);
        M_ClearMenus();
    }
}

extern "C" fn M_ChooseSkill(choice: c_int) {
    if choice as usize == nightmare {
        M_StartMessage(
            c"are you sure? this skill level\nisn't even remotely fair.\n\npress y or n.".as_ptr()
                as *mut c_char,
            Some(M_VerifyNightmare),
            1,
        );
        return;
    }
    unsafe {
        G_DeferedInitNew(choice, epi + 1, 1);
        M_ClearMenus();
    }
}

extern "C" fn M_Episode(choice: c_int) {
    unsafe {
        if gamemode == d_mode::shareware && choice != 0 {
            M_StartMessage(
                c"this is the shareware version of doom.\n\nyou need to order the entire trilogy.\n\npress a key.".as_ptr()
                    as *mut c_char,
                None,
                0,
            );
            M_SetupNextMenu(&raw mut ReadDef1);
            return;
        }
        if gamemode == d_mode::registered && choice > 2 {
            epi = 0;
        } else {
            epi = choice;
        }
        M_SetupNextMenu(&raw mut NewDef);
    }
}

extern "C" fn M_DrawOptions() {
    unsafe {
        V_DrawPatchDirect(
            108,
            15,
            W_CacheLumpName(c"M_OPTTTL".as_ptr(), 0) as *mut patch_t,
        );

        let detail_names: [*const c_char; 2] = [c"M_GDHIGH".as_ptr(), c"M_GDLOW".as_ptr()];
        let msg_names: [*const c_char; 2] = [c"M_MSGOFF".as_ptr(), c"M_MSGON".as_ptr()];

        V_DrawPatchDirect(
            OptionsDef.x as c_int + 175,
            OptionsDef.y as c_int + LINEHEIGHT * detail as c_int,
            W_CacheLumpName(detail_names[detailLevel as usize], 0) as *mut patch_t,
        );

        V_DrawPatchDirect(
            OptionsDef.x as c_int + 120,
            OptionsDef.y as c_int + LINEHEIGHT * messages as c_int,
            W_CacheLumpName(msg_names[showMessages as usize], 0) as *mut patch_t,
        );

        M_DrawThermo(
            OptionsDef.x as c_int,
            OptionsDef.y as c_int + LINEHEIGHT * (mousesens as c_int + 1),
            10,
            mouseSensitivity,
        );

        M_DrawThermo(
            OptionsDef.x as c_int,
            OptionsDef.y as c_int + LINEHEIGHT * (scrnsize as c_int + 1),
            9,
            screenSize,
        );
    }
}

extern "C" fn M_Options(_choice: c_int) {}

extern "C" fn M_ChangeMessages(_choice: c_int) {
    unsafe {
        showMessages = 1 - showMessages;
        if showMessages == 0 {
            M_Menu_SetPlayerMessage(c"Messages OFF".as_ptr());
        } else {
            M_Menu_SetPlayerMessage(c"Messages ON".as_ptr());
        }
        message_dontfuckwithme = 1;
    }
}

extern "C" fn M_EndGameResponse(key: c_int) {
    use super::m_controls::key_menu_confirm;
    unsafe {
        if key != key_menu_confirm {
            return;
        }
        (*currentMenu).lastOn = itemOn;
    }
    M_ClearMenus();
}

extern "C" fn M_EndGame(_choice: c_int) {
    unsafe {
        if usergame == 0 {
            S_StartSound(ptr::null_mut(), Sfx::Oof as c_int);
            return;
        }
        if netgame != 0 {
            M_StartMessage(
                c"you can't end a netgame!\n\npress a key."
                    .as_ptr()
                    .cast_mut(),
                None,
                0,
            );
            return;
        }
        M_StartMessage(
            c"are you sure you want to end the game?\n\npress y or n."
                .as_ptr()
                .cast_mut(),
            Some(M_EndGameResponse),
            1,
        );
    }
}

extern "C" fn M_ReadThis(_choice: c_int) {}

extern "C" fn M_ReadThis2(_choice: c_int) {
    unsafe {
        if gameversion <= d_mode::exe_doom_1_9 && gamemode != d_mode::commercial {
            M_SetupNextMenu(&raw mut ReadDef2);
        } else {
            M_FinishReadThis(0);
        }
    }
}

extern "C" fn M_FinishReadThis(_choice: c_int) {}

static mut quitsounds: [c_int; 8] = [
    Sfx::Pldeth as c_int,
    Sfx::Dmpain as c_int,
    Sfx::Popain as c_int,
    Sfx::Slop as c_int,
    Sfx::Telept as c_int,
    Sfx::Posit1 as c_int,
    Sfx::Posit3 as c_int,
    Sfx::Sgtatk as c_int,
];
static mut quitsounds2: [c_int; 8] = [
    Sfx::Vilact as c_int,
    Sfx::Getpow as c_int,
    Sfx::Boscub as c_int,
    Sfx::Slop as c_int,
    Sfx::Skeswg as c_int,
    Sfx::Kntdth as c_int,
    Sfx::Bspact as c_int,
    Sfx::Sgtatk as c_int,
];

extern "C" fn M_QuitResponse(key: c_int) {
    use super::m_controls::key_menu_confirm;
    unsafe {
        if key != key_menu_confirm {
            return;
        }
        if netgame == 0 {
            if gamemode == d_mode::commercial {
                S_StartSound(ptr::null_mut(), quitsounds2[((gametic >> 2) & 7) as usize]);
            } else {
                S_StartSound(ptr::null_mut(), quitsounds[((gametic >> 2) & 7) as usize]);
            }
            I_WaitVBL(105);
        }
        I_Quit();
    }
}

fn M_SelectEndMessage() -> *const c_char {
    unsafe {
        if logical_gamemission() == d_mode::doom {
            doom1_endmsg[(gametic as usize) & 7].0
        } else {
            doom2_endmsg[(gametic as usize) & 7].0
        }
    }
}

extern "C" fn M_QuitDOOM(_choice: c_int) {
    unsafe {
        let msg = M_SelectEndMessage();
        let msg_str = std::ffi::CStr::from_ptr(msg).to_string_lossy();
        c_write!(endstring, "{}\n\n(press y to quit to dos.)", msg_str);
        M_StartMessage(
            std::ptr::addr_of_mut!(endstring[0]),
            Some(M_QuitResponse),
            1,
        );
    }
}

extern "C" fn M_ChangeSensitivity(choice: c_int) {
    unsafe {
        match choice {
            0 if mouseSensitivity > 0 => {
                mouseSensitivity -= 1;
            }
            1 if mouseSensitivity < 9 => {
                mouseSensitivity += 1;
            }
            _ => {}
        }
    }
}

extern "C" fn M_ChangeDetail(_choice: c_int) {
    unsafe {
        detailLevel = 1 - detailLevel;
        R_SetViewSize(screenblocks, detailLevel);
        if detailLevel == 0 {
            M_Menu_SetPlayerMessage(c"High detail".as_ptr());
        } else {
            M_Menu_SetPlayerMessage(c"Low detail".as_ptr());
        }
    }
}

extern "C" fn M_SizeDisplay(choice: c_int) {
    unsafe {
        match choice {
            0 if screenSize > 0 => {
                screenblocks -= 1;
                screenSize -= 1;
            }
            1 if screenSize < 8 => {
                screenblocks += 1;
                screenSize += 1;
            }
            _ => {}
        }
        R_SetViewSize(screenblocks, detailLevel);
    }
}

fn M_DrawThermo(x: c_int, y: c_int, thermWidth: c_int, thermDot: c_int) {
    let mut xx = x;
    V_DrawPatchDirect(
        xx,
        y,
        W_CacheLumpName(c"M_THERML".as_ptr(), 0) as *mut patch_t,
    );
    xx += 8;
    for _ in 0..thermWidth {
        V_DrawPatchDirect(
            xx,
            y,
            W_CacheLumpName(c"M_THERMM".as_ptr(), 0) as *mut patch_t,
        );
        xx += 8;
    }
    V_DrawPatchDirect(
        xx,
        y,
        W_CacheLumpName(c"M_THERMR".as_ptr(), 0) as *mut patch_t,
    );

    V_DrawPatchDirect(
        (x + 8) + thermDot * 8,
        y,
        W_CacheLumpName(c"M_THERMO".as_ptr(), 0) as *mut patch_t,
    );
}

fn M_DrawEmptyCell(_menu: *mut menu_t, _item: c_int) {}
fn M_DrawSelCell(_menu: *mut menu_t, _item: c_int) {}

fn M_StartMessage(string: *mut c_char, routine: Option<extern "C" fn(c_int)>, input: c_int) {
    unsafe {
        messageLastMenuActive = menuactive;
        messageToPrint = 1;
        messageString = string;
        messageRoutine = routine;
        messageNeedsInput = input;
        menuactive = 1;
    }
}

fn M_StopMessage() {
    unsafe {
        menuactive = messageLastMenuActive;
        messageToPrint = 0;
    }
}

fn M_StringWidth(string: *mut c_char) -> c_int {
    unsafe {
        let len = strlen(string);
        let mut w: c_int = 0;
        for i in 0..len {
            let c = toupper(*string.add(i) as c_int) - HU_FONTSTART;
            if c < 0 || c as usize >= HU_FONTSIZE {
                w += 4;
            } else {
                let patch = hu_font[c as usize] as *const patch_stub;
                w += (*patch).width as c_int;
            }
        }
        w
    }
}

fn M_StringHeight(string: *mut c_char) -> c_int {
    unsafe {
        let patch = hu_font[0] as *const patch_stub;
        let height = (*patch).height as c_int;
        let mut h = height;
        let len = strlen(string);
        for i in 0..len {
            if *string.add(i) == b'\n' as c_char {
                h += height;
            }
        }
        h
    }
}

fn M_WriteText(x: c_int, y: c_int, string: *mut c_char) {
    unsafe {
        let mut ch_ptr = string;
        let mut cx = x;
        let mut cy = y;

        loop {
            let c = *ch_ptr;
            if c == 0 {
                break;
            }
            ch_ptr = ch_ptr.add(1);
            if c == b'\n' as c_char {
                cx = x;
                cy += 12;
                continue;
            }

            let c = toupper(c as c_int) - HU_FONTSTART;
            if c < 0 || c as usize >= HU_FONTSIZE {
                cx += 4;
                continue;
            }

            let patch = hu_font[c as usize] as *const patch_stub;
            let w = (*patch).width as c_int;
            if cx + w > SCREENWIDTH {
                break;
            }
            V_DrawPatchDirect(cx, cy, hu_font[c as usize]);
            cx += w;
        }
    }
}

fn IsNullKey(key: c_int) -> bool {
    key == KEY_PAUSE || key == KEY_CAPSLOCK || key == KEY_SCRLCK || key == KEY_NUMLOCK
}

static mut RESP_joywait: c_int = 0;
static mut RESP_mousewait: c_int = 0;
static mut RESP_mousey: c_int = 0;
static mut RESP_lasty: c_int = 0;
static mut RESP_mousex: c_int = 0;
static mut RESP_lastx: c_int = 0;

#[no_mangle]
pub extern "C" fn M_Responder(ev: *mut event_t) -> Boolean {
    use super::m_controls::{
        joybmenu, key_menu_abort, key_menu_activate, key_menu_back, key_menu_confirm,
        key_menu_decscreen, key_menu_detail, key_menu_down, key_menu_endgame, key_menu_forward,
        key_menu_gamma, key_menu_help, key_menu_incscreen, key_menu_left, key_menu_load,
        key_menu_messages, key_menu_qload, key_menu_qsave, key_menu_quit, key_menu_right,
        key_menu_save, key_menu_screenshot, key_menu_up, key_menu_volume,
    };

    unsafe {
        let ev = &*ev;

        // testcontrols mode
        if testcontrols != 0 {
            if ev.type_ == EV_QUIT
                || (ev.type_ == EV_KEYDOWN
                    && (ev.data1 == key_menu_activate || ev.data1 == key_menu_quit))
            {
                I_Quit();
                return Boolean::TRUE;
            }
            return Boolean::FALSE;
        }

        // window close button
        if ev.type_ == EV_QUIT {
            if menuactive != 0
                && messageToPrint != 0
                && messageRoutine.map(|f| f as usize).unwrap_or(0)
                    == M_QuitResponse as *const () as usize
            {
                M_QuitResponse(key_menu_confirm);
            } else {
                S_StartSound(ptr::null_mut(), Sfx::Swtchn as c_int);
                M_QuitDOOM(0);
            }
            return Boolean::TRUE;
        }

        let mut ch: c_int = 0;
        let mut key: c_int = -1;

        if ev.type_ == EV_JOYSTICK && RESP_joywait < I_GetTime() {
            if ev.data3 < 0 {
                key = key_menu_up;
                RESP_joywait = I_GetTime() + 5;
            } else if ev.data3 > 0 {
                key = key_menu_down;
                RESP_joywait = I_GetTime() + 5;
            }
            if ev.data2 < 0 {
                key = key_menu_left;
                RESP_joywait = I_GetTime() + 2;
            } else if ev.data2 > 0 {
                key = key_menu_right;
                RESP_joywait = I_GetTime() + 2;
            }
            if ev.data1 & 1 != 0 {
                key = key_menu_forward;
                RESP_joywait = I_GetTime() + 5;
            }
            if ev.data1 & 2 != 0 {
                key = key_menu_back;
                RESP_joywait = I_GetTime() + 5;
            }
            if joybmenu >= 0 && (ev.data1 & (1 << joybmenu)) != 0 {
                key = key_menu_activate;
                RESP_joywait = I_GetTime() + 5;
            }
        } else if ev.type_ == EV_MOUSE && RESP_mousewait < I_GetTime() {
            RESP_mousey += ev.data3;
            if RESP_mousey < RESP_lasty - 30 {
                key = key_menu_down;
                RESP_mousewait = I_GetTime() + 5;
                RESP_lasty -= 30;
                RESP_mousey = RESP_lasty;
            } else if RESP_mousey > RESP_lasty + 30 {
                key = key_menu_up;
                RESP_mousewait = I_GetTime() + 5;
                RESP_lasty += 30;
                RESP_mousey = RESP_lasty;
            }

            RESP_mousex += ev.data2;
            if RESP_mousex < RESP_lastx - 30 {
                key = key_menu_left;
                RESP_mousewait = I_GetTime() + 5;
                RESP_lastx -= 30;
                RESP_mousex = RESP_lastx;
            } else if RESP_mousex > RESP_lastx + 30 {
                key = key_menu_right;
                RESP_mousewait = I_GetTime() + 5;
                RESP_lastx += 30;
                RESP_mousex = RESP_lastx;
            }

            if ev.data1 & 1 != 0 {
                key = key_menu_forward;
                RESP_mousewait = I_GetTime() + 15;
            }
            if ev.data1 & 2 != 0 {
                key = key_menu_back;
                RESP_mousewait = I_GetTime() + 15;
            }
        } else if ev.type_ == EV_KEYDOWN {
            key = ev.data1;
            ch = ev.data2;
        }

        if key == -1 {
            return Boolean::FALSE;
        }

        // Save Game string input
        if saveStringEnter != 0 {
            if key == KEY_BACKSPACE {
                if saveCharIndex > 0 {
                    saveCharIndex -= 1;
                    savegamestrings[saveSlot as usize][saveCharIndex as usize] = 0;
                }
            } else if key == KEY_ESCAPE {
                saveStringEnter = 0;
                M_StringCopy(
                    savegamestrings[saveSlot as usize].as_mut_ptr(),
                    std::ptr::addr_of!(saveOldString[0]),
                    SAVESTRINGSIZE,
                );
            } else if key == KEY_ENTER {
                saveStringEnter = 0;
                if savegamestrings[saveSlot as usize][0] != 0 {
                    M_DoSave(saveSlot);
                }
            } else {
                if vanilla_keyboard_mapping != 0 {
                    ch = key;
                }
                ch = toupper(ch);

                if ch != b' ' as c_int
                    && (ch - HU_FONTSTART < 0 || ch - HU_FONTSTART >= HU_FONTSIZE as c_int)
                {
                    return Boolean::TRUE;
                }

                if (32..=127).contains(&ch)
                    && saveCharIndex < (SAVESTRINGSIZE as c_int - 1)
                    && M_StringWidth(savegamestrings[saveSlot as usize].as_mut_ptr())
                        < (SAVESTRINGSIZE as c_int - 2) * 8
                {
                    savegamestrings[saveSlot as usize][saveCharIndex as usize] = ch as c_char;
                    saveCharIndex += 1;
                    savegamestrings[saveSlot as usize][saveCharIndex as usize] = 0;
                }
            }
            return Boolean::TRUE;
        }

        // Messages that need input
        if messageToPrint != 0 {
            if messageNeedsInput != 0
                && key != b' ' as c_int
                && key != KEY_ESCAPE
                && key != key_menu_confirm
                && key != key_menu_abort
            {
                return Boolean::FALSE;
            }

            menuactive = messageLastMenuActive;
            messageToPrint = 0;
            if let Some(r) = messageRoutine {
                r(key);
            }
            menuactive = 0;
            S_StartSound(ptr::null_mut(), Sfx::Swtchx as c_int);
            return Boolean::TRUE;
        }

        // Screenshot
        if (devparm != 0 && key == key_menu_help) || (key != 0 && key == key_menu_screenshot) {
            G_ScreenShot();
            return Boolean::TRUE;
        }

        // F-Keys (when menu not active)
        if menuactive == 0 {
            if key == key_menu_decscreen {
                if automapactive != 0 || chat_on != 0 {
                    return Boolean::FALSE;
                }
                M_SizeDisplay(0);
                S_StartSound(ptr::null_mut(), Sfx::Stnmov as c_int);
                return Boolean::TRUE;
            } else if key == key_menu_incscreen {
                if automapactive != 0 || chat_on != 0 {
                    return Boolean::FALSE;
                }
                M_SizeDisplay(1);
                S_StartSound(ptr::null_mut(), Sfx::Stnmov as c_int);
                return Boolean::TRUE;
            } else if key == key_menu_help {
                M_StartControlPanel();
                if gamemode == d_mode::retail {
                    currentMenu = &raw mut ReadDef2;
                } else {
                    currentMenu = &raw mut ReadDef1;
                }
                itemOn = 0;
                S_StartSound(ptr::null_mut(), Sfx::Swtchn as c_int);
                return Boolean::TRUE;
            } else if key == key_menu_save {
                M_StartControlPanel();
                S_StartSound(ptr::null_mut(), Sfx::Swtchn as c_int);
                M_SaveGame(0);
                return Boolean::TRUE;
            } else if key == key_menu_load {
                M_StartControlPanel();
                S_StartSound(ptr::null_mut(), Sfx::Swtchn as c_int);
                M_LoadGame(0);
                return Boolean::TRUE;
            } else if key == key_menu_volume {
                M_StartControlPanel();
                currentMenu = &raw mut SoundDef;
                itemOn = sfx_vol as i16;
                S_StartSound(ptr::null_mut(), Sfx::Swtchn as c_int);
                return Boolean::TRUE;
            } else if key == key_menu_detail {
                M_ChangeDetail(0);
                S_StartSound(ptr::null_mut(), Sfx::Swtchn as c_int);
                return Boolean::TRUE;
            } else if key == key_menu_qsave {
                S_StartSound(ptr::null_mut(), Sfx::Swtchn as c_int);
                M_QuickSave();
                return Boolean::TRUE;
            } else if key == key_menu_endgame {
                S_StartSound(ptr::null_mut(), Sfx::Swtchn as c_int);
                M_EndGame(0);
                return Boolean::TRUE;
            } else if key == key_menu_messages {
                M_ChangeMessages(0);
                S_StartSound(ptr::null_mut(), Sfx::Swtchn as c_int);
                return Boolean::TRUE;
            } else if key == key_menu_qload {
                S_StartSound(ptr::null_mut(), Sfx::Swtchn as c_int);
                M_QuickLoad();
                return Boolean::TRUE;
            } else if key == key_menu_quit {
                S_StartSound(ptr::null_mut(), Sfx::Swtchn as c_int);
                M_QuitDOOM(0);
                return Boolean::TRUE;
            } else if key == key_menu_gamma {
                usegamma += 1;
                if usegamma > 4 {
                    usegamma = 0;
                }
                M_Menu_SetPlayerMessage(gammamsg[usegamma as usize].as_ptr());
                I_SetPalette(W_CacheLumpName(c"PLAYPAL".as_ptr(), 0) as *mut u8);
                return Boolean::TRUE;
            }
        }

        // Pop-up menu?
        if menuactive == 0 {
            if key == key_menu_activate {
                M_StartControlPanel();
                S_StartSound(ptr::null_mut(), Sfx::Swtchn as c_int);
                return Boolean::TRUE;
            }
            return Boolean::FALSE;
        }

        // Keys usable within menu
        if key == key_menu_down {
            loop {
                if itemOn + 1 > (*currentMenu).numitems - 1 {
                    itemOn = 0;
                } else {
                    itemOn += 1;
                }
                S_StartSound(ptr::null_mut(), Sfx::Pstop as c_int);
                if (*(*currentMenu).menuitems.offset(itemOn as isize)).status != -1 {
                    break;
                }
            }
            return Boolean::TRUE;
        } else if key == key_menu_up {
            loop {
                if itemOn == 0 {
                    itemOn = (*currentMenu).numitems - 1;
                } else {
                    itemOn -= 1;
                }
                S_StartSound(ptr::null_mut(), Sfx::Pstop as c_int);
                if (*(*currentMenu).menuitems.offset(itemOn as isize)).status != -1 {
                    break;
                }
            }
            return Boolean::TRUE;
        } else if key == key_menu_left {
            let item = &*(*currentMenu).menuitems.offset(itemOn as isize);
            if item.status == 2 {
                if let Some(routine) = item.routine {
                    S_StartSound(ptr::null_mut(), Sfx::Stnmov as c_int);
                    routine(0);
                }
            }
            return Boolean::TRUE;
        } else if key == key_menu_right {
            let item = &*(*currentMenu).menuitems.offset(itemOn as isize);
            if item.status == 2 {
                if let Some(routine) = item.routine {
                    S_StartSound(ptr::null_mut(), Sfx::Stnmov as c_int);
                    routine(1);
                }
            }
            return Boolean::TRUE;
        } else if key == key_menu_forward {
            let item = &*(*currentMenu).menuitems.offset(itemOn as isize);
            if let Some(routine) = item.routine {
                if item.status != 0 {
                    (*currentMenu).lastOn = itemOn;
                    if item.status == 2 {
                        routine(1);
                        S_StartSound(ptr::null_mut(), Sfx::Stnmov as c_int);
                    } else {
                        routine(itemOn as c_int);
                        S_StartSound(ptr::null_mut(), Sfx::Pistol as c_int);
                    }
                }
            }
            return Boolean::TRUE;
        } else if key == key_menu_activate {
            (*currentMenu).lastOn = itemOn;
            M_ClearMenus();
            S_StartSound(ptr::null_mut(), Sfx::Swtchx as c_int);
            return Boolean::TRUE;
        } else if key == key_menu_back {
            (*currentMenu).lastOn = itemOn;
            if !(*currentMenu).prevMenu.is_null() {
                currentMenu = (*currentMenu).prevMenu;
                itemOn = (*currentMenu).lastOn;
                S_StartSound(ptr::null_mut(), Sfx::Swtchn as c_int);
            }
            return Boolean::TRUE;
        }

        // Keyboard shortcut
        if ch != 0 || IsNullKey(key) {
            let ch_u = ch as c_char;
            for i in (itemOn + 1)..(*currentMenu).numitems {
                if (*(*currentMenu).menuitems.offset(i as isize)).alphaKey == ch_u {
                    itemOn = i;
                    S_StartSound(ptr::null_mut(), Sfx::Pstop as c_int);
                    return Boolean::TRUE;
                }
            }
            for i in 0..=itemOn {
                if (*(*currentMenu).menuitems.offset(i as isize)).alphaKey == ch_u {
                    itemOn = i;
                    S_StartSound(ptr::null_mut(), Sfx::Pstop as c_int);
                    return Boolean::TRUE;
                }
            }
        }

        Boolean::FALSE
    }
}

#[no_mangle]
pub extern "C" fn M_StartControlPanel() {
    unsafe {
        if menuactive != 0 {
            return;
        }
        menuactive = 1;
        currentMenu = &raw mut MainDef;
        itemOn = (*currentMenu).lastOn;
    }
}

#[no_mangle]
pub extern "C" fn M_Drawer() {
    unsafe {
        inhelpscreens = 0;

        if messageToPrint != 0 {
            let mut start: usize = 0;
            messy = SCREENHEIGHT / 2 - M_StringHeight(messageString) / 2;
            while *messageString.add(start) != 0 {
                let mut foundnewline = 0;
                let mut string: [c_char; 80] = [0; 80];
                let remaining = strlen(messageString.add(start));
                for i in 0..remaining {
                    if *messageString.add(start + i) == b'\n' as c_char {
                        M_StringCopy(string.as_mut_ptr(), messageString.add(start), string.len());
                        if i < string.len() {
                            string[i] = 0;
                        }
                        foundnewline = 1;
                        start += i + 1;
                        break;
                    }
                }
                if foundnewline == 0 {
                    M_StringCopy(string.as_mut_ptr(), messageString.add(start), string.len());
                    start += strlen(string.as_ptr());
                }

                messx = SCREENWIDTH / 2 - M_StringWidth(string.as_mut_ptr()) / 2;
                M_WriteText(messx, messy, string.as_mut_ptr());
                let patch = hu_font[0] as *const patch_stub;
                messy += (*patch).height as c_int;
            }
            return;
        }

        if menuactive == 0 {
            return;
        }

        if let Some(r) = (*currentMenu).routine {
            r();
        }

        let x = (*currentMenu).x as c_int;
        let y = (*currentMenu).y as c_int;
        let max = (*currentMenu).numitems as usize;

        for i in 0..max {
            let name = (*(*currentMenu).menuitems.add(i)).name.as_ptr();
            if *name != 0 {
                V_DrawPatchDirect(
                    x,
                    y + LINEHEIGHT * i as c_int,
                    W_CacheLumpName(name, 0) as *mut patch_t,
                );
            }
        }

        V_DrawPatchDirect(
            x + SKULLXOFF,
            (*currentMenu).y as c_int - 5 + itemOn as c_int * LINEHEIGHT,
            W_CacheLumpName(skullName[whichSkull as usize], 0) as *mut patch_t,
        );
    }
}

fn M_ClearMenus() {
    unsafe {
        menuactive = 0;
    }
}

fn M_SetupNextMenu(menudef: *mut menu_t) {
    unsafe {
        currentMenu = menudef;
        itemOn = (*currentMenu).lastOn;
    }
}

#[no_mangle]
pub extern "C" fn M_Ticker() {
    unsafe {
        skullAnimCounter -= 1;
        if skullAnimCounter <= 0 {
            whichSkull ^= 1;
            skullAnimCounter = 8;
        }
    }
}

#[no_mangle]
pub extern "C" fn M_Init() {
    unsafe {
        currentMenu = &raw mut MainDef;
        menuactive = 0;
        itemOn = (*currentMenu).lastOn;
        whichSkull = 0;
        skullAnimCounter = 10;
        screenSize = screenblocks - 3;
        messageToPrint = 0;
        messageString = ptr::null_mut();
        messageLastMenuActive = menuactive;
        quickSaveSlot = -1;

        // Set menuitems pointers (couldn't be done at static init time)
        MainDef.menuitems = std::ptr::addr_of_mut!(MainMenu[0]);
        EpiDef.menuitems = std::ptr::addr_of_mut!(EpisodeMenu[0]);
        NewDef.menuitems = std::ptr::addr_of_mut!(NewGameMenu[0]);
        OptionsDef.menuitems = std::ptr::addr_of_mut!(OptionsMenu[0]);
        ReadDef1.menuitems = std::ptr::addr_of_mut!(ReadMenu1[0]);
        ReadDef2.menuitems = std::ptr::addr_of_mut!(ReadMenu2[0]);
        SoundDef.menuitems = std::ptr::addr_of_mut!(SoundMenu[0]);
        LoadDef.menuitems = std::ptr::addr_of_mut!(LoadMenu[0]);
        SaveDef.menuitems = std::ptr::addr_of_mut!(SaveMenu[0]);

        // Set up prevMenu cross-links
        EpiDef.prevMenu = &raw mut MainDef;
        NewDef.prevMenu = &raw mut EpiDef;
        OptionsDef.prevMenu = &raw mut MainDef;
        ReadDef1.prevMenu = &raw mut MainDef;
        ReadDef2.prevMenu = &raw mut ReadDef1;
        SoundDef.prevMenu = &raw mut OptionsDef;
        LoadDef.prevMenu = &raw mut MainDef;
        SaveDef.prevMenu = &raw mut MainDef;

        match gamemode {
            d_mode::commercial => {
                ptr::write(&mut MainMenu[readthis], ptr::read(&MainMenu[quitdoom]));
                MainDef.numitems -= 1;
                MainDef.y += 8;
                NewDef.prevMenu = &raw mut MainDef;
            }
            d_mode::shareware | d_mode::registered | d_mode::retail => {}
            _ => {}
        }

        if gameversion < d_mode::exe_ultimate {
            EpiDef.numitems -= 1;
        }
    }
}
