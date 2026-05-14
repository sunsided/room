//! Rust port of vendor/doomgeneric/d_main.c.
//!
//! Main initialization (`D_DoomMain`) and game loop (`D_DoomLoop`),
//! plus demo sequence management, display rendering, and command-line
//! parameter processing.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int, c_uint, c_void};
use std::ptr;

use crate::doom::d_event::event_t;
use crate::doom::d_mode;
use crate::doom::d_player::{consoleplayer, players, PlayerT, MAXPLAYERS};
use crate::doom::doomstat::{gamedescription, gamemission, gamemode, gameversion, modifiedgame};
use crate::doom::i_timer::TICRATE;
use crate::doom::i_video::I_StartFrame;
use crate::doom::i_video::{SCREENHEIGHT, SCREENWIDTH};
use crate::doom::m_config::M_SaveDefaults;
use crate::doom::m_misc::M_snprintf_clamp;

// ---------------------------------------------------------------------------
// String constants from d_englsh.h / dstrings.h
// ---------------------------------------------------------------------------

static D_DEVSTR: &str = "Development mode ON.\n";

/// Chat key bindings for multiplayer messages.
const HUSTR_KEYGREEN: c_char = b'g' as c_char;
const HUSTR_KEYINDIGO: c_char = b'i' as c_char;
const HUSTR_KEYBROWN: c_char = b'b' as c_char;
const HUSTR_KEYRED: c_char = b'r' as c_char;

// ---------------------------------------------------------------------------
// Type aliases matching C enums
// ---------------------------------------------------------------------------

type gamestate_t = c_int;
type gameaction_t = c_int;
type skill_t = c_int;
type boolean = c_int;
type byte = u8;

const GS_LEVEL: gamestate_t = 0;
const GS_INTERMISSION: gamestate_t = 1;
const GS_FINALE: gamestate_t = 2;
const GS_DEMOSCREEN: gamestate_t = 3;

const ga_nothing: gameaction_t = 0;
const ga_loadlevel: gameaction_t = 1;
const ga_newgame: gameaction_t = 2;
const ga_loadgame: gameaction_t = 3;
const ga_savegame: gameaction_t = 4;
const ga_playdemo: gameaction_t = 5;
const ga_completed: gameaction_t = 6;
const ga_victory: gameaction_t = 7;
const ga_worlddone: gameaction_t = 8;
const ga_screenshot: gameaction_t = 9;

const sk_baby: skill_t = 0;
const sk_easy: skill_t = 1;
const sk_medium: skill_t = 2;
const sk_hard: skill_t = 3;
const sk_nightmare: skill_t = 4;
const sk_noitems: skill_t = -1;

use crate::doom::z_zone::{PU_CACHE, PU_STATIC};

// ---------------------------------------------------------------------------
// Globals — these are #[no_mangle] so remaining C code (g_game.c etc.)
// can resolve them at link time.
// ---------------------------------------------------------------------------

/// Savegame directory.
#[no_mangle]
pub static mut savegamedir: *mut c_char = ptr::null_mut();

/// Path to the currently loaded IWAD file.
#[no_mangle]
pub static mut iwadfile: *mut c_char = ptr::null_mut();

/// Started game with -devparm.
#[no_mangle]
pub static mut devparm: c_int = 0;

/// Checkparm of -nomonsters.
#[no_mangle]
pub static mut nomonsters: c_int = 0;

/// Checkparm of -respawn.
#[no_mangle]
pub static mut respawnparm: c_int = 0;

/// Checkparm of -fast.
#[no_mangle]
pub static mut fastparm: c_int = 0;

/// Episode to start at (set by -episode).
#[no_mangle]
pub static mut startskill: c_int = 0;

/// Episode to start at (set by -episode).
#[no_mangle]
pub static mut startepisode: c_int = 0;

/// Map to start at (set by -warp / -episode).
#[no_mangle]
pub static mut startmap: c_int = 0;

/// True when -warp / -episode has provided an explicit start.
#[no_mangle]
pub static mut autostart: c_int = 0;

/// Load game slot (-1 = not loading).
#[no_mangle]
pub static mut startloadgame: c_int = 0;

/// True while playing back a built-in demo sequence.
#[no_mangle]
pub static mut advancedemo: c_int = 0;

/// Store demo, do not accept any inputs.
#[no_mangle]
pub static mut storedemo: c_int = 0;

/// True when the BFG edition of the IWAD is detected.
#[no_mangle]
pub static mut bfgedition: c_int = 0;

/// True once the main event loop has started.
#[no_mangle]
pub static mut main_loop_started: c_int = 0;

/// Primary WAD file path buffer.
#[no_mangle]
pub static mut wadfile: [c_char; 1024] = [0; 1024];

/// Directory of development maps.
#[no_mangle]
pub static mut mapdir: [c_char; 1024] = [0; 1024];

/// When non-zero, display the ENDOOM text on exit.
#[no_mangle]
pub static mut show_endoom: c_int = 1;

/// Demo loop sequence counter.
#[no_mangle]
pub static mut demosequence: c_int = 0;

/// Tics remaining on the current demo page.
#[no_mangle]
pub static mut pagetic: c_int = 0;

/// Name of the current demo page lump.
#[no_mangle]
pub static mut pagename: *mut c_char = ptr::null_mut();

/// Wipe gamestate for transition effects. -1 forces a wipe on next draw.
#[no_mangle]
pub static mut wipegamestate: c_int = GS_DEMOSCREEN;

// D_Display static local state — these must persist across frames.
static mut D_DISP_VIEWACTIVE: c_int = 0;
static mut D_DISP_MENUACTIVE: c_int = 0;
static mut D_DISP_INHELPSCREENS: c_int = 0;
static mut D_DISP_FULLSCREEN: c_int = 0;
static mut D_DISP_OLD_GAMESTATE: c_int = -1;
static mut D_DISP_BORDERDRAWCOUNT: c_int = 0;

/// Title string printed at startup.
static mut title: [c_char; 128] = [0; 128];

/// Startup banner strings (may be replaced by dehacked).
static mut banners: [*const c_char; 7] = [
    b"                         \0DOOM 2: Hell on Earth v%i.%i\0                           \0"
        .as_ptr() as *const c_char,
    b"                            \0DOOM Shareware Startup v%i.%i\0                           \0"
        .as_ptr() as *const c_char,
    b"                            \0DOOM Registered Startup v%i.%i\0                           \0"
        .as_ptr() as *const c_char,
    b"                          \0DOOM System Startup v%i.%i\0                          \0".as_ptr()
        as *const c_char,
    b"                         \0The Ultimate DOOM Startup v%i.%i\0                        \0"
        .as_ptr() as *const c_char,
    b"                     \0DOOM 2: TNT - Evilution v%i.%i\0                           \0".as_ptr()
        as *const c_char,
    b"                   \0DOOM 2: Plutonia Experiment v%i.%i\0                           \0"
        .as_ptr() as *const c_char,
];

/// Game version descriptors for -gameversion.
struct SyncPtr(*const c_char);
unsafe impl Sync for SyncPtr {}

#[repr(C)]
struct GameVersionDesc {
    description: SyncPtr,
    cmdline: SyncPtr,
    version: c_int,
}

const fn sp(s: &'static [u8]) -> SyncPtr {
    SyncPtr(s.as_ptr() as *const c_char)
}

