//! Rust port of vendor/doomgeneric/hu_stuff.c.
//!
//! Heads-up display: map title, player messages, and chat input.

#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use crate::doom::sounds::Sfx;
use std::ffi::{c_char, c_int, c_void};
use std::ptr;

use crate::doom::d_event::event_t;
use crate::doom::d_mode;
use crate::doom::d_player::{PlayerT, MAXPLAYERS};
use crate::doom::doomkeys::{KEY_ENTER, KEY_ESCAPE, KEY_LALT, KEY_RALT, KEY_RSHIFT};
use crate::doom::doomstat::{gamemission, gameversion};
use crate::doom::hu_lib::{
    hu_itext_t, hu_stext_t, hu_textline_t, HUlib_addCharToTextLine, HUlib_addMessageToSText,
    HUlib_drawIText, HUlib_drawSText, HUlib_drawTextLine, HUlib_eraseIText, HUlib_eraseSText,
    HUlib_eraseTextLine, HUlib_initIText, HUlib_initSText, HUlib_initTextLine, HUlib_keyInIText,
    HUlib_resetIText,
};
use crate::doom::i_timer::TICRATE;
use crate::doom::m_controls::{key_message_refresh, key_multi_msg, key_multi_msgplayer};
use crate::doom::m_misc::M_StringCopy;
use crate::doom::v_video::patch_t;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

pub const HU_FONTSTART: u8 = b'!';
pub const HU_FONTEND: u8 = b'_';
pub const HU_FONTSIZE: usize = (HU_FONTEND - HU_FONTSTART + 1) as usize;
pub const HU_BROADCAST: c_int = 5;
pub const HU_MSGX: c_int = 0;
pub const HU_MSGY: c_int = 0;
pub const HU_MSGWIDTH: c_int = 64;
pub const HU_MSGHEIGHT: c_int = 1;
pub const HU_MSGTIMEOUT: c_int = 4 * TICRATE;

use crate::doom::z_zone::PU_STATIC;
const QUEUESIZE: usize = 128;

/// Identity SHORT macro for little-endian.
#[inline(always)]
fn short_swap(v: i16) -> i16 {
    v
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
// String literals
// ---------------------------------------------------------------------------

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *mut c_char
    };
}

// ---------------------------------------------------------------------------
// Exported globals
// ---------------------------------------------------------------------------

#[no_mangle]
pub static mut chat_macros: [*mut c_char; 10] = [
    cstr!("No"),
    cstr!("I'm ready to kick butt!"),
    cstr!("I'm OK."),
    cstr!("I'm not looking too good!"),
    cstr!("Help!"),
    cstr!("You suck!"),
    cstr!("Next time, scumbag..."),
    cstr!("Come here!"),
    cstr!("I'll take care of it."),
    cstr!("Yes"),
];

#[no_mangle]
pub static mut player_names: [*mut c_char; 4] = [
    cstr!("Green: "),
    cstr!("Indigo: "),
    cstr!("Brown: "),
    cstr!("Red: "),
];

#[no_mangle]
pub static mut chat_char: c_char = 0;

#[no_mangle]
pub static mut hu_font: [*mut patch_t; HU_FONTSIZE] = [ptr::null_mut(); HU_FONTSIZE];

#[no_mangle]
pub static mut chat_on: c_int = 0;

#[no_mangle]
pub static mut message_dontfuckwithme: c_int = 0;

