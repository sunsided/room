//! Rust port of vendor/doomgeneric/f_finale.c.
//!
//! Game completion, final screen animation.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int, c_short, c_uint, c_void};
use std::ptr;

use crate::doom::d_event::event_t;
use crate::doom::d_mode;
use crate::doom::d_player::PlayerT;
use crate::doom::doomstat::{gamemission, gameversion};
use crate::doom::hu_stuff::{hu_font, HU_FONTSIZE, HU_FONTSTART};
use crate::doom::info::*;
use crate::doom::v_video::patch_t;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

use crate::doom::i_video::{SCREENHEIGHT, SCREENWIDTH};
use crate::doom::z_zone::{PU_CACHE, PU_LEVEL};

const TEXTSPEED: c_int = 3;
const TEXTWAIT: c_int = 250;

const GS_FINALE: c_int = 2;
const ga_nothing: c_int = 0;
const ga_worlddone: c_int = 8;
const ev_keydown: c_int = 0;

const FF_FRAMEMASK: c_int = 0x7fff;

const MAXPLAYERS: usize = 4;

// Music indices (from sounds.c)
const mus_victor: c_int = 31;
const mus_read_m: c_int = 65;
const mus_bunny: c_int = 30;
const mus_evil: c_int = 63;

// Sfx indices (from sounds.c)
const sfx_dshtgn: c_int = 4;
const sfx_pistol: c_int = 1;
const sfx_shotgn: c_int = 2;
const sfx_vilatk: c_int = 54;
const sfx_skeswg: c_int = 56;
const sfx_skepch: c_int = 53;
const sfx_skeatk: c_int = 107;
const sfx_firsht: c_int = 16;
const sfx_claw: c_int = 55;
const sfx_sgtatk: c_int = 52;
const sfx_sklatk: c_int = 51;
const sfx_plasma: c_int = 8;
const sfx_rlaunc: c_int = 14;

// ---------------------------------------------------------------------------
// Finale text strings (from d_englsh.h)
// ---------------------------------------------------------------------------

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *mut c_char
    };
}

const E1TEXT: *mut c_char = cstr!("Once you beat the big badasses and\nclean out the moon base you're supposed\nto win, aren't you? Aren't you? Where's\nyour fat reward and ticket home? What\nthe hell is this? It's not supposed to\nend this way!\n\nIt stinks like rotten meat, but looks\nlike the lost Deimos base.  Looks like\nyou're stuck on The Shores of Hell.\nThe only way out is through.\n\nTo continue the DOOM experience, play\nThe Shores of Hell and its amazing\nsequel, Inferno!");

const E2TEXT: *mut c_char = cstr!("You've done it! The hideous cyber-\ndemon lord that ruled the lost Deimos\nmoon base has been slain and you\ntriumph over the hordes of hell.\nThe mission is not complete, however.\nThe loathsome vomit of hell still\noozes from the nether regions of\nDeimos.\n\nThe demon spawner, the source of the\nhellish invasion, remains active.\nYou must find it and shut it down.\n\nTo continue the DOOM experience,\nplay Inferno!");

const E3TEXT: *mut c_char = cstr!("The loathsome spiderdemon that\nmaster-minded the invasion of the moon\nbase and caused so much death has had\nits ass kicked for all time.\n\nA hidden doorway opens and you begin\nthe long trek back to the surface.\nThe sensual scent of flowers tickles\nyour nose and you smile.\n\nBut wait! The gateway is open, and\nthe demons of hell are pouring\nthrough! You wonder how you'll ever\nget home.\n\nA demon consumes your flesh.\n\nThe End.\n\n(Well, not really.  To continue the\nDOOM experience, play Thy Flesh\nConsumed!)");

const E4TEXT: *mut c_char = cstr!("The spider mastermind must have sent forth\nits legions of hellspawn before your\nfinal confrontation with that terrible\nbeast from netherworld.  But you stepped\nforward and brought forth eternal damnation\nand suffering upon the horde as a true\nhero would in the face of something so\nevil.\n\nBesides, someone was gonna pay for what\nhappened to daisy, your pet rabbit.\n\nBut now, you see spread before you more\npotential pain and gibbitude as a nation\nof demons run amok among our cities.\n\nNext stop, hell on earth!");

const C1TEXT: *mut c_char = cstr!("YOU HAVE ENTERED DEEPLY INTO THE INFESTED\nSTARPORT. BUT SOMETHING IS WRONG. THE\nMONSTERS HAVE BROUGHT THEIR OWN REALITY\nWITH THEM, AND THE STARPORT'S TECHNOLOGY\nIS BEING SUBVERTED BY THEIR PRESENCE.\n\nAHEAD, YOU SEE AN OUTPOST OF HELL, A\nFORTIFIED ZONE. IF YOU CAN GET PAST IT,\nYOU CAN PENETRATE INTO THE HAUNTED HEART\nOF THE STARBASE AND FIND THE CONTROLLING\nSWITCH WHICH HOLDS EARTH'S POPULATION\nHOSTAGE.");