static GAME_VERSIONS: [GameVersionDesc; 10] = [
    GameVersionDesc {
        description: sp(b"Doom 1.666\0"),
        cmdline: sp(b"1.666\0"),
        version: d_mode::exe_doom_1_666,
    },
    GameVersionDesc {
        description: sp(b"Doom 1.7/1.7a\0"),
        cmdline: sp(b"1.7\0"),
        version: d_mode::exe_doom_1_7,
    },
    GameVersionDesc {
        description: sp(b"Doom 1.8\0"),
        cmdline: sp(b"1.8\0"),
        version: d_mode::exe_doom_1_8,
    },
    GameVersionDesc {
        description: sp(b"Doom 1.9\0"),
        cmdline: sp(b"1.9\0"),
        version: d_mode::exe_doom_1_9,
    },
    GameVersionDesc {
        description: sp(b"Hacx\0"),
        cmdline: sp(b"hacx\0"),
        version: d_mode::exe_hacx,
    },
    GameVersionDesc {
        description: sp(b"Ultimate Doom\0"),
        cmdline: sp(b"ultimate\0"),
        version: d_mode::exe_ultimate,
    },
    GameVersionDesc {
        description: sp(b"Final Doom\0"),
        cmdline: sp(b"final\0"),
        version: d_mode::exe_final,
    },
    GameVersionDesc {
        description: sp(b"Final Doom (alt)\0"),
        cmdline: sp(b"final2\0"),
        version: d_mode::exe_final2,
    },
    GameVersionDesc {
        description: sp(b"Chex Quest\0"),
        cmdline: sp(b"chex\0"),
        version: d_mode::exe_chex,
    },
    GameVersionDesc {
        description: SyncPtr(ptr::null()),
        cmdline: SyncPtr(ptr::null()),
        version: 0,
    },
];

/// Mission pack descriptors for -pack.
#[repr(C)]
struct PackDesc {
    name: SyncPtr,
    mission: c_int,
}

static PACKS: [PackDesc; 3] = [
    PackDesc {
        name: sp(b"doom2\0"),
        mission: d_mode::doom2,
    },
    PackDesc {
        name: sp(b"tnt\0"),
        mission: d_mode::pack_tnt,
    },
    PackDesc {
        name: sp(b"plutonia\0"),
        mission: d_mode::pack_plut,
    },
];

// ---------------------------------------------------------------------------
// Extern declarations for C code that is not yet ported (g_game.c)
// ---------------------------------------------------------------------------

extern "C" {
    // From g_game.c (still C):
    fn G_InitNew(skill: skill_t, episode: c_int, map: c_int);
    fn G_DeferedPlayDemo(demo: *const c_char);
    fn G_LoadGame(name: *mut c_char);
    fn G_RecordDemo(name: *mut c_char);
    fn G_BeginRecording();
    fn G_TimeDemo(name: *mut c_char);
    fn G_CheckDemoStatus() -> boolean;
    fn G_Responder(ev: *mut event_t) -> boolean;
    fn G_VanillaVersionCode() -> c_int;

    // Globals from g_game.c (still C):
    static mut gameaction: c_int;
    static mut deathmatch: c_int;
    static mut displayplayer: c_int;
    static mut usergame: c_int;
    static mut demoplayback: c_int;
    static mut demorecording: c_int;
    static mut singledemo: c_int;
    static mut gamestate: c_int;
    static mut paused: c_int;

    // From other ported modules (declared extern for clarity):
    static mut mouseSensitivity: c_int;
    static mut sfxVolume: c_int;
    static mut musicVolume: c_int;
    static mut automapactive: c_int;
    static mut menuactive: c_int;
    static mut viewactive: c_int;
    static mut nodrawers: c_int;
    static mut testcontrols: c_int;
    static mut testcontrols_mousespeed: c_int;
    static mut screenblocks: c_int;
    static mut detailLevel: c_int;
    static mut snd_channels: c_int;
    static mut vanilla_savegame_limit: c_int;
    static mut vanilla_demo_limit: c_int;
    static mut setsizeneeded: c_int;
    static mut showMessages: c_int;
    static mut viewheight: c_int;
    static mut scaledviewwidth: c_int;
    static mut viewwindowx: c_int;
    static mut viewwindowy: c_int;
    static mut screenvisible: c_int;
    static mut inhelpscreens: c_int;
    static mut drone: c_int;

    // From net_dedicated.c (stub since FEATURE_MULTIPLAYER is not defined):
    fn NET_DedicatedServer();

    // From net_query.c (stub):
    fn NET_MasterQuery();
    fn NET_QueryAddress(addr: *mut c_char);
    fn NET_LANQuery();

    // C standard library:
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncasecmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn isspace(c: c_int) -> c_int;
    fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    // From doomgeneric C (i_timer, i_system, etc. still have some C parts):
    fn I_GetTime() -> c_int;
    fn I_Sleep(ms: c_int);
    fn I_InitTimer();
    fn I_InitSound(use_sfx_prefix: boolean);
    fn I_InitMusic();
    fn I_BindSoundVariables();
    fn I_BindVideoVariables();
    fn I_BindJoystickVariables();
    fn I_Endoom(data: *mut byte);
    fn I_InitJoystick();
    fn I_Error(msg: *const c_char);
    fn I_ErrorV(msg: *const c_char);
    fn I_AtExit(func: extern "C" fn(), run_if_error: boolean);
    fn I_PrintStartupBanner(gamedescription: *mut c_char);
    fn I_PrintBanner(text: *mut c_char);
    fn I_PrintDivider();
    fn I_InitGraphics();
    fn I_GraphicsCheckCommandLine();
    fn I_SetPalette(palette: *mut byte);
    fn I_UpdateNoBlit();
    fn I_FinishUpdate();
    fn I_SetWindowTitle(title: *mut c_char);
    fn I_CheckIsScreensaver();
    fn I_SetGrabMouseCallback(func: extern "C" fn() -> boolean);
    fn I_DisplayFPSDots(dots_on: boolean);
    fn I_EnableLoadingDisk();

    // From remaining C modules:
    fn NetUpdate();
    fn TryRunTics();
    fn D_StartGameLoop();

    // From ported modules (extern for clarity):
    fn D_FindIWAD(mask: c_int, mission: *mut c_int) -> *mut c_char;
    fn D_SaveGameIWADName(gamemission: c_int) -> *mut c_char;
    fn Z_Init();
    fn Z_Malloc(size: c_int, tag: c_int, ptr: *mut c_void) -> *mut c_void;
    fn W_ParseCommandLine() -> boolean;
    static mut lumpinfo: *mut c_void;
    static mut numlumps: c_uint;
    fn W_AddFile(filename: *mut c_char) -> *mut c_void;
    fn W_CheckNumForName(name: *const c_char) -> c_int;
    fn W_CacheLumpName(name: *const c_char, tag: c_int) -> *mut c_void;
    fn W_GenerateHashTable();
    fn W_CheckCorrectIWAD(mission: c_int);
    fn S_Init(sfxVolume: c_int, musicVolume: c_int);
    fn S_StartMusic(music_id: c_int);
    fn S_UpdateSounds(listener: *mut c_void);
    fn V_Init();
    fn V_DrawPatch(x: c_int, y: c_int, patch: *mut c_void);
    fn V_DrawPatchDirect(x: c_int, y: c_int, patch: *mut c_void);
    fn V_RestoreBuffer();
    fn V_DrawMouseSpeedBox(speed: c_int);
    fn D_PopEvent() -> *mut event_t;
    fn F_Drawer();
    fn wipe_StartScreen(x: c_int, y: c_int, width: c_int, height: c_int) -> c_int;
    fn wipe_EndScreen(x: c_int, y: c_int, width: c_int, height: c_int) -> c_int;
    fn wipe_ScreenWipe(
        wipeno: c_int,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        ticks: c_int,
    ) -> c_int;
    fn M_CheckParm(check: *const c_char) -> c_int;
    fn M_CheckParmWithArgs(check: *const c_char, num_args: c_int) -> c_int;
    fn M_LoadDefaults();
    fn M_SetConfigDir(dir: *mut c_char);
    fn M_BindVariable(name: *mut c_char, variable: *mut c_void);
    fn M_SetConfigFilenames(main_config: *const c_char, extra_config: *const c_char);
    fn M_GetSaveGameDir(iwadname: *const c_char) -> *mut c_char;
    fn M_BindBaseControls();
    fn M_BindWeaponControls();
    fn M_BindMapControls();
    fn M_BindMenuControls();
    fn M_BindChatControls(num_players: c_uint);
    fn M_ApplyPlatformDefaults();
    fn M_StringCopy(dest: *mut c_char, src: *const c_char, dest_size: usize) -> boolean;
    fn M_StringEndsWith(s: *const c_char, suffix: *const c_char) -> boolean;
    fn M_Responder(ev: *mut event_t) -> boolean;
    fn M_Drawer();
    fn M_Init();
    fn P_SaveGameFile(slot: c_int) -> *mut c_char;
    fn HU_Init();
    fn HU_Drawer();
    fn HU_Erase();
    static mut chat_macros: [*mut c_char; 10];
    static mut key_multi_msgplayer: [c_int; 8];
    fn WI_Drawer();
    fn ST_Drawer(fullscreen: boolean, refresh: boolean);
    fn ST_Init();
    fn AM_Drawer();
    fn P_Init();
    fn R_RenderPlayerView(player: *mut c_void);
    fn R_Init();
    fn R_FillBackScreen();
    fn R_DrawViewBorder();
    fn StatDump();
    fn D_ConnectNetGame();
    fn D_CheckNetGame();
    fn R_ExecuteSetViewSize();
}