#[no_mangle]
pub static mut mapnames: [*mut c_char; 45] = [
    cstr!("E1M1: Hangar"),
    cstr!("E1M2: Nuclear Plant"),
    cstr!("E1M3: Toxin Refinery"),
    cstr!("E1M4: Command Control"),
    cstr!("E1M5: Phobos Lab"),
    cstr!("E1M6: Central Processing"),
    cstr!("E1M7: Computer Station"),
    cstr!("E1M8: Phobos Anomaly"),
    cstr!("E1M9: Military Base"),
    cstr!("E2M1: Deimos Anomaly"),
    cstr!("E2M2: Containment Area"),
    cstr!("E2M3: Refinery"),
    cstr!("E2M4: Deimos Lab"),
    cstr!("E2M5: Command Center"),
    cstr!("E2M6: Halls of the Damned"),
    cstr!("E2M7: Spawning Vats"),
    cstr!("E2M8: Tower of Babel"),
    cstr!("E2M9: Fortress of Mystery"),
    cstr!("E3M1: Hell Keep"),
    cstr!("E3M2: Slough of Despair"),
    cstr!("E3M3: Pandemonium"),
    cstr!("E3M4: House of Pain"),
    cstr!("E3M5: Unholy Cathedral"),
    cstr!("E3M6: Mt. Erebus"),
    cstr!("E3M7: Limbo"),
    cstr!("E3M8: Dis"),
    cstr!("E3M9: Warrens"),
    cstr!("E4M1: Hell Beneath"),
    cstr!("E4M2: Perfect Hatred"),
    cstr!("E4M3: Sever The Wicked"),
    cstr!("E4M4: Unruly Evil"),
    cstr!("E4M5: They Will Repent"),
    cstr!("E4M6: Against Thee Wickedly"),
    cstr!("E4M7: And Hell Followed"),
    cstr!("E4M8: Unto The Cruel"),
    cstr!("E4M9: Fear"),
    cstr!("NEWLEVEL"),
    cstr!("NEWLEVEL"),
    cstr!("NEWLEVEL"),
    cstr!("NEWLEVEL"),
    cstr!("NEWLEVEL"),
    cstr!("NEWLEVEL"),
    cstr!("NEWLEVEL"),
    cstr!("NEWLEVEL"),
    cstr!("NEWLEVEL"),
];

#[no_mangle]
pub static mut mapnames_commercial: [*mut c_char; 96] = [
    // DOOM 2
    cstr!("level 1: entryway"),
    cstr!("level 2: underhalls"),
    cstr!("level 3: the gantlet"),
    cstr!("level 4: the focus"),
    cstr!("level 5: the waste tunnels"),
    cstr!("level 6: the crusher"),
    cstr!("level 7: dead simple"),
    cstr!("level 8: tricks and traps"),
    cstr!("level 9: the pit"),
    cstr!("level 10: refueling base"),
    cstr!("level 11: 'o' of destruction!"),
    cstr!("level 12: the factory"),
    cstr!("level 13: downtown"),
    cstr!("level 14: the inmost dens"),
    cstr!("level 15: industrial zone"),
    cstr!("level 16: suburbs"),
    cstr!("level 17: tenements"),
    cstr!("level 18: the courtyard"),
    cstr!("level 19: the citadel"),
    cstr!("level 20: gotcha!"),
    cstr!("level 21: nirvana"),
    cstr!("level 22: the catacombs"),
    cstr!("level 23: barrels o' fun"),
    cstr!("level 24: the chasm"),
    cstr!("level 25: bloodfalls"),
    cstr!("level 26: the abandoned mines"),
    cstr!("level 27: monster condo"),
    cstr!("level 28: the spirit world"),
    cstr!("level 29: the living end"),
    cstr!("level 30: icon of sin"),
    cstr!("level 31: wolfenstein"),
    cstr!("level 32: grosse"),
    // Plutonia
    cstr!("level 1: congo"),
    cstr!("level 2: well of souls"),
    cstr!("level 3: aztec"),
    cstr!("level 4: caged"),
    cstr!("level 5: ghost town"),
    cstr!("level 6: baron's lair"),
    cstr!("level 7: caughtyard"),
    cstr!("level 8: realm"),
    cstr!("level 9: abattoire"),
    cstr!("level 10: onslaught"),
    cstr!("level 11: hunted"),
    cstr!("level 12: speed"),
    cstr!("level 13: the crypt"),
    cstr!("level 14: genesis"),
    cstr!("level 15: the twilight"),
    cstr!("level 16: the omen"),
    cstr!("level 17: compound"),
    cstr!("level 18: neurosphere"),
    cstr!("level 19: nme"),
    cstr!("level 20: the death domain"),
    cstr!("level 21: slayer"),
    cstr!("level 22: impossible mission"),
    cstr!("level 23: tombstone"),
    cstr!("level 24: the final frontier"),
    cstr!("level 25: the temple of darkness"),
    cstr!("level 26: bunker"),
    cstr!("level 27: anti-christ"),
    cstr!("level 28: the sewers"),
    cstr!("level 29: odyssey of noises"),
    cstr!("level 30: the gateway of hell"),
    cstr!("level 31: cyberden"),
    cstr!("level 32: go 2 it"),
    // TNT
    cstr!("level 1: system control"),
    cstr!("level 2: human bbq"),
    cstr!("level 3: power control"),
    cstr!("level 4: wormhole"),
    cstr!("level 5: hanger"),
    cstr!("level 6: open season"),
    cstr!("level 7: prison"),
    cstr!("level 8: metal"),
    cstr!("level 9: stronghold"),
    cstr!("level 10: redemption"),
    cstr!("level 11: storage facility"),
    cstr!("level 12: crater"),
    cstr!("level 13: nukage processing"),
    cstr!("level 14: steel works"),
    cstr!("level 15: dead zone"),
    cstr!("level 16: deepest reaches"),
    cstr!("level 17: processing area"),
    cstr!("level 18: mill"),
    cstr!("level 19: shipping/respawning"),
    cstr!("level 20: central processing"),
    cstr!("level 21: administration center"),
    cstr!("level 22: habitat"),
    cstr!("level 23: lunar mining project"),
    cstr!("level 24: quarry"),
    cstr!("level 25: baron's den"),
    cstr!("level 26: ballistyx"),
    cstr!("level 27: mount pain"),
    cstr!("level 28: heck"),
    cstr!("level 29: river styx"),
    cstr!("level 30: last call"),
    cstr!("level 31: pharaoh"),
    cstr!("level 32: caribbean"),
];