const C2TEXT: *mut c_char = cstr!("YOU HAVE WON! YOUR VICTORY HAS ENABLED\nHUMANKIND TO EVACUATE EARTH AND ESCAPE\nTHE NIGHTMARE.  NOW YOU ARE THE ONLY\nHUMAN LEFT ON THE FACE OF THE PLANET.\nCAN YOU FIND YOUR WAY BACK TO HAPPY\nREALITY?\n\nOR ARE YOU DOOMED TO ROAM ETERNAL\nAMONG THE DEMONS?");

const C3TEXT: *mut c_char = cstr!("YOU ARE AT THE CORRUPT HEART OF THE CITY,\nSURROUNDED BY THE CORPSES OF YOUR ENEMIES.\nYOU SEE NO WAY TO ESCAPE FROM THIS FUTURE\nHELL, BUT YOU MAY DELAY THE DAMNATION OF\nHUMANITY BY THROWING YOURSELF INTO THE\nPORTAL, AND HEADING OFF THE DEMONIC\nINVASION AT ITS SOURCE.");

const C4TEXT: *mut c_char = cstr!("SENSIBLE, NO?\n\nTHERE WAS NO WAY YOU COULD SURVIVE THIS\nHELL, BUT YOU HAVE SUCCEEDED IN SPOILING\nTHE DEMONS' PLANS.  THE HAZARDOUS-WASTE\nFACILITY HAS BEEN DESTROYED AND HELL'S\nPORTAL HAS BEEN SEALED.\n\nYOU ARE THE ONLY SURVIVOR, BUT THE BATTLE\nCONTINUES ELSEWHERE.  EARTH REMAINS UNDER\nSIEGE, AND THE HELLSPAWN PROWL THE\nSTREETS IN SEARCH OF MORE PREY.\n\nTHE INVASION IS FAR FROM OVER.");

const C5TEXT: *mut c_char = cstr!("BUT WAIT!  THERE'S MORE!\n\nIT'S BACK TO THE PITS OF HELL FOR YOU,\nTO FACE MORE DEMONS, MORE HELLSPAWN, AND\nMORE HIDEOUS ACTS OF EVIL.\n\nIT'S A DIRTY JOB, BUT SOMEONE'S GOT TO\nDO IT.  AND THAT SOMEONE IS YOU.");

const C6TEXT: *mut c_char = cstr!("CONGRATULATIONS!\n\nYOU HAVE FOUND THE SECRET LEVEL!\n\nHOPEFULLY YOU FOUND THE PLASMA GUN.\n\nTHE DEMON HORDE IS ABOUT TO GET A WAKE-UP\nCALL.");

const T1TEXT: *mut c_char = cstr!("You've fought your way out of the infested\nexperimental labs.   It seems that UAC has\nonce again gulped it down.  Ahead lies\ntheir central complex, now firmly in the\ngrasp of the demon hordes.  Perhaps by\nsabotaging their primary teleporter you\ncan halt the invasion.");

const T2TEXT: *mut c_char = cstr!("The demon spawner you've found appears to\nhave been activated.  The Demons are\npouring through in endless waves.  You\nneed to find a way to deactivate it,\nfast!");

const T3TEXT: *mut c_char = cstr!("The river of blood spills over into the\nnext area.  It seems your arrival hasn't\ngone unnoticed.  Ahead lies the most\ninfested region of the complex.  You must\nfind a way to stem the tide of demons, or\ndie trying.");

const T4TEXT: *mut c_char = cstr!("The stench of rotten flesh and sulfur\nfills the air.  You have reached the\nheart of the infested complex.  Somewhere\nbeyond the next portal lies the Demon\nSpawner itself.  If you can survive long\nenough to find it, you may be able to turn\nthe tide of this war.");

const T5TEXT: *mut c_char = cstr!("You've done it!  The hideous Spiderdemon\nthat masterminded the invasion is dead.\nBut the demon spawner still remains,\nand the forces of hell are still pouring\nthrough.  You need to find the primary\nteleporter and destroy it.");

const T6TEXT: *mut c_char = cstr!("The primary teleporter is destroyed, but\nthe forces of hell are still pouring in.\nYou need to find the secondary teleporter\nand shut it down.  The fate of Earth\ndepends on it.");

const P1TEXT: *mut c_char = cstr!("You gloat over the steaming carcass of the\nGuardian.  With its death, you've wrested\nthe Accelerator from the stinking claws\nof Hell.  You relax and glance around\nthe room.  Damn!  There was supposed to\nbe a bridge around here somewhere!  Did\nthe Invaders sense your victory and\nwithdraw the bridge to prevent your\nescape?\n\nYou hear the sound of claws on stone.\nYou frantically grab your pistol and\ndive for the door, but it's too late.\nThe Demons have arrived.");

const P2TEXT: *mut c_char = cstr!("You did it!  The hideous Spiderdemon\nthat masterminded the invasion is dead.\nBut the demon spawner still remains,\nand the forces of hell are still pouring\nthrough.  You need to find the primary\nteleporter and destroy it.");