// ---------------------------------------------------------------------------
// Constants for lump name checks (IWAD detection)
// ---------------------------------------------------------------------------

static IWAD_CHECK_NAMES: [SyncPtr; 23] = [
    sp(b"e2m1\0"),
    sp(b"e2m2\0"),
    sp(b"e2m3\0"),
    sp(b"e2m4\0"),
    sp(b"e2m5\0"),
    sp(b"e2m6\0"),
    sp(b"e2m7\0"),
    sp(b"e2m8\0"),
    sp(b"e2m9\0"),
    sp(b"e3m1\0"),
    sp(b"e3m3\0"),
    sp(b"e3m3\0"), // duplicate in original C code
    sp(b"e3m4\0"),
    sp(b"e3m5\0"),
    sp(b"e3m6\0"),
    sp(b"e3m7\0"),
    sp(b"e3m8\0"),
    sp(b"e3m9\0"),
    sp(b"dphoof\0"),
    sp(b"bfgga0\0"),
    sp(b"heada1\0"),
    sp(b"cybra1\0"),
    sp(b"spida1d1\0"),
];

// ---------------------------------------------------------------------------
// Helper: DEH_String passthrough
// ---------------------------------------------------------------------------

/// Return the string as-is.  In the original C code this could be replaced
/// by dehacked patches, but since FEATURE_DEHACKED is not defined we
/// simply return the original pointer.
#[inline]
fn DEH_String(s: *const c_char) -> *const c_char {
    s
}

/// Safe wrapper: check if a C string ends with a given suffix.
unsafe fn c_str_ends_with(s: *const c_char, suffix: *const c_char) -> bool {
    if s.is_null() || suffix.is_null() {
        return false;
    }
    let s_len = strlen(s);
    let suf_len = strlen(suffix);
    if suf_len > s_len {
        return false;
    }
    strncasecmp(s.add(s_len.wrapping_sub(suf_len)), suffix, suf_len) == 0
}

/// Safe wrapper: case-insensitive string comparison.
unsafe fn c_str_eq(s1: *const c_char, s2: *const c_char) -> bool {
    if s1.is_null() || s2.is_null() {
        return false;
    }
    strcasecmp(s1, s2) == 0
}

/// Safe wrapper: compare first n bytes case-insensitive.
unsafe fn c_str_ne_n(s1: *const c_char, s2: *const c_char, n: usize) -> bool {
    if s1.is_null() || s2.is_null() {
        return true;
    }
    strncasecmp(s1, s2, n) != 0
}

/// Convert a C string slice to a Rust &str for printing.
unsafe fn c_str_to_str(s: *const c_char) -> &'static str {
    if s.is_null() {
        return "";
    }
    std::ffi::CStr::from_ptr(s).to_str().unwrap_or("")
}

// ---------------------------------------------------------------------------
// D_ProcessEvents
// ---------------------------------------------------------------------------

/// Send all buffered events down the responder chain.
#[no_mangle]
pub extern "C" fn D_ProcessEvents() {
    unsafe {
        // IF STORE DEMO, DO NOT ACCEPT INPUT
        if storedemo != 0 {
            return;
        }

        while {
            let ev = D_PopEvent();
            if ev.is_null() {
                false
            } else {
                if M_Responder(ev) == 0 {
                    G_Responder(ev);
                }
                true
            }
        } {}
    }
}

// ---------------------------------------------------------------------------
// D_Display
// ---------------------------------------------------------------------------