// ---------------------------------------------------------------------------
// Local statics
// ---------------------------------------------------------------------------

static mut plr: *mut PlayerT = ptr::null_mut();
static mut w_title: hu_textline_t = unsafe { std::mem::zeroed() };
static mut w_chat: hu_itext_t = unsafe { std::mem::zeroed() };
static mut always_off: c_int = 0;
static mut chat_dest: [c_char; MAXPLAYERS] = [0; MAXPLAYERS];
static mut w_inputbuffer: [hu_itext_t; MAXPLAYERS] = unsafe { std::mem::zeroed() };
static mut message_on: c_int = 0;
static mut message_nottobefuckedwith: c_int = 0;
static mut w_message: hu_stext_t = unsafe { std::mem::zeroed() };
static mut message_counter: c_int = 0;
static mut headsupactive: bool = false;

static mut chatchars: [c_char; QUEUESIZE] = [0; QUEUESIZE];
static mut head: usize = 0;
static mut tail: usize = 0;

extern "C" {
    fn W_CacheLumpName(name: *const c_char, tag: c_int) -> *mut c_void;
    fn S_StartSound(origin_p: *mut c_void, sfx_id: c_int);
    static mut automapactive: c_int;
    static mut netgame: c_int;
    static mut playeringame: [c_int; MAXPLAYERS];
    static mut consoleplayer: c_int;
    static mut showMessages: c_int;
    static mut gamemode: c_int;
    static mut gameepisode: c_int;
    static mut gamemap: c_int;
}