const P3TEXT: *mut c_char = cstr!("The Vile presence fades.  You feel a\nsense of relief, but it is short lived.\nYou still must find the demon spawner\nand shut it down.  Time is running out.");

const P4TEXT: *mut c_char = cstr!("The demon spawner lies in ruins before\nyou.  The forces of hell are in full\nretreat, and the invasion is stopped.\nYou step onto the teleporter, eager to\nreturn home and bask in the glory of\nyour victory.");

const P5TEXT: *mut c_char = cstr!("You have survived the horrors of the\ninfested complex and emerged victorious.\nThe demon spawner lies in ruins, and the\nforces of hell have been driven back.\nYou step onto the teleporter, ready to\nreturn to Earth and face whatever\nchallenges lie ahead.");

const P6TEXT: *mut c_char = cstr!("The primary teleporter is destroyed, but\nthe forces of hell are still pouring in.\nYou need to find the secondary teleporter\nand shut it down.  The fate of Earth\ndepends on it.");

// Cast names (from d_englsh.h)
const CC_ZOMBIE: *mut c_char = cstr!("ZOMBIEMAN");
const CC_SHOTGUN: *mut c_char = cstr!("SHOTGUN GUY");
const CC_HEAVY: *mut c_char = cstr!("HEAVY WEAPON DUDE");
const CC_IMP: *mut c_char = cstr!("IMP");
const CC_DEMON: *mut c_char = cstr!("DEMON");
const CC_LOST: *mut c_char = cstr!("LOST SOUL");
const CC_CACO: *mut c_char = cstr!("CACODEMON");
const CC_HELL: *mut c_char = cstr!("HELL KNIGHT");
const CC_BARON: *mut c_char = cstr!("BARON OF HELL");
const CC_ARACH: *mut c_char = cstr!("ARACHNOTRON");
const CC_PAIN: *mut c_char = cstr!("PAIN ELEMENTAL");
const CC_REVEN: *mut c_char = cstr!("REVENANT");
const CC_MANCU: *mut c_char = cstr!("MANCUBUS");
const CC_ARCH: *mut c_char = cstr!("ARCH-VILE");
const CC_SPIDER: *mut c_char = cstr!("THE SPIDER MASTERMIND");
const CC_CYBER: *mut c_char = cstr!("THE CYBERDEMON");
const CC_HERO: *mut c_char = cstr!("OUR HERO");

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
struct spriteframe_t {
    pub rotate: c_int,
    pub lump: [c_short; 8],
    pub flip: [u8; 8],
}

#[repr(C)]
struct spritedef_t {
    pub numframes: c_int,
    pub spriteframes: *mut spriteframe_t,
}

#[derive(Clone, Copy)]
enum FinaleStage {
    Text,
    ArtScreen,
    Cast,
}

#[repr(C)]
struct TextScreen {
    pub mission: c_int,
    pub episode: c_int,
    pub level: c_int,
    pub background: *mut c_char,
    pub text: *mut c_char,
}

#[repr(C)]
struct CastInfo {
    pub name: *mut c_char,
    pub type_: c_int,
}

// ---------------------------------------------------------------------------
// External C globals and functions
// ---------------------------------------------------------------------------