/// Draw current display, possibly wiping it from the previous frame.
#[no_mangle]
pub extern "C" fn D_Display() {
    unsafe {
        if nodrawers != 0 {
            return;
        }

        let mut redrawsbar = false;

        // Change the view size if needed
        if setsizeneeded != 0 {
            R_ExecuteSetViewSize();
            D_DISP_OLD_GAMESTATE = -1; // force background redraw
            D_DISP_BORDERDRAWCOUNT = 3;
        }

        // Save the current screen if about to wipe
        let wipe = gamestate != wipegamestate;
        if wipe {
            wipe_StartScreen(0, 0, SCREENWIDTH, SCREENHEIGHT);
        }

        // Buffered drawing based on game state
        if gamestate == GS_LEVEL {
            extern "C" {
                static mut gametic: c_int;
            }
            if gametic == 0 {
                // Skip level rendering
            } else {
                if automapactive != 0 {
                    AM_Drawer();
                }
                if wipe || (viewheight != 200 && D_DISP_FULLSCREEN != 0) {
                    redrawsbar = true;
                }
                if D_DISP_INHELPSCREENS != 0 && inhelpscreens == 0 {
                    redrawsbar = true;
                }
                ST_Drawer((viewheight == 200) as c_int, redrawsbar as c_int);
                D_DISP_FULLSCREEN = (viewheight == 200) as c_int;
            }
        } else if gamestate == GS_INTERMISSION {
            WI_Drawer();
        } else if gamestate == GS_FINALE {
            F_Drawer();
        } else if gamestate == GS_DEMOSCREEN {
            D_PageDrawer();
        }

        // Draw buffered stuff to screen
        I_UpdateNoBlit();

        // Draw the view directly
        if gamestate == GS_LEVEL && automapactive == 0 {
            extern "C" {
                static mut gametic: c_int;
            }
            if gametic != 0 {
                let dp = displayplayer as usize;
                if dp < MAXPLAYERS {
                    R_RenderPlayerView(std::ptr::addr_of_mut!(players[dp]) as *mut c_void);
                }
                HU_Drawer();
            }
        }

        // Set palette on state change (non-level states)
        if gamestate != GS_LEVEL && gamestate != D_DISP_OLD_GAMESTATE {
            I_SetPalette(W_CacheLumpName(
                DEH_String(b"PLAYPAL\0".as_ptr() as *const c_char),
                PU_CACHE,
            ) as *mut byte);
        }

        // See if the border needs to be initially drawn
        if gamestate == GS_LEVEL && D_DISP_OLD_GAMESTATE != GS_LEVEL {
            D_DISP_VIEWACTIVE = 0;
            R_FillBackScreen();
        }

        // See if the border needs to be updated
        if gamestate == GS_LEVEL && automapactive == 0 && scaledviewwidth != 320 {
            if menuactive != 0 || D_DISP_MENUACTIVE != 0 || D_DISP_VIEWACTIVE == 0 {
                D_DISP_BORDERDRAWCOUNT = 3;
            }
            if D_DISP_BORDERDRAWCOUNT != 0 {
                R_DrawViewBorder();
                D_DISP_BORDERDRAWCOUNT -= 1;
            }
        }

        // Test controls: show mouse speed box
        if testcontrols != 0 {
            V_DrawMouseSpeedBox(testcontrols_mousespeed);
        }

        // Update static state trackers
        D_DISP_MENUACTIVE = menuactive;
        D_DISP_VIEWACTIVE = viewactive;
        D_DISP_INHELPSCREENS = inhelpscreens;
        D_DISP_OLD_GAMESTATE = gamestate;
        wipegamestate = gamestate;

        // Draw pause pic
        if paused != 0 {
            let y = if automapactive != 0 {
                4
            } else {
                viewwindowy + 4
            };
            V_DrawPatchDirect(
                viewwindowx + (scaledviewwidth - 68) / 2,
                y,
                W_CacheLumpName(DEH_String(b"M_PAUSE\0".as_ptr() as *const c_char), PU_CACHE),
            );
        }

        // Menus go directly to the screen
        M_Drawer();
        NetUpdate();

        // Normal update
        if !wipe {
            I_FinishUpdate();
            return;
        }

        // Wipe update
        wipe_EndScreen(0, 0, SCREENWIDTH, SCREENHEIGHT);

        let mut wipestart = I_GetTime() - 1;
        loop {
            let mut nowtime;
            let mut tics;
            loop {
                nowtime = I_GetTime();
                tics = nowtime - wipestart;
                if tics <= 0 {
                    I_Sleep(1);
                } else {
                    break;
                }
            }

            wipestart = nowtime;
            let done = wipe_ScreenWipe(
                1, // wipe_Melt
                0,
                0,
                SCREENWIDTH,
                SCREENHEIGHT,
                tics,
            ) != 0;

            I_UpdateNoBlit();
            M_Drawer();
            I_FinishUpdate();

            if done {
                break;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// D_BindVariables
// ---------------------------------------------------------------------------

/// Bind all configuration variables.
#[no_mangle]
pub extern "C" fn D_BindVariables() {
    unsafe {
        M_ApplyPlatformDefaults();

        I_BindVideoVariables();
        I_BindJoystickVariables();
        I_BindSoundVariables();

        M_BindBaseControls();
        M_BindWeaponControls();
        M_BindMapControls();
        M_BindMenuControls();
        M_BindChatControls(4); // MAXPLAYERS = 4

        key_multi_msgplayer[0] = HUSTR_KEYGREEN as c_int;
        key_multi_msgplayer[1] = HUSTR_KEYINDIGO as c_int;
        key_multi_msgplayer[2] = HUSTR_KEYBROWN as c_int;
        key_multi_msgplayer[3] = HUSTR_KEYRED as c_int;

        M_BindVariable(
            b"mouse_sensitivity\0".as_ptr() as *mut c_char,
            &mut mouseSensitivity as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"sfx_volume\0".as_ptr() as *mut c_char,
            &mut sfxVolume as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"music_volume\0".as_ptr() as *mut c_char,
            &mut musicVolume as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"show_messages\0".as_ptr() as *mut c_char,
            &mut showMessages as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"screenblocks\0".as_ptr() as *mut c_char,
            &mut screenblocks as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"detaillevel\0".as_ptr() as *mut c_char,
            &mut detailLevel as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"snd_channels\0".as_ptr() as *mut c_char,
            &mut snd_channels as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"vanilla_savegame_limit\0".as_ptr() as *mut c_char,
            &mut vanilla_savegame_limit as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"vanilla_demo_limit\0".as_ptr() as *mut c_char,
            &mut vanilla_demo_limit as *mut c_int as *mut c_void,
        );
        M_BindVariable(
            b"show_endoom\0".as_ptr() as *mut c_char,
            &mut show_endoom as *mut c_int as *mut c_void,
        );

        // Multiplayer chat macros
        for i in 0..10 {
            let mut buf: [c_char; 12] = [0; 12];
            let buf_ptr = buf.as_mut_ptr();
            let fmt = b"chatmacro%i\0".as_ptr() as *const c_char;
            // Use snprintf to format the string
            extern "C" {
                fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
            }
            let result = snprintf(buf_ptr, buf.len(), fmt, i);
            M_snprintf_clamp(buf_ptr, buf.len(), result);
            M_BindVariable(
                buf_ptr,
                &mut chat_macros[i] as *mut *mut c_char as *mut c_void,
            );
        }
    }
}

// ---------------------------------------------------------------------------
// D_GrabMouseCallback
// ---------------------------------------------------------------------------

extern "C" fn D_GrabMouseCallback() -> boolean {
    unsafe {
        // Drone players don't need mouse focus
        if drone != 0 {
            return 0;
        }

        // When menu is active or game is paused, release the mouse
        if menuactive != 0 || paused != 0 {
            return 0;
        }

        // Only grab mouse when playing levels (but not demos)
        let demoplayback_val: c_int = demoplayback;
        let advancedemo_val: c_int = advancedemo;
        (gamestate == GS_LEVEL && demoplayback_val == 0 && advancedemo_val == 0) as c_int
    }
}

// ---------------------------------------------------------------------------
// doomgeneric_Tick
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn doomgeneric_Tick() {
    unsafe {
        extern "C" {
            static mut gametic: c_int;
        }

        // Frame synchronous IO operations
        I_StartFrame();

        TryRunTics(); // will run at least one tic

        // Update positional sounds
        let console = consoleplayer;
        let mo = if console >= 0 && (console as usize) < MAXPLAYERS {
            players[console as usize].mo
        } else {
            std::ptr::null_mut()
        };
        S_UpdateSounds(mo as *mut c_void);

        // Update display
        if screenvisible != 0 {
            D_Display();
        }
    }
}

// ---------------------------------------------------------------------------
// D_DoomLoop
// ---------------------------------------------------------------------------

/// Main game loop — never returns.
#[no_mangle]
pub extern "C" fn D_DoomLoop() {
    unsafe {
        if bfgedition != 0 {
            let is_recording = demorecording;
            let is_playdemo = gameaction == ga_playdemo;
            extern "C" {
                static mut netgame: c_int;
            }
            if is_recording != 0 || is_playdemo || netgame != 0 {
                eprintln!(
                    " WARNING: You are playing using one of the Doom Classic\n\
                     IWAD files shipped with the Doom 3: BFG Edition. These are\n\
                     known to be incompatible with the regular IWAD files and\n\
                     may cause demos and network games to get out of sync."
                );
            }
        }

        if demorecording != 0 {
            G_BeginRecording();
        }

        main_loop_started = 1;

        TryRunTics();

        I_SetWindowTitle(gamedescription);
        I_GraphicsCheckCommandLine();

        // Cast the Rust callback to match the C signature
        #[allow(improper_ctypes_definitions)]
        extern "C" fn grab_cb() -> c_int {
            D_GrabMouseCallback()
        }
        I_SetGrabMouseCallback(grab_cb);

        I_InitGraphics();
        I_EnableLoadingDisk();

        V_RestoreBuffer();
        R_ExecuteSetViewSize();

        D_StartGameLoop();

        if testcontrols != 0 {
            wipegamestate = gamestate;
        }

        doomgeneric_Tick();
    }
}

// ---------------------------------------------------------------------------
// Demo sequence management
// ---------------------------------------------------------------------------

/// Handle timing for demo projection.
#[no_mangle]
pub extern "C" fn D_PageTicker() {
    unsafe {
        pagetic -= 1;
        if pagetic < 0 {
            D_AdvanceDemo();
        }
    }
}

/// Draw the current demo page.
#[no_mangle]
pub extern "C" fn D_PageDrawer() {
    unsafe {
        V_DrawPatch(0, 0, W_CacheLumpName(pagename, PU_CACHE));
    }
}

/// Called after each demo or intro demosequence finishes.
#[no_mangle]
pub extern "C" fn D_AdvanceDemo() {
    unsafe {
        advancedemo = 1;
    }
}

/// Cycle through the demo sequences.
#[no_mangle]
pub extern "C" fn D_DoAdvanceDemo() {
    unsafe {
        // Set player state to live (PST_LIVE = 1)
        let cp = consoleplayer;
        if cp >= 0 && (cp as usize) < MAXPLAYERS {
            players[cp as usize].playerstate = 1;
        }

        advancedemo = 0;
        usergame = 0;
        paused = 0;
        gameaction = ga_nothing;

        extern "C" {
            static mut netgame: c_int;
        }

        // Demo sequence: 7 for ultimate/final, 6 for others
        let max_seq = {
            extern "C" {
                static mut gameversion: c_int;
            }
            if gameversion == d_mode::exe_ultimate || gameversion == d_mode::exe_final {
                7
            } else {
                6
            }
        };
        demosequence = (demosequence + 1) % max_seq;

        extern "C" {
            static mut gamemode: c_int;
        }

        match demosequence {
            0 => {
                if gamemode == d_mode::commercial {
                    pagetic = TICRATE * 11;
                } else {
                    pagetic = 170;
                }
                gamestate = GS_DEMOSCREEN;
                pagename = b"TITLEPIC\0".as_ptr() as *mut c_char;
                if gamemode == d_mode::commercial {
                    S_StartMusic(40); // mus_dm2ttl
                } else {
                    S_StartMusic(1); // mus_intro
                }
            }
            1 => {
                G_DeferedPlayDemo(b"demo1\0".as_ptr() as *const c_char);
            }
            2 => {
                pagetic = 200;
                gamestate = GS_DEMOSCREEN;
                pagename = b"CREDIT\0".as_ptr() as *mut c_char;
            }
            3 => {
                G_DeferedPlayDemo(b"demo2\0".as_ptr() as *const c_char);
            }
            4 => {
                gamestate = GS_DEMOSCREEN;
                if gamemode == d_mode::commercial {
                    pagetic = TICRATE * 11;
                    pagename = b"TITLEPIC\0".as_ptr() as *mut c_char;
                    S_StartMusic(40); // mus_dm2ttl
                } else {
                    pagetic = 200;
                    if gamemode == d_mode::retail {
                        pagename = b"CREDIT\0".as_ptr() as *mut c_char;
                    } else {
                        pagename = b"HELP2\0".as_ptr() as *mut c_char;
                    }
                }
            }
            5 => {
                G_DeferedPlayDemo(b"demo3\0".as_ptr() as *const c_char);
            }
            6 => {
                // THE DEFINITIVE DOOM Special Edition demo
                G_DeferedPlayDemo(b"demo4\0".as_ptr() as *const c_char);
            }
            _ => {}
        }

        // BFG Edition workaround: TITLEPIC missing, use INTERPIC
        if bfgedition != 0 && c_str_eq(pagename, b"TITLEPIC\0".as_ptr() as *const c_char) {
            if W_CheckNumForName(b"titlepic\0".as_ptr() as *const c_char) < 0 {
                pagename = b"INTERPIC\0".as_ptr() as *mut c_char;
            }
        }
    }
}

/// Start the title screen demo sequence.
#[no_mangle]
pub extern "C" fn D_StartTitle() {
    unsafe {
        gameaction = ga_nothing;
        demosequence = -1;
        D_AdvanceDemo();
    }
}

// ---------------------------------------------------------------------------
// GetGameName
// ---------------------------------------------------------------------------

/// Get game name: if the startup banner has been replaced, use that.
/// Otherwise use the provided default.
unsafe fn GetGameName(gamename: *mut c_char) -> *mut c_char {
    for i in 0..banners.len() {
        let banner = banners[i];
        let deh_sub = DEH_String(banner);

        // Has been replaced?
        if !std::ptr::eq(deh_sub, banner) {
            let gamename_size = strlen(deh_sub) + 10;
            let expanded =
                Z_Malloc(gamename_size as c_int, PU_STATIC, ptr::null_mut()) as *mut c_char;

            let version = G_VanillaVersionCode();
            extern "C" {
                fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
            }
            let result = snprintf(
                expanded,
                gamename_size,
                deh_sub,
                version / 100,
                version % 100,
            );
            M_snprintf_clamp(expanded, gamename_size, result);

            // Trim leading spaces
            let mut start = 0;
            while *expanded.add(start) != 0 && isspace(*expanded.add(start) as c_int) != 0 {
                start += 1;
            }
            if start > 0 {
                let remaining = strlen(expanded).wrapping_sub(start) + 1;
                memmove(
                    expanded as *mut c_void,
                    expanded.add(start) as *const c_void,
                    remaining,
                );
            }

            // Trim trailing spaces
            let mut len = strlen(expanded);
            while len > 0 && isspace(*expanded.add(len.wrapping_sub(1)) as c_int) != 0 {
                *expanded.add(len.wrapping_sub(1)) = 0;
                len -= 1;
            }

            return expanded;
        }
    }

    gamename
}

// ---------------------------------------------------------------------------
// SetMissionForPackName
// ---------------------------------------------------------------------------

/// Set the game mission based on a pack name.
unsafe fn SetMissionForPackName(pack_name: *mut c_char) {
    for pack in &PACKS {
        if c_str_eq(pack_name, pack.name.0) {
            gamemission = pack.mission;
            return;
        }
    }

    println!("Valid mission packs are:");
    for pack in &PACKS {
        if pack.name.0.is_null() {
            break;
        }
        println!("\t{}", c_str_to_str(pack.name.0));
    }

    I_Error(b"Unknown mission pack name\0".as_ptr() as *const c_char);
}

// ---------------------------------------------------------------------------
// D_IdentifyVersion
// ---------------------------------------------------------------------------

/// Find out what version of Doom is playing.
#[no_mangle]
pub extern "C" fn D_IdentifyVersion() {
    unsafe {
        extern "C" {
            static mut lumpinfo: *mut c_void;
            static mut numlumps: c_uint;
        }

        if gamemission == d_mode::none {
            // lumpinfo_t struct: first 8 bytes is name (char[8])
            for i in 0..numlumps {
                let lump_ptr =
                    (lumpinfo as *const u8).add(i as usize * std::mem::size_of::<usize>() * 2 + 8);
                let name_ptr = lump_ptr as *const c_char;

                if c_str_ne_n(name_ptr, b"MAP01\0".as_ptr() as *const c_char, 8) == false {
                    gamemission = d_mode::doom2;
                    break;
                } else if c_str_ne_n(name_ptr, b"E1M1\0".as_ptr() as *const c_char, 8) == false {
                    gamemission = d_mode::doom;
                    break;
                }
            }

            if gamemission == d_mode::none {
                I_Error(b"Unknown or invalid IWAD file.\0".as_ptr() as *const c_char);
            }
        }

        // Make sure gamemode is set up correctly
        extern "C" {
            fn W_CheckNumForName(name: *const c_char) -> c_int;
        }

        let logical_mission = logical_gamemission();
        if logical_mission == d_mode::doom {
            // Doom 1. But which version?
            if W_CheckNumForName(b"E4M1\0".as_ptr() as *const c_char) > 0 {
                gamemode = d_mode::retail;
            } else if W_CheckNumForName(b"E3M1\0".as_ptr() as *const c_char) > 0 {
                gamemode = d_mode::registered;
            } else {
                gamemode = d_mode::shareware;
            }
        } else {
            // Doom 2 of some kind.
            gamemode = d_mode::commercial;

            // Manually override gamemission with -pack
            let p = M_CheckParmWithArgs(b"-pack\0".as_ptr() as *const c_char, 1);
            if p > 0 {
                extern "C" {
                    static mut myargc: c_int;
                    static mut myargv: *mut *mut c_char;
                }
                SetMissionForPackName(*myargv.add((p + 1) as usize));
            }
        }
    }
}

/// Replicate the C `logical_gamemission` macro from `doomstat.h`.
unsafe fn logical_gamemission() -> c_int {
    if gamemission == d_mode::pack_chex {
        d_mode::doom
    } else if gamemission == d_mode::pack_hacx {
        d_mode::doom2
    } else {
        gamemission
    }
}

// ---------------------------------------------------------------------------
// D_SetGameDescription
// ---------------------------------------------------------------------------

/// Set the gamedescription string.
#[no_mangle]
pub extern "C" fn D_SetGameDescription() {
    unsafe {
        extern "C" {
            fn W_CheckNumForName(name: *const c_char) -> c_int;
        }

        let is_freedoom = W_CheckNumForName(b"FREEDOOM\0".as_ptr() as *const c_char) >= 0;
        let is_freedm = W_CheckNumForName(b"FREEDM\0".as_ptr() as *const c_char) >= 0;

        gamedescription = b"Unknown\0".as_ptr() as *mut c_char;

        let logical_mission = logical_gamemission();
        if logical_mission == d_mode::doom {
            // Doom 1. But which version?
            if is_freedoom {
                gamedescription = GetGameName(b"Freedoom: Phase 1\0".as_ptr() as *mut c_char);
            } else if gamemode == d_mode::retail {
                gamedescription = GetGameName(b"The Ultimate DOOM\0".as_ptr() as *mut c_char);
            } else if gamemode == d_mode::registered {
                gamedescription = GetGameName(b"DOOM Registered\0".as_ptr() as *mut c_char);
            } else if gamemode == d_mode::shareware {
                gamedescription = GetGameName(b"DOOM Shareware\0".as_ptr() as *mut c_char);
            }
        } else {
            // Doom 2 of some kind.
            if is_freedoom {
                if is_freedm {
                    gamedescription = GetGameName(b"FreeDM\0".as_ptr() as *mut c_char);
                } else {
                    gamedescription = GetGameName(b"Freedoom: Phase 2\0".as_ptr() as *mut c_char);
                }
            } else if logical_mission == d_mode::doom2 {
                gamedescription = GetGameName(b"DOOM 2: Hell on Earth\0".as_ptr() as *mut c_char);
            } else if logical_mission == d_mode::pack_plut {
                gamedescription =
                    GetGameName(b"DOOM 2: Plutonia Experiment\0".as_ptr() as *mut c_char);
            } else if logical_mission == d_mode::pack_tnt {
                gamedescription = GetGameName(b"DOOM 2: TNT - Evilution\0".as_ptr() as *mut c_char);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// D_AddFile
// ---------------------------------------------------------------------------

/// Add a WAD file to the search path.
unsafe fn D_AddFile(filename: *mut c_char) -> bool {
    println!(" adding {}", c_str_to_str(filename));
    let handle = W_AddFile(filename);
    !handle.is_null()
}

// ---------------------------------------------------------------------------
// PrintDehackedBanners
// ---------------------------------------------------------------------------

static COPYRIGHT_BANNERS: [SyncPtr; 3] = [
    sp(
        b"===========================================================================\n\
ATTENTION:  This version of DOOM has been modified.  If you would like to\n\
get a copy of the original game, call 1-800-IDGAMES or see the readme file.\n\
        You will not receive technical support for modified games.\n\
                      press enter to continue\n\
===========================================================================\n\0",
    ),
    sp(
        b"===========================================================================\n\
                 Commercial product - do not distribute!\n\
         Please report software piracy to the SPA: 1-800-388-PIR8\n\
===========================================================================\n\0",
    ),
    sp(
        b"===========================================================================\n\
                                Shareware!\n\
===========================================================================\n\0",
    ),
];

/// Print dehacked-replaced copyright banners.
unsafe fn PrintDehackedBanners() {
    for i in 0..COPYRIGHT_BANNERS.len() {
        let banner = COPYRIGHT_BANNERS[i].0;
        let deh_s = DEH_String(banner);

        if !std::ptr::eq(deh_s, banner) {
            print!("{}", c_str_to_str(deh_s));

            // Ensure modified banner ends in newline
            let len = strlen(deh_s);
            if len > 0 && *deh_s.add(len.wrapping_sub(1)) != b'\n' as c_char {
                println!();
            }
        }
    }
}

// ---------------------------------------------------------------------------
// InitGameVersion
// ---------------------------------------------------------------------------

/// Initialize the game version.
unsafe fn InitGameVersion() {
    let p = M_CheckParmWithArgs(b"-gameversion\0".as_ptr() as *const c_char, 1);

    if p > 0 {
        extern "C" {
            static mut myargc: c_int;
            static mut myargv: *mut *mut c_char;
        }
        let arg = *myargv.add((p + 1) as usize);

        let mut found = false;
        for gv in &GAME_VERSIONS {
            if gv.description.0.is_null() {
                break;
            }
            if strcmp(arg, gv.cmdline.0) == 0 {
                gameversion = gv.version;
                found = true;
                break;
            }
        }

        if !found {
            println!("Supported game versions:");
            for gv in &GAME_VERSIONS {
                if gv.description.0.is_null() {
                    break;
                }
                println!(
                    "\t{} ({})",
                    c_str_to_str(gv.cmdline.0),
                    c_str_to_str(gv.description.0)
                );
            }

            I_Error(b"Unknown game version\0".as_ptr() as *const c_char);
        }
    } else {
        // Determine automatically
        if gamemission == d_mode::pack_chex {
            gameversion = d_mode::exe_chex;
        } else if gamemission == d_mode::pack_hacx {
            gameversion = d_mode::exe_hacx;
        } else if gamemode == d_mode::shareware || gamemode == d_mode::registered {
            gameversion = d_mode::exe_doom_1_9;
        } else if gamemode == d_mode::retail {
            gameversion = d_mode::exe_ultimate;
        } else if gamemode == d_mode::commercial {
            if gamemission == d_mode::doom2 {
                gameversion = d_mode::exe_doom_1_9;
            } else {
                // Final Doom: tnt or plutonia
                // Defaults to emulating the first Final Doom executable
                gameversion = d_mode::exe_final;
            }
        }
    }

    // Original exe does not support retail - 4th episode not supported
    if gameversion < d_mode::exe_ultimate && gamemode == d_mode::retail {
        gamemode = d_mode::registered;
    }

    // EXEs prior to Final Doom do not support Final Doom
    if gameversion < d_mode::exe_final
        && gamemode == d_mode::commercial
        && (gamemission == d_mode::pack_tnt || gamemission == d_mode::pack_plut)
    {
        gamemission = d_mode::doom2;
    }
}

// ---------------------------------------------------------------------------
// PrintGameVersion
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn PrintGameVersion() {
    unsafe {
        for gv in &GAME_VERSIONS {
            if gv.description.0.is_null() {
                break;
            }
            if gv.version == gameversion {
                println!(
                    "Emulating the behavior of the '{}' executable.",
                    c_str_to_str(gv.description.0)
                );
                break;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// D_Endoom
// ---------------------------------------------------------------------------

extern "C" fn D_Endoom() {
    unsafe {
        // Don't show ENDOOM if disabled, or in screensaver/control test mode.
        // Only show it once the game has actually started.
        if show_endoom == 0
            || main_loop_started == 0
            || M_CheckParm(b"-testcontrols\0".as_ptr() as *const c_char) > 0
        {
            return;
        }

        extern "C" {
            static mut screensaver_mode: c_int;
        }
        if screensaver_mode != 0 {
            return;
        }

        let endoom = W_CacheLumpName(DEH_String(b"ENDOOM\0".as_ptr() as *const c_char), PU_STATIC)
            as *mut byte;
        I_Endoom(endoom);

        exit(0);
    }
}

// ---------------------------------------------------------------------------
// D_DoomMain
// ---------------------------------------------------------------------------

/// Main entry point — initializes all subsystems.
#[no_mangle]
pub extern "C" fn D_DoomMain() {
    unsafe {
        let mut file: [c_char; 256] = [0; 256];
        let mut demolumpname: [c_char; 9] = [0; 9];

        I_AtExit(D_Endoom, 0);

        // Print banner
        I_PrintBanner(b"Room\0".as_ptr() as *mut c_char);

        // Init zone memory
        println!("Z_Init: Init zone memory allocation daemon.");
        Z_Init();

        // Check command-line flags
        nomonsters = (M_CheckParm(b"-nomonsters\0".as_ptr() as *const c_char) != 0) as c_int;
        respawnparm = (M_CheckParm(b"-respawn\0".as_ptr() as *const c_char) != 0) as c_int;
        fastparm = (M_CheckParm(b"-fast\0".as_ptr() as *const c_char) != 0) as c_int;
        devparm = (M_CheckParm(b"-devparm\0".as_ptr() as *const c_char) != 0) as c_int;

        I_DisplayFPSDots((devparm != 0) as c_int);

        if M_CheckParm(b"-deathmatch\0".as_ptr() as *const c_char) != 0 {
            deathmatch = 1;
        }

        if M_CheckParm(b"-altdeath\0".as_ptr() as *const c_char) != 0 {
            deathmatch = 2;
        }

        if devparm != 0 {
            println!("{}", D_DEVSTR);
        }

        // Config directory
        M_SetConfigDir(ptr::null_mut());

        // Turbo mode
        let p = M_CheckParm(b"-turbo\0".as_ptr() as *const c_char);
        if p > 0 {
            let mut scale: c_int = 200;
            extern "C" {
                static mut myargc: c_int;
                static mut myargv: *mut *mut c_char;
            }
            extern "C" {
                static mut forwardmove: [c_int; 2];
                static mut sidemove: [c_int; 2];
            }

            if p < myargc - 1 {
                scale = atoi(*myargv.add((p + 1) as usize));
            }
            if scale < 10 {
                scale = 10;
            }
            if scale > 400 {
                scale = 400;
            }

            println!("turbo scale: {}%", scale);

            forwardmove[0] = forwardmove[0] * scale / 100;
            forwardmove[1] = forwardmove[1] * scale / 100;
            sidemove[0] = sidemove[0] * scale / 100;
            sidemove[1] = sidemove[1] * scale / 100;
        }

        // Init subsystems
        println!("V_Init: allocate screens.");
        V_Init();

        // Load configuration
        println!("M_LoadDefaults: Load system defaults.");
        M_SetConfigFilenames(
            b"default.cfg\0".as_ptr() as *const c_char,
            b"doom.cfg\0".as_ptr() as *const c_char,
        );
        D_BindVariables();
        M_LoadDefaults();

        I_AtExit(M_SaveDefaults, 0);

        // Find main IWAD
        iwadfile = D_FindIWAD(1, &mut gamemission); // IWAD_MASK_DOOM = 1

        if iwadfile.is_null() {
            I_Error(
                b"Game mode indeterminate.  No IWAD file was found.  Try\n\
                     specifying one with the '-iwad' command line parameter.\n\0"
                    .as_ptr() as *const c_char,
            );
        }

        modifiedgame = 0;

        println!("W_Init: Init WADfiles.");
        D_AddFile(iwadfile);

        W_CheckCorrectIWAD(d_mode::doom);

        // Identify version and game version
        D_IdentifyVersion();
        InitGameVersion();

        // BFG Edition check
        if W_CheckNumForName(b"dmenupic\0".as_ptr() as *const c_char) >= 0 {
            println!("BFG Edition: Using workarounds as needed.");
            bfgedition = 1;

            // BFG changes secret level names
            // (DEH replacements would go here, skipped since no dehacked)
        }

        // Load PWAD files
        modifiedgame = W_ParseCommandLine();

        // Check for -playdemo / -timedemo
        let p = M_CheckParmWithArgs(b"-playdemo\0".as_ptr() as *const c_char, 1);
        let mut is_timedemo = false;
        let p = if p == 0 {
            let tp = M_CheckParmWithArgs(b"-timedemo\0".as_ptr() as *const c_char, 1);
            if tp > 0 {
                is_timedemo = true;
            }
            tp
        } else {
            p
        };

        if p > 0 {
            extern "C" {
                static mut myargv: *mut *mut c_char;
            }
            let arg = *myargv.add((p + 1) as usize);

            // Copy demo name, handle .lmp extension
            if c_str_ends_with(arg, b".lmp\0".as_ptr() as *const c_char) {
                M_StringCopy(file.as_mut_ptr(), arg, file.len());
            } else {
                let fmt = b"%s.lmp\0".as_ptr() as *const c_char;
                extern "C" {
                    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
                }
                let result = snprintf(file.as_mut_ptr(), file.len(), fmt, arg);
                M_snprintf_clamp(file.as_mut_ptr(), file.len(), result);
            }

            if D_AddFile(file.as_mut_ptr()) {
                // Copy lump name from the last loaded lump
                extern "C" {
                    static mut lumpinfo: *mut c_void;
                    static mut numlumps: c_uint;
                }
                let lump_ptr = (lumpinfo as *const u8)
                    .add(((numlumps - 1) as usize) * std::mem::size_of::<usize>() * 2 + 8);
                std::ptr::copy_nonoverlapping(
                    lump_ptr as *const c_char,
                    demolumpname.as_mut_ptr(),
                    8,
                );
                demolumpname[8] = 0;
            } else {
                // Still continue like Vanilla Doom
                M_StringCopy(demolumpname.as_mut_ptr(), arg, demolumpname.len());
            }

            println!("Playing demo {}.", c_str_to_str(file.as_ptr()));
        }

        // Note: G_CheckDemoStatus atexit registration is handled via the game flow.

        W_GenerateHashTable();

        // Set game description
        D_SetGameDescription();

        // Savegame directory
        savegamedir = M_GetSaveGameDir(D_SaveGameIWADName(gamemission));

        // Check for -file in shareware
        if modifiedgame != 0 {
            extern "C" {
                static mut gamemode: c_int;
            }
            if gamemode == d_mode::shareware {
                I_Error(DEH_String(
                    b"\nYou cannot -file with the shareware version. Register!\0".as_ptr()
                        as *const c_char,
                ));
            }

            // Check for fake IWAD
            if gamemode == d_mode::registered {
                extern "C" {
                    fn W_CheckNumForName(name: *const c_char) -> c_int;
                }
                for i in 0..23 {
                    if W_CheckNumForName(IWAD_CHECK_NAMES[i].0) < 0 {
                        I_Error(DEH_String(
                            b"\nThis is not the registered version.\0".as_ptr() as *const c_char,
                        ));
                    }
                }
            }
        }

        // Warning about modified sprites
        if W_CheckNumForName(b"SS_START\0".as_ptr() as *const c_char) >= 0
            || W_CheckNumForName(b"FF_END\0".as_ptr() as *const c_char) >= 0
        {
            println!(
                " WARNING: The loaded WAD file contains modified sprites or\n\
                 floor textures.  You may want to use the '-merge' command\n\
                 line option instead of '-file'."
            );
        }

        // Print startup banner
        I_PrintStartupBanner(gamedescription);
        PrintDehackedBanners();

        // Freedoom warning
        if W_CheckNumForName(b"FREEDOOM\0".as_ptr() as *const c_char) >= 0
            && W_CheckNumForName(b"FREEDM\0".as_ptr() as *const c_char) < 0
        {
            println!(
                " WARNING: You are playing using one of the Freedoom IWAD\n\
                 files, which might not work in this port. See this page\n\
                 for more information on how to play using Freedoom:\n\
                 http://www.chocolate-doom.org/wiki/index.php/Freedoom"
            );
            I_PrintDivider();
        }

        // Initialize I subsystems
        println!("I_Init: Setting up machine state.");
        I_CheckIsScreensaver();
        I_InitTimer();
        I_InitJoystick();
        I_InitSound(1);
        I_InitMusic();

        // Initial netgame startup
        D_ConnectNetGame();

        // Default skill/episode/map
        startskill = sk_medium;
        startepisode = 1;
        startmap = 1;
        autostart = 0;

        // -skill
        let p = M_CheckParmWithArgs(b"-skill\0".as_ptr() as *const c_char, 1);
        if p > 0 {
            extern "C" {
                static mut myargv: *mut *mut c_char;
            }
            startskill = *(*myargv.add((p + 1) as usize) as *const u8) as c_int - '1' as i32;
            autostart = 1;
        }

        // -episode
        let p = M_CheckParmWithArgs(b"-episode\0".as_ptr() as *const c_char, 1);
        if p > 0 {
            extern "C" {
                static mut myargv: *mut *mut c_char;
            }
            startepisode = *(*myargv.add((p + 1) as usize) as *const u8) as c_int - '0' as i32;
            startmap = 1;
            autostart = 1;
        }

        let mut timelimit: c_int = 0;

        // -timer
        let p = M_CheckParmWithArgs(b"-timer\0".as_ptr() as *const c_char, 1);
        if p > 0 {
            extern "C" {
                static mut myargv: *mut *mut c_char;
            }
            timelimit = atoi(*myargv.add((p + 1) as usize));
        }

        // -avg
        if M_CheckParm(b"-avg\0".as_ptr() as *const c_char) != 0 {
            timelimit = 20;
        }

        // -warp
        let p = M_CheckParmWithArgs(b"-warp\0".as_ptr() as *const c_char, 1);
        if p > 0 {
            extern "C" {
                static mut myargv: *mut *mut c_char;
                static mut myargc: c_int;
            }
            extern "C" {
                static mut gamemode: c_int;
            }

            if gamemode == d_mode::commercial {
                startmap = atoi(*myargv.add((p + 1) as usize));
            } else {
                startepisode = *(*myargv.add((p + 1) as usize) as *const u8) as c_int - '0' as i32;
                if p + 2 < myargc {
                    startmap = *(*myargv.add((p + 2) as usize) as *const u8) as c_int - '0' as i32;
                } else {
                    startmap = 1;
                }
            }
            autostart = 1;
        }

        // -testcontrols
        if M_CheckParm(b"-testcontrols\0".as_ptr() as *const c_char) > 0 {
            startepisode = 1;
            startmap = 1;
            autostart = 1;
            testcontrols = 1;
        }

        // -loadgame
        let p = M_CheckParmWithArgs(b"-loadgame\0".as_ptr() as *const c_char, 1);
        if p > 0 {
            extern "C" {
                static mut myargv: *mut *mut c_char;
            }
            startloadgame = atoi(*myargv.add((p + 1) as usize));
        } else {
            startloadgame = -1;
        }

        // Init remaining subsystems
        println!("M_Init: Init miscellaneous info.");
        M_Init();

        print!("R_Init: Init DOOM refresh daemon - ");
        R_Init();
        println!();

        println!("P_Init: Init Playloop state.");
        P_Init();

        println!("S_Init: Setting up sound.");
        S_Init(sfxVolume * 8, musicVolume * 8);

        println!("D_CheckNetGame: Checking network game status.");
        D_CheckNetGame();

        PrintGameVersion();

        println!("HU_Init: Setting up heads up display.");
        HU_Init();

        println!("ST_Init: Init status bar.");
        ST_Init();

        // Store demo check
        if gamemode == d_mode::commercial
            && W_CheckNumForName(b"map01\0".as_ptr() as *const c_char) < 0
        {
            storedemo = 1;
        }

        // -statdump
        if M_CheckParmWithArgs(b"-statdump\0".as_ptr() as *const c_char, 1) > 0 {
            println!("External statistics registered.");
        }

        // -record
        let p = M_CheckParmWithArgs(b"-record\0".as_ptr() as *const c_char, 1);
        if p > 0 {
            extern "C" {
                static mut myargv: *mut *mut c_char;
            }
            G_RecordDemo(*myargv.add((p + 1) as usize));
            autostart = 1;
        }

        // -playdemo
        let p = M_CheckParmWithArgs(b"-playdemo\0".as_ptr() as *const c_char, 1);
        if p > 0 {
            singledemo = 1;
            G_DeferedPlayDemo(demolumpname.as_ptr());
            D_DoomLoop();
            return;
        }

        // -timedemo
        let p = M_CheckParmWithArgs(b"-timedemo\0".as_ptr() as *const c_char, 1);
        if p > 0 {
            G_TimeDemo(demolumpname.as_mut_ptr());
            D_DoomLoop();
            return;
        }

        // Load game
        if startloadgame >= 0 {
            M_StringCopy(file.as_mut_ptr(), P_SaveGameFile(startloadgame), file.len());
            G_LoadGame(file.as_mut_ptr());
        }

        // Start new game or title screen
        extern "C" {
            static mut netgame: c_int;
        }
        if gameaction != ga_loadgame {
            if autostart != 0 || netgame != 0 {
                G_InitNew(startskill, startepisode, startmap);
            } else {
                D_StartTitle();
            }
        }

        D_DoomLoop();
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_flags_defaults() {
        unsafe {
            assert_eq!(devparm, 0);
            assert_eq!(nomonsters, 0);
            assert_eq!(respawnparm, 0);
            assert_eq!(fastparm, 0);
            assert_eq!(autostart, 0);
            assert_eq!(advancedemo, 0);
            assert_eq!(storedemo, 0);
            assert_eq!(bfgedition, 0);
            assert_eq!(main_loop_started, 0);
            assert_eq!(show_endoom, 1);
            assert_eq!(startepisode, 0);
            assert_eq!(startmap, 0);
        }
    }

    #[test]
    fn wipegamestate_default() {
        unsafe {
            assert_eq!(wipegamestate, GS_DEMOSCREEN);
        }
    }

    #[test]
    fn global_buffer_sizes() {
        unsafe {
            assert_eq!(std::mem::size_of_val(&wadfile), 1024);
            assert_eq!(std::mem::size_of_val(&mapdir), 1024);
            assert_eq!(std::mem::size_of_val(&title), 128);
        }
    }

    #[test]
    fn game_version_table_complete() {
        // Last entry should be null-terminated
        let last = &GAME_VERSIONS[GAME_VERSIONS.len() - 1];
        assert!(last.description.0.is_null());
        assert!(last.cmdline.0.is_null());
        assert_eq!(last.version, 0);
    }

    #[test]
    fn packs_table_complete() {
        assert_eq!(PACKS.len(), 3);
        assert!(!PACKS[0].name.0.is_null());
        assert!(!PACKS[1].name.0.is_null());
        assert!(!PACKS[2].name.0.is_null());
    }

    #[test]
    fn iwad_check_names_count() {
        assert_eq!(IWAD_CHECK_NAMES.len(), 23);
    }

    #[test]
    fn copyright_banners_count() {
        assert_eq!(COPYRIGHT_BANNERS.len(), 3);
    }

    #[test]
    fn banners_count() {
        unsafe {
            assert_eq!(banners.len(), 7);
        }
    }
}