// ---------------------------------------------------------------------------
// Functions
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn HU_Init() {
    unsafe {
        let mut j = HU_FONTSTART as c_int;
        for i in 0..HU_FONTSIZE {
            let name = format!("STCFN{:03}\0", j);
            hu_font[i] = W_CacheLumpName(name.as_ptr() as *const c_char, PU_STATIC) as *mut patch_t;
            j += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn HU_Stop() {
    unsafe {
        headsupactive = false;
    }
}

#[no_mangle]
pub extern "C" fn HU_Start() {
    unsafe {
        if headsupactive {
            HU_Stop();
        }

        plr = std::ptr::addr_of_mut!(crate::doom::d_player::players[0])
            .offset(consoleplayer as isize);
        message_on = 0;
        message_dontfuckwithme = 0;
        message_nottobefuckedwith = 0;
        chat_on = 0;

        // create the message widget
        HUlib_initSText(
            &raw mut w_message,
            HU_MSGX,
            HU_MSGY,
            HU_MSGHEIGHT,
            std::ptr::addr_of_mut!(hu_font[0]),
            HU_FONTSTART as c_int,
            &raw mut message_on,
        );

        // compute font height for title/input placement
        let font_h = if !hu_font[0].is_null() {
            short_swap((*hu_font[0]).height) as c_int
        } else {
            0
        };
        let title_y = 167 - font_h;

        // create the map title widget
        HUlib_initTextLine(
            &raw mut w_title,
            0,
            title_y,
            std::ptr::addr_of_mut!(hu_font[0]),
            HU_FONTSTART as c_int,
        );

        let mut s: *mut c_char = match logical_gamemission() {
            d_mode::doom => mapnames[((gameepisode - 1) * 9 + gamemap - 1) as usize],
            d_mode::doom2 => mapnames_commercial[(gamemap - 1) as usize],
            d_mode::pack_plut => mapnames_commercial[(gamemap - 1 + 32) as usize],
            d_mode::pack_tnt => mapnames_commercial[(gamemap - 1 + 64) as usize],
            _ => cstr!("Unknown level"),
        };

        if gameversion == d_mode::exe_chex {
            s = mapnames[(gamemap - 1) as usize];
        }

        // dehacked substitution is identity in this build
        while *s != 0 {
            HUlib_addCharToTextLine(&raw mut w_title, *s);
            s = s.add(1);
        }

        // create the chat widget
        let input_y = HU_MSGY + HU_MSGHEIGHT * (font_h + 1);
        HUlib_initIText(
            &raw mut w_chat,
            HU_MSGX,
            input_y,
            std::ptr::addr_of_mut!(hu_font[0]),
            HU_FONTSTART as c_int,
            &raw mut chat_on,
        );

        // create the inputbuffer widgets
        for i in 0..MAXPLAYERS {
            HUlib_initIText(
                &mut w_inputbuffer[i],
                0,
                0,
                ptr::null_mut(),
                0,
                &raw mut always_off,
            );
        }

        headsupactive = true;
    }
}

#[no_mangle]
pub extern "C" fn HU_Drawer() {
    unsafe {
        HUlib_drawSText(&raw mut w_message);
        HUlib_drawIText(&raw mut w_chat);
        if automapactive != 0 {
            HUlib_drawTextLine(&raw mut w_title, 0);
        }
    }
}

#[no_mangle]
pub extern "C" fn HU_Erase() {
    HUlib_eraseSText(&raw mut w_message);
    HUlib_eraseIText(&raw mut w_chat);
    HUlib_eraseTextLine(&raw mut w_title);
}

#[no_mangle]
pub extern "C" fn HU_Ticker() {
    unsafe {
        // tick down message counter if message is up
        if message_counter != 0 {
            message_counter -= 1;
            if message_counter == 0 {
                message_on = 0;
                message_nottobefuckedwith = 0;
            }
        }

        if showMessages != 0 || message_dontfuckwithme != 0 {
            // display message if necessary
            if (!(*plr).message.is_null() && message_nottobefuckedwith == 0)
                || (!(*plr).message.is_null() && message_dontfuckwithme != 0)
            {
                HUlib_addMessageToSText(&raw mut w_message, ptr::null_mut(), (*plr).message);
                (*plr).message = ptr::null_mut();
                message_on = 1;
                message_counter = HU_MSGTIMEOUT;
                message_nottobefuckedwith = message_dontfuckwithme;
                message_dontfuckwithme = 0;
            }
        }

        // check for incoming chat characters
        if netgame != 0 {
            for i in 0..MAXPLAYERS {
                if playeringame[i] == 0 {
                    continue;
                }
                if i != consoleplayer as usize {
                    let c = (*plr.add(i)).cmd.chatchar;
                    if c != 0 {
                        if c <= HU_BROADCAST as u8 {
                            chat_dest[i] = c as c_char;
                        } else {
                            let rc = HUlib_keyInIText(&mut w_inputbuffer[i], c);
                            if rc != 0 && c == KEY_ENTER {
                                if w_inputbuffer[i].l.len != 0
                                    && (chat_dest[i] == (consoleplayer + 1) as c_char
                                        || chat_dest[i] == HU_BROADCAST as c_char)
                                {
                                    HUlib_addMessageToSText(
                                        &raw mut w_message,
                                        player_names[i],
                                        w_inputbuffer[i].l.l.as_mut_ptr(),
                                    );
                                    message_nottobefuckedwith = 1;
                                    message_on = 1;
                                    message_counter = HU_MSGTIMEOUT;
                                    if gamemode == d_mode::commercial {
                                        S_StartSound(ptr::null_mut(), Sfx::Radio as c_int);
                                    } else {
                                        S_StartSound(ptr::null_mut(), Sfx::Tink as c_int);
                                    }
                                }
                                HUlib_resetIText(&mut w_inputbuffer[i]);
                            }
                        }
                        (*plr.add(i)).cmd.chatchar = 0;
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn HU_queueChatChar(c: c_char) {
    unsafe {
        if ((head + 1) & (QUEUESIZE - 1)) == tail {
            (*plr).message = cstr!("[Message unsent]");
        } else {
            chatchars[head] = c;
            head = (head + 1) & (QUEUESIZE - 1);
        }
    }
}

#[no_mangle]
pub extern "C" fn HU_dequeueChatChar() -> c_char {
    unsafe {
        if head != tail {
            let c = chatchars[tail];
            tail = (tail + 1) & (QUEUESIZE - 1);
            c
        } else {
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn HU_Responder(ev: *mut event_t) -> c_int {
    unsafe {
        static mut lastmessage: [c_char; 81] = [0; 81];
        static mut altdown: bool = false;
        static mut num_nobrainers: c_int = 0;

        let ev = &mut *ev;

        let mut numplayers = 0;
        for i in 0..MAXPLAYERS {
            if playeringame[i] != 0 {
                numplayers += 1;
            }
        }

        if ev.data1 == KEY_RSHIFT as c_int {
            return 0;
        } else if ev.data1 == KEY_RALT as c_int || ev.data1 == KEY_LALT as c_int {
            altdown = ev.type_ == 0; // ev_keydown
            return 0;
        }

        if ev.type_ != 0 {
            // not ev_keydown
            return 0;
        }

        let mut eatkey = false;

        if chat_on == 0 {
            if ev.data1 == key_message_refresh {
                message_on = 1;
                message_counter = HU_MSGTIMEOUT;
                eatkey = true;
            } else if netgame != 0 && ev.data2 == key_multi_msg {
                eatkey = true;
                chat_on = 1;
                HUlib_resetIText(&raw mut w_chat);
                HU_queueChatChar(HU_BROADCAST as c_char);
            } else if netgame != 0 && numplayers > 2 {
                for i in 0..MAXPLAYERS {
                    if ev.data2 == key_multi_msgplayer[i] {
                        if playeringame[i] != 0 && i != consoleplayer as usize {
                            eatkey = true;
                            chat_on = 1;
                            HUlib_resetIText(&raw mut w_chat);
                            HU_queueChatChar((i + 1) as c_char);
                            break;
                        } else if i == consoleplayer as usize {
                            num_nobrainers += 1;
                            (*plr).message = match num_nobrainers {
                                1..=2 => cstr!("You mumble to yourself"),
                                3..=5 => cstr!("Who's there?"),
                                6..=8 => cstr!("You scare yourself"),
                                9..=31 => cstr!("You start to rave"),
                                _ => cstr!("You've lost it..."),
                            };
                        }
                    }
                }
            }
        } else {
            // send a macro
            if altdown {
                let c = (ev.data1 - b'0' as c_int) as u8;
                if c > 9 {
                    return 0;
                }
                let macromessage = chat_macros[c as usize];
                HU_queueChatChar(KEY_ENTER as c_char);
                let mut p = macromessage;
                while *p != 0 {
                    HU_queueChatChar(*p);
                    p = p.add(1);
                }
                HU_queueChatChar(KEY_ENTER as c_char);
                chat_on = 0;
                M_StringCopy(std::ptr::addr_of_mut!(lastmessage[0]), macromessage, 81);
                (*plr).message = std::ptr::addr_of_mut!(lastmessage[0]);
                eatkey = true;
            } else {
                let c = ev.data2 as u8;
                eatkey = HUlib_keyInIText(&raw mut w_chat, c) != 0;
                if eatkey {
                    HU_queueChatChar(c as c_char);
                }
                if c == KEY_ENTER {
                    chat_on = 0;
                    if w_chat.l.len != 0 {
                        M_StringCopy(
                            std::ptr::addr_of_mut!(lastmessage[0]),
                            std::ptr::addr_of_mut!(w_chat.l.l[0]),
                            81,
                        );
                        (*plr).message = std::ptr::addr_of_mut!(lastmessage[0]);
                    }
                } else if c == KEY_ESCAPE {
                    chat_on = 0;
                }
            }
        }

        if eatkey {
            1
        } else {
            0
        }
    }
}