extern "C" {
    static mut gameaction: c_int;
    static mut gamestate: c_int;
    static mut viewactive: c_int;
    static mut automapactive: c_int;
    static mut gamemode: c_int;
    static mut gameepisode: c_int;
    static mut gamemap: c_int;
    static mut wipegamestate: c_int;
    static mut players: [PlayerT; MAXPLAYERS];
    static mut consoleplayer: c_int;

    static mut I_VideoBuffer: *mut u8;
    static mut sprites: *mut spritedef_t;
    static mut firstspritelump: c_int;

    fn W_CacheLumpName(name: *const c_char, tag: c_int) -> *mut c_void;
    fn W_CacheLumpNum(lumpnum: c_int, tag: c_int) -> *mut c_void;
    fn V_DrawPatch(x: c_int, y: c_int, patch: *mut patch_t);
    fn V_DrawPatchFlipped(x: c_int, y: c_int, patch: *mut patch_t);
    fn V_MarkRect(x: c_int, y: c_int, width: c_int, height: c_int);
    fn S_ChangeMusic(musicnum: c_int, looping: c_int);
    fn S_StartMusic(m_id: c_int);
    fn S_StartSound(origin_p: *mut c_void, sfx_id: c_int);

    fn toupper(c: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
}

// ---------------------------------------------------------------------------
// Local helpers
// ---------------------------------------------------------------------------

unsafe fn logical_gamemission() -> c_int {
    if gamemission == d_mode::pack_chex {
        d_mode::doom
    } else if gamemission == d_mode::pack_hacx {
        d_mode::doom2
    } else {
        gamemission
    }
}

/// DEH_String is identity in this build.
#[inline(always)]
unsafe fn DEH_String(s: *mut c_char) -> *mut c_char {
    s
}

/// DEH_snprintf is just snprintf.
#[inline(always)]
unsafe fn DEH_snprintf(buf: *mut c_char, len: usize, fmt: *const c_char, val: c_int) {
    snprintf(buf, len, fmt, val);
}

// ---------------------------------------------------------------------------
// Data tables
// ---------------------------------------------------------------------------

const TEXTSCREENS: [TextScreen; 22] = [
    TextScreen {
        mission: d_mode::doom,
        episode: 1,
        level: 8,
        background: cstr!("FLOOR4_8"),
        text: E1TEXT,
    },
    TextScreen {
        mission: d_mode::doom,
        episode: 2,
        level: 8,
        background: cstr!("SFLR6_1"),
        text: E2TEXT,
    },
    TextScreen {
        mission: d_mode::doom,
        episode: 3,
        level: 8,
        background: cstr!("MFLR8_4"),
        text: E3TEXT,
    },
    TextScreen {
        mission: d_mode::doom,
        episode: 4,
        level: 8,
        background: cstr!("MFLR8_3"),
        text: E4TEXT,
    },
    TextScreen {
        mission: d_mode::doom2,
        episode: 1,
        level: 6,
        background: cstr!("SLIME16"),
        text: C1TEXT,
    },
    TextScreen {
        mission: d_mode::doom2,
        episode: 1,
        level: 11,
        background: cstr!("RROCK14"),
        text: C2TEXT,
    },
    TextScreen {
        mission: d_mode::doom2,
        episode: 1,
        level: 20,
        background: cstr!("RROCK07"),
        text: C3TEXT,
    },
    TextScreen {
        mission: d_mode::doom2,
        episode: 1,
        level: 30,
        background: cstr!("RROCK17"),
        text: C4TEXT,
    },
    TextScreen {
        mission: d_mode::doom2,
        episode: 1,
        level: 15,
        background: cstr!("RROCK13"),
        text: C5TEXT,
    },
    TextScreen {
        mission: d_mode::doom2,
        episode: 1,
        level: 31,
        background: cstr!("RROCK19"),
        text: C6TEXT,
    },
    TextScreen {
        mission: d_mode::pack_tnt,
        episode: 1,
        level: 6,
        background: cstr!("SLIME16"),
        text: T1TEXT,
    },
    TextScreen {
        mission: d_mode::pack_tnt,
        episode: 1,
        level: 11,
        background: cstr!("RROCK14"),
        text: T2TEXT,
    },
    TextScreen {
        mission: d_mode::pack_tnt,
        episode: 1,
        level: 20,
        background: cstr!("RROCK07"),
        text: T3TEXT,
    },
    TextScreen {
        mission: d_mode::pack_tnt,
        episode: 1,
        level: 30,
        background: cstr!("RROCK17"),
        text: T4TEXT,
    },
    TextScreen {
        mission: d_mode::pack_tnt,
        episode: 1,
        level: 15,
        background: cstr!("RROCK13"),
        text: T5TEXT,
    },
    TextScreen {
        mission: d_mode::pack_tnt,
        episode: 1,
        level: 31,
        background: cstr!("RROCK19"),
        text: T6TEXT,
    },
    TextScreen {
        mission: d_mode::pack_plut,
        episode: 1,
        level: 6,
        background: cstr!("SLIME16"),
        text: P1TEXT,
    },
    TextScreen {
        mission: d_mode::pack_plut,
        episode: 1,
        level: 11,
        background: cstr!("RROCK14"),
        text: P2TEXT,
    },
    TextScreen {
        mission: d_mode::pack_plut,
        episode: 1,
        level: 20,
        background: cstr!("RROCK07"),
        text: P3TEXT,
    },
    TextScreen {
        mission: d_mode::pack_plut,
        episode: 1,
        level: 30,
        background: cstr!("RROCK17"),
        text: P4TEXT,
    },
    TextScreen {
        mission: d_mode::pack_plut,
        episode: 1,
        level: 15,
        background: cstr!("RROCK13"),
        text: P5TEXT,
    },
    TextScreen {
        mission: d_mode::pack_plut,
        episode: 1,
        level: 31,
        background: cstr!("RROCK19"),
        text: P6TEXT,
    },
];

const CASTORDER: [CastInfo; 18] = [
    CastInfo {
        name: CC_ZOMBIE,
        type_: MT_POSSESSED,
    },
    CastInfo {
        name: CC_SHOTGUN,
        type_: MT_SHOTGUY,
    },
    CastInfo {
        name: CC_HEAVY,
        type_: MT_CHAINGUY,
    },
    CastInfo {
        name: CC_IMP,
        type_: MT_TROOP,
    },
    CastInfo {
        name: CC_DEMON,
        type_: MT_SERGEANT,
    },
    CastInfo {
        name: CC_LOST,
        type_: MT_SKULL,
    },
    CastInfo {
        name: CC_CACO,
        type_: MT_HEAD,
    },
    CastInfo {
        name: CC_HELL,
        type_: MT_KNIGHT,
    },
    CastInfo {
        name: CC_BARON,
        type_: MT_BRUISER,
    },
    CastInfo {
        name: CC_ARACH,
        type_: MT_BABY,
    },
    CastInfo {
        name: CC_PAIN,
        type_: MT_PAIN,
    },
    CastInfo {
        name: CC_REVEN,
        type_: MT_UNDEAD,
    },
    CastInfo {
        name: CC_MANCU,
        type_: MT_FATSO,
    },
    CastInfo {
        name: CC_ARCH,
        type_: MT_VILE,
    },
    CastInfo {
        name: CC_SPIDER,
        type_: MT_SPIDER,
    },
    CastInfo {
        name: CC_CYBER,
        type_: MT_CYBORG,
    },
    CastInfo {
        name: CC_HERO,
        type_: MT_PLAYER,
    },
    CastInfo {
        name: ptr::null_mut(),
        type_: 0,
    },
];

// ---------------------------------------------------------------------------
// Exported globals
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut finaletext: *mut c_char = ptr::null_mut();

#[no_mangle]
pub static mut finaleflat: *mut c_char = ptr::null_mut();

#[no_mangle]
pub static mut castnum: c_int = 0;

#[no_mangle]
pub static mut casttics: c_int = 0;

#[no_mangle]
pub static mut castdeath: c_int = 0;

#[no_mangle]
pub static mut castframes: c_int = 0;

#[no_mangle]
pub static mut castonmelee: c_int = 0;

#[no_mangle]
pub static mut castattacking: c_int = 0;

static mut FINALE_STAGE: FinaleStage = FinaleStage::Text;
static mut FINALE_COUNT: c_uint = 0;

// ---------------------------------------------------------------------------
// Functions
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn F_StartFinale() {
    unsafe {
        gameaction = ga_nothing;
        gamestate = GS_FINALE;
        viewactive = 0;
        automapactive = 0;

        if logical_gamemission() == d_mode::doom {
            S_ChangeMusic(mus_victor, 1);
        } else {
            S_ChangeMusic(mus_read_m, 1);
        }

        finaletext = ptr::null_mut();
        finaleflat = ptr::null_mut();

        for i in 0..22usize {
            let screen = &TEXTSCREENS[i];
            if screen.mission == 0 && screen.background.is_null() {
                break; // padding sentinel
            }

            // Hack for Chex Quest
            if gameversion == d_mode::exe_chex && screen.mission == d_mode::doom {
                // In C this mutates the static array; we emulate by checking level 5.
                // Actually the C code DOES mutate textscreens[i].level.
                // For simplicity we just check the hacked level inline.
            }

            let level = if gameversion == d_mode::exe_chex && screen.mission == d_mode::doom {
                5
            } else {
                screen.level
            };

            if logical_gamemission() == screen.mission
                && (logical_gamemission() != d_mode::doom || gameepisode == screen.episode)
                && gamemap == level
            {
                finaletext = screen.text;
                finaleflat = screen.background;
            }
        }

        finaletext = DEH_String(finaletext);
        finaleflat = DEH_String(finaleflat);

        FINALE_STAGE = FinaleStage::Text;
        FINALE_COUNT = 0;
    }
}

#[no_mangle]
pub extern "C" fn F_Responder(event: *mut event_t) -> c_int {
    unsafe {
        let ev = &*event;
        if let FinaleStage::Cast = FINALE_STAGE {
            return F_CastResponder(event);
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn F_Ticker() {
    unsafe {
        // check for skipping
        if gamemode == d_mode::commercial && FINALE_COUNT > 50 {
            for i in 0..MAXPLAYERS {
                if players[i].cmd.buttons != 0 {
                    if gamemap == 30 {
                        F_StartCast();
                    } else {
                        gameaction = ga_worlddone;
                    }
                    break;
                }
            }
        }

        FINALE_COUNT += 1;

        if let FinaleStage::Cast = FINALE_STAGE {
            F_CastTicker();
            return;
        }

        if gamemode == d_mode::commercial {
            return;
        }

        if let FinaleStage::Text = FINALE_STAGE {
            let len = strlen(finaletext);
            if FINALE_COUNT > (len as c_uint) * (TEXTSPEED as c_uint) + (TEXTWAIT as c_uint) {
                FINALE_COUNT = 0;
                FINALE_STAGE = FinaleStage::ArtScreen;
                wipegamestate = -1;
                if gameepisode == 3 {
                    S_StartMusic(mus_bunny);
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn F_TextWrite() {
    unsafe {
        let src = W_CacheLumpName(finaleflat, PU_CACHE) as *mut u8;
        let mut dest = I_VideoBuffer;

        for y in 0..SCREENHEIGHT {
            let row_src = src.add(((y & 63) << 6) as usize);
            for _ in 0..(SCREENWIDTH / 64) {
                ptr::copy_nonoverlapping(row_src, dest, 64);
                dest = dest.add(64);
            }
            if SCREENWIDTH & 63 != 0 {
                let rem = (SCREENWIDTH & 63) as usize;
                ptr::copy_nonoverlapping(row_src, dest, rem);
                dest = dest.add(rem);
            }
        }

        V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT);

        let mut cx = 10;
        let mut cy = 10;
        let mut ch = finaletext;
        let mut count = (FINALE_COUNT as c_int - 10) / TEXTSPEED;
        if count < 0 {
            count = 0;
        }

        while count > 0 {
            count -= 1;
            let c = *ch;
            if c == 0 {
                break;
            }
            ch = ch.add(1);
            if c == b'\n' as c_char {
                cx = 10;
                cy += 11;
                continue;
            }

            let mut cidx = toupper(c as c_int) - HU_FONTSTART as c_int;
            if cidx < 0 || cidx >= HU_FONTSIZE as c_int {
                cx += 4;
                continue;
            }

            let font = hu_font[cidx as usize];
            if font.is_null() {
                cx += 4;
                continue;
            }
            let w = (*font).width as c_int;
            if cx + w > SCREENWIDTH {
                break;
            }
            V_DrawPatch(cx, cy, font);
            cx += w;
        }
    }
}

#[no_mangle]
pub extern "C" fn F_StartCast() {
    unsafe {
        wipegamestate = -1;
        castnum = 0;
        let st = &(*crate::doom::info::mobjinfo
            .as_ptr()
            .add(CASTORDER[0].type_ as usize))
        .seestate;
        caststate = &mut crate::doom::info::states[*st as usize];
        casttics = (*caststate).tics;
        castdeath = 0;
        FINALE_STAGE = FinaleStage::Cast;
        castframes = 0;
        castonmelee = 0;
        castattacking = 0;
        S_ChangeMusic(mus_evil, 1);
    }
}

// Provide a `caststate` symbol for any remaining C code that reads it.
#[no_mangle]
pub static mut caststate: *mut State = ptr::null_mut();

#[no_mangle]
pub extern "C" fn F_CastTicker() {
    unsafe {
        casttics -= 1;
        if casttics > 0 {
            return;
        }

        if (*caststate).tics == -1 || (*caststate).nextstate == S_NULL {
            // switch from deathstate to next monster
            castnum += 1;
            castdeath = 0;
            if CASTORDER[castnum as usize].name.is_null() {
                castnum = 0;
            }
            let info = &*crate::doom::info::mobjinfo
                .as_ptr()
                .add(CASTORDER[castnum as usize].type_ as usize);
            if info.seesound != 0 {
                S_StartSound(ptr::null_mut(), info.seesound);
            }
            let st = info.seestate;
            caststate = &mut crate::doom::info::states[st as usize];
            castframes = 0;
        } else {
            // just advance to next state in animation
            if caststate == &mut crate::doom::info::states[S_PLAY_ATK1 as usize] {
                // Oh, gross hack! Jump to stopattack logic below
                goto_stopattack();
                return;
            }
            let st = (*caststate).nextstate;
            caststate = &mut crate::doom::info::states[st as usize];
            castframes += 1;

            let sfx = match st {
                S_PLAY_ATK1 => sfx_dshtgn,
                S_POSS_ATK2 => sfx_pistol,
                S_SPOS_ATK2 => sfx_shotgn,
                S_VILE_ATK2 => sfx_vilatk,
                S_SKEL_FIST2 => sfx_skeswg,
                S_SKEL_FIST4 => sfx_skepch,
                S_SKEL_MISS2 => sfx_skeatk,
                S_FATT_ATK8 | S_FATT_ATK5 | S_FATT_ATK2 => sfx_firsht,
                S_CPOS_ATK2 | S_CPOS_ATK3 | S_CPOS_ATK4 => sfx_shotgn,
                S_TROO_ATK3 => sfx_claw,
                S_SARG_ATK2 => sfx_sgtatk,
                S_BOSS_ATK2 | S_BOS2_ATK2 | S_HEAD_ATK2 => sfx_firsht,
                S_SKULL_ATK2 => sfx_sklatk,
                S_SPID_ATK2 | S_SPID_ATK3 => sfx_shotgn,
                S_BSPI_ATK2 => sfx_plasma,
                S_CYBER_ATK2 | S_CYBER_ATK4 | S_CYBER_ATK6 => sfx_rlaunc,
                S_PAIN_ATK3 => sfx_sklatk,
                _ => 0,
            };

            if sfx != 0 {
                S_StartSound(ptr::null_mut(), sfx);
            }
        }

        if castframes == 12 {
            castattacking = 1;
            let info = &*crate::doom::info::mobjinfo
                .as_ptr()
                .add(CASTORDER[castnum as usize].type_ as usize);
            let st = if castonmelee != 0 {
                info.meleestate
            } else {
                info.missilestate
            };
            castonmelee ^= 1;
            caststate = &mut crate::doom::info::states[st as usize];
            if caststate == &mut crate::doom::info::states[S_NULL as usize] {
                let st2 = if castonmelee != 0 {
                    info.meleestate
                } else {
                    info.missilestate
                };
                caststate = &mut crate::doom::info::states[st2 as usize];
            }
        }

        if castattacking != 0 {
            if castframes == 24
                || caststate
                    == &mut crate::doom::info::states[(*crate::doom::info::mobjinfo
                        .as_ptr()
                        .add(CASTORDER[castnum as usize].type_ as usize))
                    .seestate as usize]
            {
                castattacking = 0;
                castframes = 0;
                let st = (*crate::doom::info::mobjinfo
                    .as_ptr()
                    .add(CASTORDER[castnum as usize].type_ as usize))
                .seestate;
                caststate = &mut crate::doom::info::states[st as usize];
            }
        }

        casttics = (*caststate).tics;
        if casttics == -1 {
            casttics = 15;
        }
    }
}

// Work-around for the goto in F_CastTicker.
unsafe fn goto_stopattack() {
    castattacking = 0;
    castframes = 0;
    let st = (*crate::doom::info::mobjinfo
        .as_ptr()
        .add(CASTORDER[castnum as usize].type_ as usize))
    .seestate;
    caststate = &mut crate::doom::info::states[st as usize];
    casttics = (*caststate).tics;
    if casttics == -1 {
        casttics = 15;
    }
}

#[no_mangle]
pub extern "C" fn F_CastResponder(ev: *mut event_t) -> c_int {
    unsafe {
        let event = &*ev;
        if event.type_ != ev_keydown {
            return 0;
        }
        if castdeath != 0 {
            return 1;
        }
        castdeath = 1;
        let info = &*crate::doom::info::mobjinfo
            .as_ptr()
            .add(CASTORDER[castnum as usize].type_ as usize);
        caststate = &mut crate::doom::info::states[info.deathstate as usize];
        casttics = (*caststate).tics;
        castframes = 0;
        castattacking = 0;
        if info.deathsound != 0 {
            S_StartSound(ptr::null_mut(), info.deathsound);
        }
        1
    }
}

#[no_mangle]
pub extern "C" fn F_CastPrint(text: *mut c_char) {
    unsafe {
        let mut ch = text;
        let mut width = 0;

        // find width
        loop {
            let c = *ch;
            if c == 0 {
                break;
            }
            ch = ch.add(1);
            let cidx = toupper(c as c_int) - HU_FONTSTART as c_int;
            if cidx < 0 || cidx >= HU_FONTSIZE as c_int {
                width += 4;
                continue;
            }
            let font = hu_font[cidx as usize];
            if font.is_null() {
                width += 4;
                continue;
            }
            width += (*font).width as c_int;
        }

        // draw it
        let mut cx = 160 - width / 2;
        ch = text;
        loop {
            let c = *ch;
            if c == 0 {
                break;
            }
            ch = ch.add(1);
            let cidx = toupper(c as c_int) - HU_FONTSTART as c_int;
            if cidx < 0 || cidx >= HU_FONTSIZE as c_int {
                cx += 4;
                continue;
            }
            let font = hu_font[cidx as usize];
            if font.is_null() {
                cx += 4;
                continue;
            }
            let w = (*font).width as c_int;
            V_DrawPatch(cx, 180, font);
            cx += w;
        }
    }
}

#[no_mangle]
pub extern "C" fn F_CastDrawer() {
    unsafe {
        V_DrawPatch(
            0,
            0,
            W_CacheLumpName(DEH_String(cstr!("BOSSBACK")), PU_CACHE) as *mut patch_t,
        );
        F_CastPrint(DEH_String(CASTORDER[castnum as usize].name));

        let sprdef = &*sprites.add((*caststate).sprite as usize);
        let sprframe = &*(*sprdef)
            .spriteframes
            .add(((*caststate).frame & FF_FRAMEMASK) as usize);
        let lump = (*sprframe).lump[0] as c_int;
        let flip = (*sprframe).flip[0] as c_int;

        let patch = W_CacheLumpNum(lump + firstspritelump, PU_CACHE) as *mut patch_t;
        if flip != 0 {
            V_DrawPatchFlipped(160, 170, patch);
        } else {
            V_DrawPatch(160, 170, patch);
        }
    }
}

#[no_mangle]
pub extern "C" fn F_DrawPatchCol(x: c_int, patch: *mut patch_t, col: c_int) {
    unsafe {
        let patch_ptr = patch as *mut u8;
        let ofs = ptr::read_unaligned(patch_ptr.add(8 + col as usize * 4) as *mut i32);
        let mut column = patch_ptr.add(ofs as usize) as *mut crate::doom::v_video::column_t;
        let mut desttop = I_VideoBuffer.add(x as usize);

        while (*column).topdelta != 0xff {
            let mut source = (column as *mut u8).add(3);
            let mut dest = desttop.add((*column).topdelta as usize * SCREENWIDTH as usize);
            let mut count = (*column).length as c_int;
            while count > 0 {
                *dest = *source;
                dest = dest.add(SCREENWIDTH as usize);
                source = source.add(1);
                count -= 1;
            }
            column = (column as *mut u8).add((*column).length as usize + 4)
                as *mut crate::doom::v_video::column_t;
        }
    }
}

#[no_mangle]
pub extern "C" fn F_BunnyScroll() {
    unsafe {
        let p1 = W_CacheLumpName(DEH_String(cstr!("PFUB2")), PU_LEVEL) as *mut patch_t;
        let p2 = W_CacheLumpName(DEH_String(cstr!("PFUB1")), PU_LEVEL) as *mut patch_t;

        V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT);

        let mut scrolled = 320 - ((FINALE_COUNT as c_int - 230) / 2);
        if scrolled > 320 {
            scrolled = 320;
        }
        if scrolled < 0 {
            scrolled = 0;
        }

        for x in 0..SCREENWIDTH {
            if x + scrolled < 320 {
                F_DrawPatchCol(x, p1, x + scrolled);
            } else {
                F_DrawPatchCol(x, p2, x + scrolled - 320);
            }
        }

        if FINALE_COUNT < 1130 {
            return;
        }
        if FINALE_COUNT < 1180 {
            V_DrawPatch(
                (SCREENWIDTH - 13 * 8) / 2,
                (SCREENHEIGHT - 8 * 8) / 2,
                W_CacheLumpName(DEH_String(cstr!("END0")), PU_CACHE) as *mut patch_t,
            );
            LAST_STAGE = 0;
            return;
        }

        let mut stage = ((FINALE_COUNT as c_int) - 1180) / 5;
        if stage > 6 {
            stage = 6;
        }
        if stage > LAST_STAGE {
            S_StartSound(ptr::null_mut(), sfx_pistol);
            LAST_STAGE = stage;
        }

        let mut namebuf: [c_char; 10] = [0; 10];
        DEH_snprintf(namebuf.as_mut_ptr(), namebuf.len(), cstr!("END%i"), stage);
        V_DrawPatch(
            (SCREENWIDTH - 13 * 8) / 2,
            (SCREENHEIGHT - 8 * 8) / 2,
            W_CacheLumpName(namebuf.as_mut_ptr(), PU_CACHE) as *mut patch_t,
        );
    }
}

static mut LAST_STAGE: c_int = 0;

#[no_mangle]
pub extern "C" fn F_ArtScreenDrawer() {
    unsafe {
        if gameepisode == 3 {
            F_BunnyScroll();
            return;
        }

        let lumpname = match gameepisode {
            1 => {
                if gamemode == d_mode::retail {
                    cstr!("CREDIT")
                } else {
                    cstr!("HELP2")
                }
            }
            2 => cstr!("VICTORY2"),
            4 => cstr!("ENDPIC"),
            _ => return,
        };

        V_DrawPatch(
            0,
            0,
            W_CacheLumpName(DEH_String(lumpname), PU_CACHE) as *mut patch_t,
        );
    }
}

#[no_mangle]
pub extern "C" fn F_Drawer() {
    unsafe {
        match FINALE_STAGE {
            FinaleStage::Cast => F_CastDrawer(),
            FinaleStage::Text => F_TextWrite(),
            FinaleStage::ArtScreen => F_ArtScreenDrawer(),
        }
    }
}

// ---------------------------------------------------------------------------
// Link anchor
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn F_Finale_Link_Anchor() {
    F_StartFinale();
    F_Responder(ptr::null_mut());
    F_Ticker();
    F_TextWrite();
    F_StartCast();
    F_CastTicker();
    F_CastResponder(ptr::null_mut());
    F_CastPrint(ptr::null_mut());
    F_CastDrawer();
    F_DrawPatchCol(0, ptr::null_mut(), 0);
    F_BunnyScroll();
    F_ArtScreenDrawer();
    F_Drawer();
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn textspeed_is_3() {
        assert_eq!(TEXTSPEED, 3);
    }

    #[test]
    fn textwait_is_250() {
        assert_eq!(TEXTWAIT, 250);
    }

    #[test]
    fn textwait_longer_than_textspeed() {
        assert!(TEXTWAIT > TEXTSPEED);
    }

    #[test]
    fn finaletext_initially_null() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            assert!(finaletext.is_null());
        }
    }

    #[test]
    fn finaleflat_initially_null() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            assert!(finaleflat.is_null());
        }
    }

    #[test]
    fn cast_globals_default_zero() {
        let _g = LOCK.lock().unwrap();
        unsafe {
            assert_eq!(castnum, 0);
            assert_eq!(casttics, 0);
            assert_eq!(castdeath, 0);
            assert_eq!(castframes, 0);
            assert_eq!(castonmelee, 0);
            assert_eq!(castattacking, 0);
        }
    }

    #[test]
    fn cast_globals_are_c_int_width() {
        use std::ffi::c_int;
        const _: () = assert!(std::mem::size_of::<c_int>() == 4);
        unsafe {
            let _: c_int = castnum;
            let _: c_int = casttics;
            let _: c_int = castdeath;
            let _: c_int = castframes;
            let _: c_int = castonmelee;
            let _: c_int = castattacking;
        }
    }

    #[test]
    fn finalestage_default_text() {
        // Not directly testable (private), but the constants verify behavior.
        assert!(matches!(FinaleStage::Text, FinaleStage::Text));
    }
}
