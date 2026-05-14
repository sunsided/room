#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

use std::ffi::{c_char, c_int};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

pub const NUMSTATES: usize = 967;
pub const NUMMOBJTYPES: usize = 137;

const FRACUNIT: c_int = 65536;

// Sprite name indices

pub const SPR_TROO: c_int = 0;
pub const SPR_SHTG: c_int = 1;
pub const SPR_PUNG: c_int = 2;
pub const SPR_PISG: c_int = 3;
pub const SPR_PISF: c_int = 4;
pub const SPR_SHTF: c_int = 5;
pub const SPR_SHT2: c_int = 6;
pub const SPR_CHGG: c_int = 7;
pub const SPR_CHGF: c_int = 8;
pub const SPR_MISG: c_int = 9;
pub const SPR_MISF: c_int = 10;
pub const SPR_SAWG: c_int = 11;
pub const SPR_PLSG: c_int = 12;
pub const SPR_PLSF: c_int = 13;
pub const SPR_BFGG: c_int = 14;
pub const SPR_BFGF: c_int = 15;
pub const SPR_BLUD: c_int = 16;
pub const SPR_PUFF: c_int = 17;
pub const SPR_BAL1: c_int = 18;
pub const SPR_BAL2: c_int = 19;
pub const SPR_PLSS: c_int = 20;
pub const SPR_PLSE: c_int = 21;
pub const SPR_MISL: c_int = 22;
pub const SPR_BFS1: c_int = 23;
pub const SPR_BFE1: c_int = 24;
pub const SPR_BFE2: c_int = 25;
pub const SPR_TFOG: c_int = 26;
pub const SPR_IFOG: c_int = 27;
pub const SPR_PLAY: c_int = 28;
pub const SPR_POSS: c_int = 29;
pub const SPR_SPOS: c_int = 30;
pub const SPR_VILE: c_int = 31;
pub const SPR_FIRE: c_int = 32;
pub const SPR_FATB: c_int = 33;
pub const SPR_FBXP: c_int = 34;
pub const SPR_SKEL: c_int = 35;
pub const SPR_MANF: c_int = 36;
pub const SPR_FATT: c_int = 37;
pub const SPR_CPOS: c_int = 38;
pub const SPR_SARG: c_int = 39;
pub const SPR_HEAD: c_int = 40;
pub const SPR_BAL7: c_int = 41;
pub const SPR_BOSS: c_int = 42;
pub const SPR_BOS2: c_int = 43;
pub const SPR_SKUL: c_int = 44;
pub const SPR_SPID: c_int = 45;
pub const SPR_BSPI: c_int = 46;
pub const SPR_APLS: c_int = 47;
pub const SPR_APBX: c_int = 48;
pub const SPR_CYBR: c_int = 49;
pub const SPR_PAIN: c_int = 50;
pub const SPR_SSWV: c_int = 51;
pub const SPR_KEEN: c_int = 52;
pub const SPR_BBRN: c_int = 53;
pub const SPR_BOSF: c_int = 54;
pub const SPR_ARM1: c_int = 55;
pub const SPR_ARM2: c_int = 56;
pub const SPR_BAR1: c_int = 57;
pub const SPR_BEXP: c_int = 58;
pub const SPR_FCAN: c_int = 59;
pub const SPR_BON1: c_int = 60;
pub const SPR_BON2: c_int = 61;
pub const SPR_BKEY: c_int = 62;
pub const SPR_RKEY: c_int = 63;
pub const SPR_YKEY: c_int = 64;
pub const SPR_BSKU: c_int = 65;
pub const SPR_RSKU: c_int = 66;
pub const SPR_YSKU: c_int = 67;
pub const SPR_STIM: c_int = 68;
pub const SPR_MEDI: c_int = 69;
pub const SPR_SOUL: c_int = 70;
pub const SPR_PINV: c_int = 71;
pub const SPR_PSTR: c_int = 72;
pub const SPR_PINS: c_int = 73;
pub const SPR_MEGA: c_int = 74;
pub const SPR_SUIT: c_int = 75;
pub const SPR_PMAP: c_int = 76;
pub const SPR_PVIS: c_int = 77;
pub const SPR_CLIP: c_int = 78;
pub const SPR_AMMO: c_int = 79;
pub const SPR_ROCK: c_int = 80;
pub const SPR_BROK: c_int = 81;
pub const SPR_CELL: c_int = 82;
pub const SPR_CELP: c_int = 83;
pub const SPR_SHEL: c_int = 84;
pub const SPR_SBOX: c_int = 85;
pub const SPR_BPAK: c_int = 86;
pub const SPR_BFUG: c_int = 87;
pub const SPR_MGUN: c_int = 88;
pub const SPR_CSAW: c_int = 89;
pub const SPR_LAUN: c_int = 90;
pub const SPR_PLAS: c_int = 91;
pub const SPR_SHOT: c_int = 92;
pub const SPR_SGN2: c_int = 93;
pub const SPR_COLU: c_int = 94;
pub const SPR_SMT2: c_int = 95;
pub const SPR_GOR1: c_int = 96;
pub const SPR_POL2: c_int = 97;
pub const SPR_POL5: c_int = 98;
pub const SPR_POL4: c_int = 99;
pub const SPR_POL3: c_int = 100;
pub const SPR_POL1: c_int = 101;
pub const SPR_POL6: c_int = 102;
pub const SPR_GOR2: c_int = 103;
pub const SPR_GOR3: c_int = 104;
pub const SPR_GOR4: c_int = 105;
pub const SPR_GOR5: c_int = 106;
pub const SPR_SMIT: c_int = 107;
pub const SPR_COL1: c_int = 108;
pub const SPR_COL2: c_int = 109;
pub const SPR_COL3: c_int = 110;
pub const SPR_COL4: c_int = 111;
pub const SPR_CAND: c_int = 112;
pub const SPR_CBRA: c_int = 113;
pub const SPR_COL6: c_int = 114;
pub const SPR_TRE1: c_int = 115;
pub const SPR_TRE2: c_int = 116;
pub const SPR_ELEC: c_int = 117;
pub const SPR_CEYE: c_int = 118;
pub const SPR_FSKU: c_int = 119;
pub const SPR_COL5: c_int = 120;
pub const SPR_TBLU: c_int = 121;
pub const SPR_TGRN: c_int = 122;
pub const SPR_TRED: c_int = 123;
pub const SPR_SMBT: c_int = 124;
pub const SPR_SMGT: c_int = 125;
pub const SPR_SMRT: c_int = 126;
pub const SPR_HDB1: c_int = 127;
pub const SPR_HDB2: c_int = 128;
pub const SPR_HDB3: c_int = 129;
pub const SPR_HDB4: c_int = 130;
pub const SPR_HDB5: c_int = 131;
pub const SPR_HDB6: c_int = 132;
pub const SPR_POB1: c_int = 133;
pub const SPR_POB2: c_int = 134;
pub const SPR_BRS1: c_int = 135;
pub const SPR_TLMP: c_int = 136;
pub const SPR_TLP2: c_int = 137;

// Sound index constants

const sfx_None: c_int = 0;
const sfx_pistol: c_int = 1;
const sfx_shotgn: c_int = 2;
const sfx_sgcock: c_int = 3;
const sfx_dshtgn: c_int = 4;
const sfx_dbopn: c_int = 5;
const sfx_dbcls: c_int = 6;
const sfx_dbload: c_int = 7;
const sfx_plasma: c_int = 8;
const sfx_bfg: c_int = 9;
const sfx_sawup: c_int = 10;
const sfx_sawidl: c_int = 11;
const sfx_sawful: c_int = 12;
const sfx_sawhit: c_int = 13;
const sfx_rlaunc: c_int = 14;
const sfx_rxplod: c_int = 15;
const sfx_firsht: c_int = 16;
const sfx_firxpl: c_int = 17;
const sfx_pstart: c_int = 18;
const sfx_pstop: c_int = 19;
const sfx_doropn: c_int = 20;
const sfx_dorcls: c_int = 21;
const sfx_stnmov: c_int = 22;
const sfx_swtchn: c_int = 23;
const sfx_swtchx: c_int = 24;
const sfx_plpain: c_int = 25;
const sfx_dmpain: c_int = 26;
const sfx_popain: c_int = 27;
const sfx_vipain: c_int = 28;
const sfx_mnpain: c_int = 29;
const sfx_pepain: c_int = 30;
const sfx_slop: c_int = 31;
const sfx_itemup: c_int = 32;
const sfx_wpnup: c_int = 33;
const sfx_oof: c_int = 34;
const sfx_telept: c_int = 35;
const sfx_posit1: c_int = 36;
const sfx_posit2: c_int = 37;
const sfx_posit3: c_int = 38;
const sfx_bgsit1: c_int = 39;
const sfx_bgsit2: c_int = 40;
const sfx_sgtsit: c_int = 41;
const sfx_cacsit: c_int = 42;
const sfx_brssit: c_int = 43;
const sfx_cybsit: c_int = 44;
const sfx_spisit: c_int = 45;
const sfx_bspsit: c_int = 46;
const sfx_kntsit: c_int = 47;
const sfx_vilsit: c_int = 48;
const sfx_mansit: c_int = 49;
const sfx_pesit: c_int = 50;
const sfx_sklatk: c_int = 51;
const sfx_sgtatk: c_int = 52;
const sfx_skepch: c_int = 53;
const sfx_vilatk: c_int = 54;
const sfx_claw: c_int = 55;
const sfx_skeswg: c_int = 56;
const sfx_pldeth: c_int = 57;
const sfx_pdiehi: c_int = 58;
const sfx_podth1: c_int = 59;
const sfx_podth2: c_int = 60;
const sfx_podth3: c_int = 61;
const sfx_bgdth1: c_int = 62;
const sfx_bgdth2: c_int = 63;
const sfx_sgtdth: c_int = 64;
const sfx_cacdth: c_int = 65;
const sfx_skldth: c_int = 66;
const sfx_brsdth: c_int = 67;
const sfx_cybdth: c_int = 68;
const sfx_spidth: c_int = 69;
const sfx_bspdth: c_int = 70;
const sfx_vildth: c_int = 71;
const sfx_kntdth: c_int = 72;
const sfx_pedth: c_int = 73;
const sfx_skedth: c_int = 74;
const sfx_posact: c_int = 75;
const sfx_bgact: c_int = 76;
const sfx_dmact: c_int = 77;
const sfx_bspact: c_int = 78;
const sfx_bspwlk: c_int = 79;
const sfx_vilact: c_int = 80;
const sfx_noway: c_int = 81;
const sfx_barexp: c_int = 82;
const sfx_punch: c_int = 83;
const sfx_hoof: c_int = 84;
const sfx_metal: c_int = 85;
const sfx_chgun: c_int = 86;
const sfx_tink: c_int = 87;
const sfx_bdopn: c_int = 88;
const sfx_bdcls: c_int = 89;
const sfx_itmbk: c_int = 90;
const sfx_flame: c_int = 91;
const sfx_flamst: c_int = 92;
const sfx_getpow: c_int = 93;
const sfx_bospit: c_int = 94;
const sfx_boscub: c_int = 95;
const sfx_bossit: c_int = 96;
const sfx_bospn: c_int = 97;
const sfx_bosdth: c_int = 98;
const sfx_manatk: c_int = 99;
const sfx_mandth: c_int = 100;
const sfx_sssit: c_int = 101;
const sfx_ssdth: c_int = 102;
const sfx_keenpn: c_int = 103;
const sfx_keendt: c_int = 104;
const sfx_skeact: c_int = 105;
const sfx_skesit: c_int = 106;
const sfx_skeatk: c_int = 107;
const sfx_radio: c_int = 108;

include!(concat!(env!("OUT_DIR"), "/statenum.rs"));

// MF_* flag constants

// Mobj flag bit values. These MUST match `p_mobj.h`'s `mobjflag_t` enum
// exactly: `info.rs` initialises `mobjinfo[].flags` using these constants,
// and the C simulation code (`p_map.c`, `p_enemy.c`, `p_inter.c`, ...) tests
// those stored bits against the `p_mobj.h` values. A mismatch silently
// corrupts every `flags & MF_*` check in the game.
pub const MF_SPECIAL: c_int = 0x00000001;
pub const MF_SOLID: c_int = 0x00000002;
pub const MF_SHOOTABLE: c_int = 0x00000004;
pub const MF_NOSECTOR: c_int = 0x00000008;
pub const MF_NOBLOCKMAP: c_int = 0x00000010;
pub const MF_AMBUSH: c_int = 0x00000020;
pub const MF_JUSTHIT: c_int = 0x00000040;
pub const MF_JUSTATTACKED: c_int = 0x00000080;
pub const MF_SPAWNCEILING: c_int = 0x00000100;
pub const MF_NOGRAVITY: c_int = 0x00000200;
pub const MF_DROPOFF: c_int = 0x00000400;
pub const MF_PICKUP: c_int = 0x00000800;
pub const MF_NOCLIP: c_int = 0x00001000;
pub const MF_SLIDE: c_int = 0x00002000;
pub const MF_FLOAT: c_int = 0x00004000;
pub const MF_TELEPORT: c_int = 0x00008000;
pub const MF_MISSILE: c_int = 0x00010000;
pub const MF_DROPPED: c_int = 0x00020000;
pub const MF_SHADOW: c_int = 0x00040000;
pub const MF_NOBLOOD: c_int = 0x00080000;
pub const MF_CORPSE: c_int = 0x00100000;
pub const MF_INFLOAT: c_int = 0x00200000;
pub const MF_COUNTKILL: c_int = 0x00400000;
pub const MF_COUNTITEM: c_int = 0x00800000;
pub const MF_SKULLFLY: c_int = 0x01000000;
pub const MF_NOTDMATCH: c_int = 0x02000000;
pub const MF_TRANSLATION: c_int = 0x0c000000;
pub const MF_TRANSSHIFT: c_int = 26;

// Mobj type indices (from info.h mobjtype_t enum).
pub const MT_PLAYER: c_int = 0;
pub const MT_POSSESSED: c_int = 1;
pub const MT_SHOTGUY: c_int = 2;
pub const MT_VILE: c_int = 3;
pub const MT_FIRE: c_int = 4;
pub const MT_UNDEAD: c_int = 5;
pub const MT_TRACER: c_int = 6;
pub const MT_SMOKE: c_int = 7;
pub const MT_FATSO: c_int = 8;
pub const MT_FATSHOT: c_int = 9;
pub const MT_CHAINGUY: c_int = 10;
pub const MT_TROOP: c_int = 11;
pub const MT_SERGEANT: c_int = 12;
pub const MT_SHADOWS: c_int = 13;
pub const MT_HEAD: c_int = 14;
pub const MT_BRUISER: c_int = 15;
pub const MT_BRUISERSHOT: c_int = 16;
pub const MT_KNIGHT: c_int = 17;
pub const MT_SKULL: c_int = 18;
pub const MT_SPIDER: c_int = 19;
pub const MT_BABY: c_int = 20;
pub const MT_CYBORG: c_int = 21;
pub const MT_PAIN: c_int = 22;
pub const MT_WOLFSS: c_int = 23;
pub const MT_KEEN: c_int = 24;
pub const MT_BOSSBRAIN: c_int = 25;
pub const MT_BOSSSPIT: c_int = 26;
pub const MT_BOSSTARGET: c_int = 27;
pub const MT_SPAWNSHOT: c_int = 28;
pub const MT_SPAWNFIRE: c_int = 29;
pub const MT_BARREL: c_int = 30;
pub const MT_TROOPSHOT: c_int = 31;
pub const MT_HEADSHOT: c_int = 32;
pub const MT_ROCKET: c_int = 33;
pub const MT_PLASMA: c_int = 34;
pub const MT_BFG: c_int = 35;
pub const MT_ARACHPLAZ: c_int = 36;
pub const MT_PUFF: c_int = 37;
pub const MT_BLOOD: c_int = 38;
pub const MT_TFOG: c_int = 39;
pub const MT_IFOG: c_int = 40;
pub const MT_TELEPORTMAN: c_int = 41;
pub const MT_EXTRABFG: c_int = 42;
pub const MT_MISC0: c_int = 43;
pub const MT_MISC1: c_int = 44;
pub const MT_MISC2: c_int = 45;
pub const MT_MISC3: c_int = 46;
pub const MT_MISC4: c_int = 47;
pub const MT_MISC5: c_int = 48;
pub const MT_MISC6: c_int = 49;
pub const MT_MISC7: c_int = 50;
pub const MT_MISC8: c_int = 51;
pub const MT_MISC9: c_int = 52;
pub const MT_MISC10: c_int = 53;
pub const MT_MISC11: c_int = 54;
pub const MT_MISC12: c_int = 55;
pub const MT_INV: c_int = 56;
pub const MT_MISC13: c_int = 57;
pub const MT_INS: c_int = 58;
pub const MT_MISC14: c_int = 59;
pub const MT_MISC15: c_int = 60;
pub const MT_MISC16: c_int = 61;
pub const MT_MEGA: c_int = 62;
pub const MT_CLIP: c_int = 63;
pub const MT_MISC17: c_int = 64;
pub const MT_MISC18: c_int = 65;
pub const MT_MISC19: c_int = 66;
pub const MT_MISC20: c_int = 67;
pub const MT_MISC21: c_int = 68;
pub const MT_MISC22: c_int = 69;
pub const MT_MISC23: c_int = 70;
pub const MT_MISC24: c_int = 71;
pub const MT_MISC25: c_int = 72;
pub const MT_CHAINGUN: c_int = 73;
pub const MT_MISC26: c_int = 74;
pub const MT_MISC27: c_int = 75;
pub const MT_MISC28: c_int = 76;
pub const MT_SHOTGUN: c_int = 77;
pub const MT_SUPERSHOTGUN: c_int = 78;
pub const MT_MISC29: c_int = 79;
pub const MT_MISC30: c_int = 80;
pub const MT_MISC31: c_int = 81;
pub const MT_MISC32: c_int = 82;
pub const MT_MISC33: c_int = 83;
pub const MT_MISC34: c_int = 84;
pub const MT_MISC35: c_int = 85;
pub const MT_MISC36: c_int = 86;
pub const MT_MISC37: c_int = 87;
pub const MT_MISC38: c_int = 88;
pub const MT_MISC39: c_int = 89;
pub const MT_MISC40: c_int = 90;
pub const MT_MISC41: c_int = 91;
pub const MT_MISC42: c_int = 92;
pub const MT_MISC43: c_int = 93;
pub const MT_MISC44: c_int = 94;
pub const MT_MISC45: c_int = 95;
pub const MT_MISC46: c_int = 96;
pub const MT_MISC47: c_int = 97;
pub const MT_MISC48: c_int = 98;
pub const MT_MISC49: c_int = 99;
pub const MT_MISC50: c_int = 100;
pub const MT_MISC51: c_int = 101;
pub const MT_MISC52: c_int = 102;
pub const MT_MISC53: c_int = 103;
pub const MT_MISC54: c_int = 104;
pub const MT_MISC55: c_int = 105;
pub const MT_MISC56: c_int = 106;
pub const MT_MISC57: c_int = 107;
pub const MT_MISC58: c_int = 108;
pub const MT_MISC59: c_int = 109;
pub const MT_MISC60: c_int = 110;
pub const MT_MISC61: c_int = 111;
pub const MT_MISC62: c_int = 112;
pub const MT_MISC63: c_int = 113;
pub const MT_MISC64: c_int = 114;
pub const MT_MISC65: c_int = 115;
pub const MT_MISC66: c_int = 116;
pub const MT_MISC67: c_int = 117;
pub const MT_MISC68: c_int = 118;
pub const MT_MISC69: c_int = 119;
pub const MT_MISC70: c_int = 120;
pub const MT_MISC71: c_int = 121;
pub const MT_MISC72: c_int = 122;
pub const MT_MISC73: c_int = 123;
pub const MT_MISC74: c_int = 124;
pub const MT_MISC75: c_int = 125;
pub const MT_MISC76: c_int = 126;
pub const MT_MISC77: c_int = 127;
pub const MT_MISC78: c_int = 128;
pub const MT_MISC79: c_int = 129;
pub const MT_MISC80: c_int = 130;
pub const MT_MISC81: c_int = 131;
pub const MT_MISC82: c_int = 132;
pub const MT_MISC83: c_int = 133;
pub const MT_MISC84: c_int = 134;
pub const MT_MISC85: c_int = 135;
pub const MT_MISC86: c_int = 136;

#[repr(C)]
pub struct State {
    pub sprite: c_int,
    pub frame: c_int,
    pub tics: c_int,
    pub action: Option<unsafe extern "C" fn()>,
    pub nextstate: c_int,
    pub misc1: c_int,
    pub misc2: c_int,
}

#[repr(C)]
pub struct MobjInfo {
    pub doomednum: c_int,
    pub spawnstate: c_int,
    pub spawnhealth: c_int,
    pub seestate: c_int,
    pub seesound: c_int,
    pub reactiontime: c_int,
    pub attacksound: c_int,
    pub painstate: c_int,
    pub painchance: c_int,
    pub painsound: c_int,
    pub meleestate: c_int,
    pub missilestate: c_int,
    pub deathstate: c_int,
    pub xdeathstate: c_int,
    pub deathsound: c_int,
    pub speed: c_int,
    pub radius: c_int,
    pub height: c_int,
    pub mass: c_int,
    pub damage: c_int,
    pub activesound: c_int,
    pub flags: c_int,
    pub raisestate: c_int,
}

const _: () = assert!(std::mem::size_of::<MobjInfo>() == 92);
const _: () = assert!(std::mem::offset_of!(MobjInfo, speed) == 60);

const _: () = assert!(std::mem::size_of::<State>() == 40);
const _: () = assert!(std::mem::offset_of!(State, tics) == 8);

extern "C" {
    fn A_Light0();
    fn A_WeaponReady();
    fn A_Lower();
    fn A_Raise();
    fn A_Punch();
    fn A_ReFire();
    fn A_FirePistol();
    fn A_Light1();
    fn A_FireShotgun();
    fn A_Light2();
    fn A_FireShotgun2();
    fn A_CheckReload();
    fn A_OpenShotgun2();
    fn A_LoadShotgun2();
    fn A_CloseShotgun2();
    fn A_FireCGun();
    fn A_GunFlash();
    fn A_FireMissile();
    fn A_Saw();
    fn A_FirePlasma();
    fn A_BFGsound();
    fn A_FireBFG();
    fn A_BFGSpray();
    fn A_Explode();
    fn A_Pain();
    fn A_PlayerScream();
    fn A_Fall();
    fn A_XScream();
    fn A_Look();
    fn A_Chase();
    fn A_FaceTarget();
    fn A_PosAttack();
    fn A_Scream();
    fn A_SPosAttack();
    fn A_VileChase();
    fn A_VileStart();
    fn A_VileTarget();
    fn A_VileAttack();
    fn A_StartFire();
    fn A_Fire();
    fn A_FireCrackle();
    fn A_Tracer();
    fn A_SkelWhoosh();
    fn A_SkelFist();
    fn A_SkelMissile();
    fn A_FatRaise();
    fn A_FatAttack1();
    fn A_FatAttack2();
    fn A_FatAttack3();
    fn A_BossDeath();
    fn A_CPosAttack();
    fn A_CPosRefire();
    fn A_TroopAttack();
    fn A_SargAttack();
    fn A_HeadAttack();
    fn A_BruisAttack();
    fn A_SkullAttack();
    fn A_Metal();
    fn A_SpidRefire();
    fn A_BabyMetal();
    fn A_BspiAttack();
    fn A_Hoof();
    fn A_CyberAttack();
    fn A_PainAttack();
    fn A_PainDie();
    fn A_KeenDie();
    fn A_BrainPain();
    fn A_BrainScream();
    fn A_BrainDie();
    fn A_BrainAwake();
    fn A_BrainSpit();
    fn A_SpawnSound();
    fn A_SpawnFly();
    fn A_BrainExplode();
}

#[no_mangle]
pub static mut sprnames: [*mut c_char; 139] = [
    cstr(b"TROO\0"),
    cstr(b"SHTG\0"),
    cstr(b"PUNG\0"),
    cstr(b"PISG\0"),
    cstr(b"PISF\0"),
    cstr(b"SHTF\0"),
    cstr(b"SHT2\0"),
    cstr(b"CHGG\0"),
    cstr(b"CHGF\0"),
    cstr(b"MISG\0"),
    cstr(b"MISF\0"),
    cstr(b"SAWG\0"),
    cstr(b"PLSG\0"),
    cstr(b"PLSF\0"),
    cstr(b"BFGG\0"),
    cstr(b"BFGF\0"),
    cstr(b"BLUD\0"),
    cstr(b"PUFF\0"),
    cstr(b"BAL1\0"),
    cstr(b"BAL2\0"),
    cstr(b"PLSS\0"),
    cstr(b"PLSE\0"),
    cstr(b"MISL\0"),
    cstr(b"BFS1\0"),
    cstr(b"BFE1\0"),
    cstr(b"BFE2\0"),
    cstr(b"TFOG\0"),
    cstr(b"IFOG\0"),
    cstr(b"PLAY\0"),
    cstr(b"POSS\0"),
    cstr(b"SPOS\0"),
    cstr(b"VILE\0"),
    cstr(b"FIRE\0"),
    cstr(b"FATB\0"),
    cstr(b"FBXP\0"),
    cstr(b"SKEL\0"),
    cstr(b"MANF\0"),
    cstr(b"FATT\0"),
    cstr(b"CPOS\0"),
    cstr(b"SARG\0"),
    cstr(b"HEAD\0"),
    cstr(b"BAL7\0"),
    cstr(b"BOSS\0"),
    cstr(b"BOS2\0"),
    cstr(b"SKUL\0"),
    cstr(b"SPID\0"),
    cstr(b"BSPI\0"),
    cstr(b"APLS\0"),
    cstr(b"APBX\0"),
    cstr(b"CYBR\0"),
    cstr(b"PAIN\0"),
    cstr(b"SSWV\0"),
    cstr(b"KEEN\0"),
    cstr(b"BBRN\0"),
    cstr(b"BOSF\0"),
    cstr(b"ARM1\0"),
    cstr(b"ARM2\0"),
    cstr(b"BAR1\0"),
    cstr(b"BEXP\0"),
    cstr(b"FCAN\0"),
    cstr(b"BON1\0"),
    cstr(b"BON2\0"),
    cstr(b"BKEY\0"),
    cstr(b"RKEY\0"),
    cstr(b"YKEY\0"),
    cstr(b"BSKU\0"),
    cstr(b"RSKU\0"),
    cstr(b"YSKU\0"),
    cstr(b"STIM\0"),
    cstr(b"MEDI\0"),
    cstr(b"SOUL\0"),
    cstr(b"PINV\0"),
    cstr(b"PSTR\0"),
    cstr(b"PINS\0"),
    cstr(b"MEGA\0"),
    cstr(b"SUIT\0"),
    cstr(b"PMAP\0"),
    cstr(b"PVIS\0"),
    cstr(b"CLIP\0"),
    cstr(b"AMMO\0"),
    cstr(b"ROCK\0"),
    cstr(b"BROK\0"),
    cstr(b"CELL\0"),
    cstr(b"CELP\0"),
    cstr(b"SHEL\0"),
    cstr(b"SBOX\0"),
    cstr(b"BPAK\0"),
    cstr(b"BFUG\0"),
    cstr(b"MGUN\0"),
    cstr(b"CSAW\0"),
    cstr(b"LAUN\0"),
    cstr(b"PLAS\0"),
    cstr(b"SHOT\0"),
    cstr(b"SGN2\0"),
    cstr(b"COLU\0"),
    cstr(b"SMT2\0"),
    cstr(b"GOR1\0"),
    cstr(b"POL2\0"),
    cstr(b"POL5\0"),
    cstr(b"POL4\0"),
    cstr(b"POL3\0"),
    cstr(b"POL1\0"),
    cstr(b"POL6\0"),
    cstr(b"GOR2\0"),
    cstr(b"GOR3\0"),
    cstr(b"GOR4\0"),
    cstr(b"GOR5\0"),
    cstr(b"SMIT\0"),
    cstr(b"COL1\0"),
    cstr(b"COL2\0"),
    cstr(b"COL3\0"),
    cstr(b"COL4\0"),
    cstr(b"CAND\0"),
    cstr(b"CBRA\0"),
    cstr(b"COL6\0"),
    cstr(b"TRE1\0"),
    cstr(b"TRE2\0"),
    cstr(b"ELEC\0"),
    cstr(b"CEYE\0"),
    cstr(b"FSKU\0"),
    cstr(b"COL5\0"),
    cstr(b"TBLU\0"),
    cstr(b"TGRN\0"),
    cstr(b"TRED\0"),
    cstr(b"SMBT\0"),
    cstr(b"SMGT\0"),
    cstr(b"SMRT\0"),
    cstr(b"HDB1\0"),
    cstr(b"HDB2\0"),
    cstr(b"HDB3\0"),
    cstr(b"HDB4\0"),
    cstr(b"HDB5\0"),
    cstr(b"HDB6\0"),
    cstr(b"POB1\0"),
    cstr(b"POB2\0"),
    cstr(b"BRS1\0"),
    cstr(b"TLMP\0"),
    cstr(b"TLP2\0"),
    std::ptr::null_mut(),
];

const fn cstr(b: &[u8]) -> *mut c_char {
    b.as_ptr() as *mut c_char
}

#[no_mangle]
pub static mut states: [State; NUMSTATES] = [
    State {
        sprite: SPR_TROO,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_NULL
    State {
        sprite: SPR_SHTG,
        frame: 4,
        tics: 0,
        action: Some(A_Light0),
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_LIGHTDONE
    State {
        sprite: SPR_PUNG,
        frame: 0,
        tics: 1,
        action: Some(A_WeaponReady),
        nextstate: S_PUNCH,
        misc1: 0,
        misc2: 0,
    }, // S_PUNCH
    State {
        sprite: SPR_PUNG,
        frame: 0,
        tics: 1,
        action: Some(A_Lower),
        nextstate: S_PUNCHDOWN,
        misc1: 0,
        misc2: 0,
    }, // S_PUNCHDOWN
    State {
        sprite: SPR_PUNG,
        frame: 0,
        tics: 1,
        action: Some(A_Raise),
        nextstate: S_PUNCHUP,
        misc1: 0,
        misc2: 0,
    }, // S_PUNCHUP
    State {
        sprite: SPR_PUNG,
        frame: 1,
        tics: 4,
        action: None,
        nextstate: S_PUNCH2,
        misc1: 0,
        misc2: 0,
    }, // S_PUNCH1
    State {
        sprite: SPR_PUNG,
        frame: 2,
        tics: 4,
        action: Some(A_Punch),
        nextstate: S_PUNCH3,
        misc1: 0,
        misc2: 0,
    }, // S_PUNCH2
    State {
        sprite: SPR_PUNG,
        frame: 3,
        tics: 5,
        action: None,
        nextstate: S_PUNCH4,
        misc1: 0,
        misc2: 0,
    }, // S_PUNCH3
    State {
        sprite: SPR_PUNG,
        frame: 2,
        tics: 4,
        action: None,
        nextstate: S_PUNCH5,
        misc1: 0,
        misc2: 0,
    }, // S_PUNCH4
    State {
        sprite: SPR_PUNG,
        frame: 1,
        tics: 5,
        action: Some(A_ReFire),
        nextstate: S_PUNCH,
        misc1: 0,
        misc2: 0,
    }, // S_PUNCH5
    State {
        sprite: SPR_PISG,
        frame: 0,
        tics: 1,
        action: Some(A_WeaponReady),
        nextstate: S_PISTOL,
        misc1: 0,
        misc2: 0,
    }, // S_PISTOL
    State {
        sprite: SPR_PISG,
        frame: 0,
        tics: 1,
        action: Some(A_Lower),
        nextstate: S_PISTOLDOWN,
        misc1: 0,
        misc2: 0,
    }, // S_PISTOLDOWN
    State {
        sprite: SPR_PISG,
        frame: 0,
        tics: 1,
        action: Some(A_Raise),
        nextstate: S_PISTOLUP,
        misc1: 0,
        misc2: 0,
    }, // S_PISTOLUP
    State {
        sprite: SPR_PISG,
        frame: 0,
        tics: 4,
        action: None,
        nextstate: S_PISTOL2,
        misc1: 0,
        misc2: 0,
    }, // S_PISTOL1
    State {
        sprite: SPR_PISG,
        frame: 1,
        tics: 6,
        action: Some(A_FirePistol),
        nextstate: S_PISTOL3,
        misc1: 0,
        misc2: 0,
    }, // S_PISTOL2
    State {
        sprite: SPR_PISG,
        frame: 2,
        tics: 4,
        action: None,
        nextstate: S_PISTOL4,
        misc1: 0,
        misc2: 0,
    }, // S_PISTOL3
    State {
        sprite: SPR_PISG,
        frame: 1,
        tics: 5,
        action: Some(A_ReFire),
        nextstate: S_PISTOL,
        misc1: 0,
        misc2: 0,
    }, // S_PISTOL4
    State {
        sprite: SPR_PISF,
        frame: 32768,
        tics: 7,
        action: Some(A_Light1),
        nextstate: S_LIGHTDONE,
        misc1: 0,
        misc2: 0,
    }, // S_PISTOLFLASH
    State {
        sprite: SPR_SHTG,
        frame: 0,
        tics: 1,
        action: Some(A_WeaponReady),
        nextstate: S_SGUN,
        misc1: 0,
        misc2: 0,
    }, // S_SGUN
    State {
        sprite: SPR_SHTG,
        frame: 0,
        tics: 1,
        action: Some(A_Lower),
        nextstate: S_SGUNDOWN,
        misc1: 0,
        misc2: 0,
    }, // S_SGUNDOWN
    State {
        sprite: SPR_SHTG,
        frame: 0,
        tics: 1,
        action: Some(A_Raise),
        nextstate: S_SGUNUP,
        misc1: 0,
        misc2: 0,
    }, // S_SGUNUP
    State {
        sprite: SPR_SHTG,
        frame: 0,
        tics: 3,
        action: None,
        nextstate: S_SGUN2,
        misc1: 0,
        misc2: 0,
    }, // S_SGUN1
    State {
        sprite: SPR_SHTG,
        frame: 0,
        tics: 7,
        action: Some(A_FireShotgun),
        nextstate: S_SGUN3,
        misc1: 0,
        misc2: 0,
    }, // S_SGUN2
    State {
        sprite: SPR_SHTG,
        frame: 1,
        tics: 5,
        action: None,
        nextstate: S_SGUN4,
        misc1: 0,
        misc2: 0,
    }, // S_SGUN3
    State {
        sprite: SPR_SHTG,
        frame: 2,
        tics: 5,
        action: None,
        nextstate: S_SGUN5,
        misc1: 0,
        misc2: 0,
    }, // S_SGUN4
    State {
        sprite: SPR_SHTG,
        frame: 3,
        tics: 4,
        action: None,
        nextstate: S_SGUN6,
        misc1: 0,
        misc2: 0,
    }, // S_SGUN5
    State {
        sprite: SPR_SHTG,
        frame: 2,
        tics: 5,
        action: None,
        nextstate: S_SGUN7,
        misc1: 0,
        misc2: 0,
    }, // S_SGUN6
    State {
        sprite: SPR_SHTG,
        frame: 1,
        tics: 5,
        action: None,
        nextstate: S_SGUN8,
        misc1: 0,
        misc2: 0,
    }, // S_SGUN7
    State {
        sprite: SPR_SHTG,
        frame: 0,
        tics: 3,
        action: None,
        nextstate: S_SGUN9,
        misc1: 0,
        misc2: 0,
    }, // S_SGUN8
    State {
        sprite: SPR_SHTG,
        frame: 0,
        tics: 7,
        action: Some(A_ReFire),
        nextstate: S_SGUN,
        misc1: 0,
        misc2: 0,
    }, // S_SGUN9
    State {
        sprite: SPR_SHTF,
        frame: 32768,
        tics: 4,
        action: Some(A_Light1),
        nextstate: S_SGUNFLASH2,
        misc1: 0,
        misc2: 0,
    }, // S_SGUNFLASH1
    State {
        sprite: SPR_SHTF,
        frame: 32769,
        tics: 3,
        action: Some(A_Light2),
        nextstate: S_LIGHTDONE,
        misc1: 0,
        misc2: 0,
    }, // S_SGUNFLASH2
    State {
        sprite: SPR_SHT2,
        frame: 0,
        tics: 1,
        action: Some(A_WeaponReady),
        nextstate: S_DSGUN,
        misc1: 0,
        misc2: 0,
    }, // S_DSGUN
    State {
        sprite: SPR_SHT2,
        frame: 0,
        tics: 1,
        action: Some(A_Lower),
        nextstate: S_DSGUNDOWN,
        misc1: 0,
        misc2: 0,
    }, // S_DSGUNDOWN
    State {
        sprite: SPR_SHT2,
        frame: 0,
        tics: 1,
        action: Some(A_Raise),
        nextstate: S_DSGUNUP,
        misc1: 0,
        misc2: 0,
    }, // S_DSGUNUP
    State {
        sprite: SPR_SHT2,
        frame: 0,
        tics: 3,
        action: None,
        nextstate: S_DSGUN2,
        misc1: 0,
        misc2: 0,
    }, // S_DSGUN1
    State {
        sprite: SPR_SHT2,
        frame: 0,
        tics: 7,
        action: Some(A_FireShotgun2),
        nextstate: S_DSGUN3,
        misc1: 0,
        misc2: 0,
    }, // S_DSGUN2
    State {
        sprite: SPR_SHT2,
        frame: 1,
        tics: 7,
        action: None,
        nextstate: S_DSGUN4,
        misc1: 0,
        misc2: 0,
    }, // S_DSGUN3
    State {
        sprite: SPR_SHT2,
        frame: 2,
        tics: 7,
        action: Some(A_CheckReload),
        nextstate: S_DSGUN5,
        misc1: 0,
        misc2: 0,
    }, // S_DSGUN4
    State {
        sprite: SPR_SHT2,
        frame: 3,
        tics: 7,
        action: Some(A_OpenShotgun2),
        nextstate: S_DSGUN6,
        misc1: 0,
        misc2: 0,
    }, // S_DSGUN5
    State {
        sprite: SPR_SHT2,
        frame: 4,
        tics: 7,
        action: None,
        nextstate: S_DSGUN7,
        misc1: 0,
        misc2: 0,
    }, // S_DSGUN6
    State {
        sprite: SPR_SHT2,
        frame: 5,
        tics: 7,
        action: Some(A_LoadShotgun2),
        nextstate: S_DSGUN8,
        misc1: 0,
        misc2: 0,
    }, // S_DSGUN7
    State {
        sprite: SPR_SHT2,
        frame: 6,
        tics: 6,
        action: None,
        nextstate: S_DSGUN9,
        misc1: 0,
        misc2: 0,
    }, // S_DSGUN8
    State {
        sprite: SPR_SHT2,
        frame: 7,
        tics: 6,
        action: Some(A_CloseShotgun2),
        nextstate: S_DSGUN10,
        misc1: 0,
        misc2: 0,
    }, // S_DSGUN9
    State {
        sprite: SPR_SHT2,
        frame: 0,
        tics: 5,
        action: Some(A_ReFire),
        nextstate: S_DSGUN,
        misc1: 0,
        misc2: 0,
    }, // S_DSGUN10
    State {
        sprite: SPR_SHT2,
        frame: 1,
        tics: 7,
        action: None,
        nextstate: S_DSNR2,
        misc1: 0,
        misc2: 0,
    }, // S_DSNR1
    State {
        sprite: SPR_SHT2,
        frame: 0,
        tics: 3,
        action: None,
        nextstate: S_DSGUNDOWN,
        misc1: 0,
        misc2: 0,
    }, // S_DSNR2
    State {
        sprite: SPR_SHT2,
        frame: 32776,
        tics: 5,
        action: Some(A_Light1),
        nextstate: S_DSGUNFLASH2,
        misc1: 0,
        misc2: 0,
    }, // S_DSGUNFLASH1
    State {
        sprite: SPR_SHT2,
        frame: 32777,
        tics: 4,
        action: Some(A_Light2),
        nextstate: S_LIGHTDONE,
        misc1: 0,
        misc2: 0,
    }, // S_DSGUNFLASH2
    State {
        sprite: SPR_CHGG,
        frame: 0,
        tics: 1,
        action: Some(A_WeaponReady),
        nextstate: S_CHAIN,
        misc1: 0,
        misc2: 0,
    }, // S_CHAIN
    State {
        sprite: SPR_CHGG,
        frame: 0,
        tics: 1,
        action: Some(A_Lower),
        nextstate: S_CHAINDOWN,
        misc1: 0,
        misc2: 0,
    }, // S_CHAINDOWN
    State {
        sprite: SPR_CHGG,
        frame: 0,
        tics: 1,
        action: Some(A_Raise),
        nextstate: S_CHAINUP,
        misc1: 0,
        misc2: 0,
    }, // S_CHAINUP
    State {
        sprite: SPR_CHGG,
        frame: 0,
        tics: 4,
        action: Some(A_FireCGun),
        nextstate: S_CHAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_CHAIN1
    State {
        sprite: SPR_CHGG,
        frame: 1,
        tics: 4,
        action: Some(A_FireCGun),
        nextstate: S_CHAIN3,
        misc1: 0,
        misc2: 0,
    }, // S_CHAIN2
    State {
        sprite: SPR_CHGG,
        frame: 1,
        tics: 0,
        action: Some(A_ReFire),
        nextstate: S_CHAIN,
        misc1: 0,
        misc2: 0,
    }, // S_CHAIN3
    State {
        sprite: SPR_CHGF,
        frame: 32768,
        tics: 5,
        action: Some(A_Light1),
        nextstate: S_LIGHTDONE,
        misc1: 0,
        misc2: 0,
    }, // S_CHAINFLASH1
    State {
        sprite: SPR_CHGF,
        frame: 32769,
        tics: 5,
        action: Some(A_Light2),
        nextstate: S_LIGHTDONE,
        misc1: 0,
        misc2: 0,
    }, // S_CHAINFLASH2
    State {
        sprite: SPR_MISG,
        frame: 0,
        tics: 1,
        action: Some(A_WeaponReady),
        nextstate: S_MISSILE,
        misc1: 0,
        misc2: 0,
    }, // S_MISSILE
    State {
        sprite: SPR_MISG,
        frame: 0,
        tics: 1,
        action: Some(A_Lower),
        nextstate: S_MISSILEDOWN,
        misc1: 0,
        misc2: 0,
    }, // S_MISSILEDOWN
    State {
        sprite: SPR_MISG,
        frame: 0,
        tics: 1,
        action: Some(A_Raise),
        nextstate: S_MISSILEUP,
        misc1: 0,
        misc2: 0,
    }, // S_MISSILEUP
    State {
        sprite: SPR_MISG,
        frame: 1,
        tics: 8,
        action: Some(A_GunFlash),
        nextstate: S_MISSILE2,
        misc1: 0,
        misc2: 0,
    }, // S_MISSILE1
    State {
        sprite: SPR_MISG,
        frame: 1,
        tics: 12,
        action: Some(A_FireMissile),
        nextstate: S_MISSILE3,
        misc1: 0,
        misc2: 0,
    }, // S_MISSILE2
    State {
        sprite: SPR_MISG,
        frame: 1,
        tics: 0,
        action: Some(A_ReFire),
        nextstate: S_MISSILE,
        misc1: 0,
        misc2: 0,
    }, // S_MISSILE3
    State {
        sprite: SPR_MISF,
        frame: 32768,
        tics: 3,
        action: Some(A_Light1),
        nextstate: S_MISSILEFLASH2,
        misc1: 0,
        misc2: 0,
    }, // S_MISSILEFLASH1
    State {
        sprite: SPR_MISF,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_MISSILEFLASH3,
        misc1: 0,
        misc2: 0,
    }, // S_MISSILEFLASH2
    State {
        sprite: SPR_MISF,
        frame: 32770,
        tics: 4,
        action: Some(A_Light2),
        nextstate: S_MISSILEFLASH4,
        misc1: 0,
        misc2: 0,
    }, // S_MISSILEFLASH3
    State {
        sprite: SPR_MISF,
        frame: 32771,
        tics: 4,
        action: Some(A_Light2),
        nextstate: S_LIGHTDONE,
        misc1: 0,
        misc2: 0,
    }, // S_MISSILEFLASH4
    State {
        sprite: SPR_SAWG,
        frame: 2,
        tics: 4,
        action: Some(A_WeaponReady),
        nextstate: S_SAWB,
        misc1: 0,
        misc2: 0,
    }, // S_SAW
    State {
        sprite: SPR_SAWG,
        frame: 3,
        tics: 4,
        action: Some(A_WeaponReady),
        nextstate: S_SAW,
        misc1: 0,
        misc2: 0,
    }, // S_SAWB
    State {
        sprite: SPR_SAWG,
        frame: 2,
        tics: 1,
        action: Some(A_Lower),
        nextstate: S_SAWDOWN,
        misc1: 0,
        misc2: 0,
    }, // S_SAWDOWN
    State {
        sprite: SPR_SAWG,
        frame: 2,
        tics: 1,
        action: Some(A_Raise),
        nextstate: S_SAWUP,
        misc1: 0,
        misc2: 0,
    }, // S_SAWUP
    State {
        sprite: SPR_SAWG,
        frame: 0,
        tics: 4,
        action: Some(A_Saw),
        nextstate: S_SAW2,
        misc1: 0,
        misc2: 0,
    }, // S_SAW1
    State {
        sprite: SPR_SAWG,
        frame: 1,
        tics: 4,
        action: Some(A_Saw),
        nextstate: S_SAW3,
        misc1: 0,
        misc2: 0,
    }, // S_SAW2
    State {
        sprite: SPR_SAWG,
        frame: 1,
        tics: 0,
        action: Some(A_ReFire),
        nextstate: S_SAW,
        misc1: 0,
        misc2: 0,
    }, // S_SAW3
    State {
        sprite: SPR_PLSG,
        frame: 0,
        tics: 1,
        action: Some(A_WeaponReady),
        nextstate: S_PLASMA,
        misc1: 0,
        misc2: 0,
    }, // S_PLASMA
    State {
        sprite: SPR_PLSG,
        frame: 0,
        tics: 1,
        action: Some(A_Lower),
        nextstate: S_PLASMADOWN,
        misc1: 0,
        misc2: 0,
    }, // S_PLASMADOWN
    State {
        sprite: SPR_PLSG,
        frame: 0,
        tics: 1,
        action: Some(A_Raise),
        nextstate: S_PLASMAUP,
        misc1: 0,
        misc2: 0,
    }, // S_PLASMAUP
    State {
        sprite: SPR_PLSG,
        frame: 0,
        tics: 3,
        action: Some(A_FirePlasma),
        nextstate: S_PLASMA2,
        misc1: 0,
        misc2: 0,
    }, // S_PLASMA1
    State {
        sprite: SPR_PLSG,
        frame: 1,
        tics: 20,
        action: Some(A_ReFire),
        nextstate: S_PLASMA,
        misc1: 0,
        misc2: 0,
    }, // S_PLASMA2
    State {
        sprite: SPR_PLSF,
        frame: 32768,
        tics: 4,
        action: Some(A_Light1),
        nextstate: S_LIGHTDONE,
        misc1: 0,
        misc2: 0,
    }, // S_PLASMAFLASH1
    State {
        sprite: SPR_PLSF,
        frame: 32769,
        tics: 4,
        action: Some(A_Light1),
        nextstate: S_LIGHTDONE,
        misc1: 0,
        misc2: 0,
    }, // S_PLASMAFLASH2
    State {
        sprite: SPR_BFGG,
        frame: 0,
        tics: 1,
        action: Some(A_WeaponReady),
        nextstate: S_BFG,
        misc1: 0,
        misc2: 0,
    }, // S_BFG
    State {
        sprite: SPR_BFGG,
        frame: 0,
        tics: 1,
        action: Some(A_Lower),
        nextstate: S_BFGDOWN,
        misc1: 0,
        misc2: 0,
    }, // S_BFGDOWN
    State {
        sprite: SPR_BFGG,
        frame: 0,
        tics: 1,
        action: Some(A_Raise),
        nextstate: S_BFGUP,
        misc1: 0,
        misc2: 0,
    }, // S_BFGUP
    State {
        sprite: SPR_BFGG,
        frame: 0,
        tics: 20,
        action: Some(A_BFGsound),
        nextstate: S_BFG2,
        misc1: 0,
        misc2: 0,
    }, // S_BFG1
    State {
        sprite: SPR_BFGG,
        frame: 1,
        tics: 10,
        action: Some(A_GunFlash),
        nextstate: S_BFG3,
        misc1: 0,
        misc2: 0,
    }, // S_BFG2
    State {
        sprite: SPR_BFGG,
        frame: 1,
        tics: 10,
        action: Some(A_FireBFG),
        nextstate: S_BFG4,
        misc1: 0,
        misc2: 0,
    }, // S_BFG3
    State {
        sprite: SPR_BFGG,
        frame: 1,
        tics: 20,
        action: Some(A_ReFire),
        nextstate: S_BFG,
        misc1: 0,
        misc2: 0,
    }, // S_BFG4
    State {
        sprite: SPR_BFGF,
        frame: 32768,
        tics: 11,
        action: Some(A_Light1),
        nextstate: S_BFGFLASH2,
        misc1: 0,
        misc2: 0,
    }, // S_BFGFLASH1
    State {
        sprite: SPR_BFGF,
        frame: 32769,
        tics: 6,
        action: Some(A_Light2),
        nextstate: S_LIGHTDONE,
        misc1: 0,
        misc2: 0,
    }, // S_BFGFLASH2
    State {
        sprite: SPR_BLUD,
        frame: 2,
        tics: 8,
        action: None,
        nextstate: S_BLOOD2,
        misc1: 0,
        misc2: 0,
    }, // S_BLOOD1
    State {
        sprite: SPR_BLUD,
        frame: 1,
        tics: 8,
        action: None,
        nextstate: S_BLOOD3,
        misc1: 0,
        misc2: 0,
    }, // S_BLOOD2
    State {
        sprite: SPR_BLUD,
        frame: 0,
        tics: 8,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BLOOD3
    State {
        sprite: SPR_PUFF,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_PUFF2,
        misc1: 0,
        misc2: 0,
    }, // S_PUFF1
    State {
        sprite: SPR_PUFF,
        frame: 1,
        tics: 4,
        action: None,
        nextstate: S_PUFF3,
        misc1: 0,
        misc2: 0,
    }, // S_PUFF2
    State {
        sprite: SPR_PUFF,
        frame: 2,
        tics: 4,
        action: None,
        nextstate: S_PUFF4,
        misc1: 0,
        misc2: 0,
    }, // S_PUFF3
    State {
        sprite: SPR_PUFF,
        frame: 3,
        tics: 4,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_PUFF4
    State {
        sprite: SPR_BAL1,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_TBALL2,
        misc1: 0,
        misc2: 0,
    }, // S_TBALL1
    State {
        sprite: SPR_BAL1,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_TBALL1,
        misc1: 0,
        misc2: 0,
    }, // S_TBALL2
    State {
        sprite: SPR_BAL1,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_TBALLX2,
        misc1: 0,
        misc2: 0,
    }, // S_TBALLX1
    State {
        sprite: SPR_BAL1,
        frame: 32771,
        tics: 6,
        action: None,
        nextstate: S_TBALLX3,
        misc1: 0,
        misc2: 0,
    }, // S_TBALLX2
    State {
        sprite: SPR_BAL1,
        frame: 32772,
        tics: 6,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_TBALLX3
    State {
        sprite: SPR_BAL2,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_RBALL2,
        misc1: 0,
        misc2: 0,
    }, // S_RBALL1
    State {
        sprite: SPR_BAL2,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_RBALL1,
        misc1: 0,
        misc2: 0,
    }, // S_RBALL2
    State {
        sprite: SPR_BAL2,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_RBALLX2,
        misc1: 0,
        misc2: 0,
    }, // S_RBALLX1
    State {
        sprite: SPR_BAL2,
        frame: 32771,
        tics: 6,
        action: None,
        nextstate: S_RBALLX3,
        misc1: 0,
        misc2: 0,
    }, // S_RBALLX2
    State {
        sprite: SPR_BAL2,
        frame: 32772,
        tics: 6,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_RBALLX3
    State {
        sprite: SPR_PLSS,
        frame: 32768,
        tics: 6,
        action: None,
        nextstate: S_PLASBALL2,
        misc1: 0,
        misc2: 0,
    }, // S_PLASBALL
    State {
        sprite: SPR_PLSS,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_PLASBALL,
        misc1: 0,
        misc2: 0,
    }, // S_PLASBALL2
    State {
        sprite: SPR_PLSE,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_PLASEXP2,
        misc1: 0,
        misc2: 0,
    }, // S_PLASEXP
    State {
        sprite: SPR_PLSE,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_PLASEXP3,
        misc1: 0,
        misc2: 0,
    }, // S_PLASEXP2
    State {
        sprite: SPR_PLSE,
        frame: 32770,
        tics: 4,
        action: None,
        nextstate: S_PLASEXP4,
        misc1: 0,
        misc2: 0,
    }, // S_PLASEXP3
    State {
        sprite: SPR_PLSE,
        frame: 32771,
        tics: 4,
        action: None,
        nextstate: S_PLASEXP5,
        misc1: 0,
        misc2: 0,
    }, // S_PLASEXP4
    State {
        sprite: SPR_PLSE,
        frame: 32772,
        tics: 4,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_PLASEXP5
    State {
        sprite: SPR_MISL,
        frame: 32768,
        tics: 1,
        action: None,
        nextstate: S_ROCKET,
        misc1: 0,
        misc2: 0,
    }, // S_ROCKET
    State {
        sprite: SPR_BFS1,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_BFGSHOT2,
        misc1: 0,
        misc2: 0,
    }, // S_BFGSHOT
    State {
        sprite: SPR_BFS1,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_BFGSHOT,
        misc1: 0,
        misc2: 0,
    }, // S_BFGSHOT2
    State {
        sprite: SPR_BFE1,
        frame: 32768,
        tics: 8,
        action: None,
        nextstate: S_BFGLAND2,
        misc1: 0,
        misc2: 0,
    }, // S_BFGLAND
    State {
        sprite: SPR_BFE1,
        frame: 32769,
        tics: 8,
        action: None,
        nextstate: S_BFGLAND3,
        misc1: 0,
        misc2: 0,
    }, // S_BFGLAND2
    State {
        sprite: SPR_BFE1,
        frame: 32770,
        tics: 8,
        action: Some(A_BFGSpray),
        nextstate: S_BFGLAND4,
        misc1: 0,
        misc2: 0,
    }, // S_BFGLAND3
    State {
        sprite: SPR_BFE1,
        frame: 32771,
        tics: 8,
        action: None,
        nextstate: S_BFGLAND5,
        misc1: 0,
        misc2: 0,
    }, // S_BFGLAND4
    State {
        sprite: SPR_BFE1,
        frame: 32772,
        tics: 8,
        action: None,
        nextstate: S_BFGLAND6,
        misc1: 0,
        misc2: 0,
    }, // S_BFGLAND5
    State {
        sprite: SPR_BFE1,
        frame: 32773,
        tics: 8,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BFGLAND6
    State {
        sprite: SPR_BFE2,
        frame: 32768,
        tics: 8,
        action: None,
        nextstate: S_BFGEXP2,
        misc1: 0,
        misc2: 0,
    }, // S_BFGEXP
    State {
        sprite: SPR_BFE2,
        frame: 32769,
        tics: 8,
        action: None,
        nextstate: S_BFGEXP3,
        misc1: 0,
        misc2: 0,
    }, // S_BFGEXP2
    State {
        sprite: SPR_BFE2,
        frame: 32770,
        tics: 8,
        action: None,
        nextstate: S_BFGEXP4,
        misc1: 0,
        misc2: 0,
    }, // S_BFGEXP3
    State {
        sprite: SPR_BFE2,
        frame: 32771,
        tics: 8,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BFGEXP4
    State {
        sprite: SPR_MISL,
        frame: 32769,
        tics: 8,
        action: Some(A_Explode),
        nextstate: S_EXPLODE2,
        misc1: 0,
        misc2: 0,
    }, // S_EXPLODE1
    State {
        sprite: SPR_MISL,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_EXPLODE3,
        misc1: 0,
        misc2: 0,
    }, // S_EXPLODE2
    State {
        sprite: SPR_MISL,
        frame: 32771,
        tics: 4,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_EXPLODE3
    State {
        sprite: SPR_TFOG,
        frame: 32768,
        tics: 6,
        action: None,
        nextstate: S_TFOG01,
        misc1: 0,
        misc2: 0,
    }, // S_TFOG
    State {
        sprite: SPR_TFOG,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_TFOG02,
        misc1: 0,
        misc2: 0,
    }, // S_TFOG01
    State {
        sprite: SPR_TFOG,
        frame: 32768,
        tics: 6,
        action: None,
        nextstate: S_TFOG2,
        misc1: 0,
        misc2: 0,
    }, // S_TFOG02
    State {
        sprite: SPR_TFOG,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_TFOG3,
        misc1: 0,
        misc2: 0,
    }, // S_TFOG2
    State {
        sprite: SPR_TFOG,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_TFOG4,
        misc1: 0,
        misc2: 0,
    }, // S_TFOG3
    State {
        sprite: SPR_TFOG,
        frame: 32771,
        tics: 6,
        action: None,
        nextstate: S_TFOG5,
        misc1: 0,
        misc2: 0,
    }, // S_TFOG4
    State {
        sprite: SPR_TFOG,
        frame: 32772,
        tics: 6,
        action: None,
        nextstate: S_TFOG6,
        misc1: 0,
        misc2: 0,
    }, // S_TFOG5
    State {
        sprite: SPR_TFOG,
        frame: 32773,
        tics: 6,
        action: None,
        nextstate: S_TFOG7,
        misc1: 0,
        misc2: 0,
    }, // S_TFOG6
    State {
        sprite: SPR_TFOG,
        frame: 32774,
        tics: 6,
        action: None,
        nextstate: S_TFOG8,
        misc1: 0,
        misc2: 0,
    }, // S_TFOG7
    State {
        sprite: SPR_TFOG,
        frame: 32775,
        tics: 6,
        action: None,
        nextstate: S_TFOG9,
        misc1: 0,
        misc2: 0,
    }, // S_TFOG8
    State {
        sprite: SPR_TFOG,
        frame: 32776,
        tics: 6,
        action: None,
        nextstate: S_TFOG10,
        misc1: 0,
        misc2: 0,
    }, // S_TFOG9
    State {
        sprite: SPR_TFOG,
        frame: 32777,
        tics: 6,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_TFOG10
    State {
        sprite: SPR_IFOG,
        frame: 32768,
        tics: 6,
        action: None,
        nextstate: S_IFOG01,
        misc1: 0,
        misc2: 0,
    }, // S_IFOG
    State {
        sprite: SPR_IFOG,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_IFOG02,
        misc1: 0,
        misc2: 0,
    }, // S_IFOG01
    State {
        sprite: SPR_IFOG,
        frame: 32768,
        tics: 6,
        action: None,
        nextstate: S_IFOG2,
        misc1: 0,
        misc2: 0,
    }, // S_IFOG02
    State {
        sprite: SPR_IFOG,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_IFOG3,
        misc1: 0,
        misc2: 0,
    }, // S_IFOG2
    State {
        sprite: SPR_IFOG,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_IFOG4,
        misc1: 0,
        misc2: 0,
    }, // S_IFOG3
    State {
        sprite: SPR_IFOG,
        frame: 32771,
        tics: 6,
        action: None,
        nextstate: S_IFOG5,
        misc1: 0,
        misc2: 0,
    }, // S_IFOG4
    State {
        sprite: SPR_IFOG,
        frame: 32772,
        tics: 6,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_IFOG5
    State {
        sprite: SPR_PLAY,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY
    State {
        sprite: SPR_PLAY,
        frame: 0,
        tics: 4,
        action: None,
        nextstate: S_PLAY_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_RUN1
    State {
        sprite: SPR_PLAY,
        frame: 1,
        tics: 4,
        action: None,
        nextstate: S_PLAY_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_RUN2
    State {
        sprite: SPR_PLAY,
        frame: 2,
        tics: 4,
        action: None,
        nextstate: S_PLAY_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_RUN3
    State {
        sprite: SPR_PLAY,
        frame: 3,
        tics: 4,
        action: None,
        nextstate: S_PLAY_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_RUN4
    State {
        sprite: SPR_PLAY,
        frame: 4,
        tics: 12,
        action: None,
        nextstate: S_PLAY,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_ATK1
    State {
        sprite: SPR_PLAY,
        frame: 32773,
        tics: 6,
        action: None,
        nextstate: S_PLAY_ATK1,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_ATK2
    State {
        sprite: SPR_PLAY,
        frame: 6,
        tics: 4,
        action: None,
        nextstate: S_PLAY_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_PAIN
    State {
        sprite: SPR_PLAY,
        frame: 6,
        tics: 4,
        action: Some(A_Pain),
        nextstate: S_PLAY,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_PAIN2
    State {
        sprite: SPR_PLAY,
        frame: 7,
        tics: 10,
        action: None,
        nextstate: S_PLAY_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_DIE1
    State {
        sprite: SPR_PLAY,
        frame: 8,
        tics: 10,
        action: Some(A_PlayerScream),
        nextstate: S_PLAY_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_DIE2
    State {
        sprite: SPR_PLAY,
        frame: 9,
        tics: 10,
        action: Some(A_Fall),
        nextstate: S_PLAY_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_DIE3
    State {
        sprite: SPR_PLAY,
        frame: 10,
        tics: 10,
        action: None,
        nextstate: S_PLAY_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_DIE4
    State {
        sprite: SPR_PLAY,
        frame: 11,
        tics: 10,
        action: None,
        nextstate: S_PLAY_DIE6,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_DIE5
    State {
        sprite: SPR_PLAY,
        frame: 12,
        tics: 10,
        action: None,
        nextstate: S_PLAY_DIE7,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_DIE6
    State {
        sprite: SPR_PLAY,
        frame: 13,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_DIE7
    State {
        sprite: SPR_PLAY,
        frame: 14,
        tics: 5,
        action: None,
        nextstate: S_PLAY_XDIE2,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_XDIE1
    State {
        sprite: SPR_PLAY,
        frame: 15,
        tics: 5,
        action: Some(A_XScream),
        nextstate: S_PLAY_XDIE3,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_XDIE2
    State {
        sprite: SPR_PLAY,
        frame: 16,
        tics: 5,
        action: Some(A_Fall),
        nextstate: S_PLAY_XDIE4,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_XDIE3
    State {
        sprite: SPR_PLAY,
        frame: 17,
        tics: 5,
        action: None,
        nextstate: S_PLAY_XDIE5,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_XDIE4
    State {
        sprite: SPR_PLAY,
        frame: 18,
        tics: 5,
        action: None,
        nextstate: S_PLAY_XDIE6,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_XDIE5
    State {
        sprite: SPR_PLAY,
        frame: 19,
        tics: 5,
        action: None,
        nextstate: S_PLAY_XDIE7,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_XDIE6
    State {
        sprite: SPR_PLAY,
        frame: 20,
        tics: 5,
        action: None,
        nextstate: S_PLAY_XDIE8,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_XDIE7
    State {
        sprite: SPR_PLAY,
        frame: 21,
        tics: 5,
        action: None,
        nextstate: S_PLAY_XDIE9,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_XDIE8
    State {
        sprite: SPR_PLAY,
        frame: 22,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_PLAY_XDIE9
    State {
        sprite: SPR_POSS,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_POSS_STND2,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_STND
    State {
        sprite: SPR_POSS,
        frame: 1,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_POSS_STND,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_STND2
    State {
        sprite: SPR_POSS,
        frame: 0,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_POSS_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_RUN1
    State {
        sprite: SPR_POSS,
        frame: 0,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_POSS_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_RUN2
    State {
        sprite: SPR_POSS,
        frame: 1,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_POSS_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_RUN3
    State {
        sprite: SPR_POSS,
        frame: 1,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_POSS_RUN5,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_RUN4
    State {
        sprite: SPR_POSS,
        frame: 2,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_POSS_RUN6,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_RUN5
    State {
        sprite: SPR_POSS,
        frame: 2,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_POSS_RUN7,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_RUN6
    State {
        sprite: SPR_POSS,
        frame: 3,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_POSS_RUN8,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_RUN7
    State {
        sprite: SPR_POSS,
        frame: 3,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_POSS_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_RUN8
    State {
        sprite: SPR_POSS,
        frame: 4,
        tics: 10,
        action: Some(A_FaceTarget),
        nextstate: S_POSS_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_ATK1
    State {
        sprite: SPR_POSS,
        frame: 5,
        tics: 8,
        action: Some(A_PosAttack),
        nextstate: S_POSS_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_ATK2
    State {
        sprite: SPR_POSS,
        frame: 4,
        tics: 8,
        action: None,
        nextstate: S_POSS_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_ATK3
    State {
        sprite: SPR_POSS,
        frame: 6,
        tics: 3,
        action: None,
        nextstate: S_POSS_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_PAIN
    State {
        sprite: SPR_POSS,
        frame: 6,
        tics: 3,
        action: Some(A_Pain),
        nextstate: S_POSS_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_PAIN2
    State {
        sprite: SPR_POSS,
        frame: 7,
        tics: 5,
        action: None,
        nextstate: S_POSS_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_DIE1
    State {
        sprite: SPR_POSS,
        frame: 8,
        tics: 5,
        action: Some(A_Scream),
        nextstate: S_POSS_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_DIE2
    State {
        sprite: SPR_POSS,
        frame: 9,
        tics: 5,
        action: Some(A_Fall),
        nextstate: S_POSS_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_DIE3
    State {
        sprite: SPR_POSS,
        frame: 10,
        tics: 5,
        action: None,
        nextstate: S_POSS_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_DIE4
    State {
        sprite: SPR_POSS,
        frame: 11,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_DIE5
    State {
        sprite: SPR_POSS,
        frame: 12,
        tics: 5,
        action: None,
        nextstate: S_POSS_XDIE2,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_XDIE1
    State {
        sprite: SPR_POSS,
        frame: 13,
        tics: 5,
        action: Some(A_XScream),
        nextstate: S_POSS_XDIE3,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_XDIE2
    State {
        sprite: SPR_POSS,
        frame: 14,
        tics: 5,
        action: Some(A_Fall),
        nextstate: S_POSS_XDIE4,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_XDIE3
    State {
        sprite: SPR_POSS,
        frame: 15,
        tics: 5,
        action: None,
        nextstate: S_POSS_XDIE5,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_XDIE4
    State {
        sprite: SPR_POSS,
        frame: 16,
        tics: 5,
        action: None,
        nextstate: S_POSS_XDIE6,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_XDIE5
    State {
        sprite: SPR_POSS,
        frame: 17,
        tics: 5,
        action: None,
        nextstate: S_POSS_XDIE7,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_XDIE6
    State {
        sprite: SPR_POSS,
        frame: 18,
        tics: 5,
        action: None,
        nextstate: S_POSS_XDIE8,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_XDIE7
    State {
        sprite: SPR_POSS,
        frame: 19,
        tics: 5,
        action: None,
        nextstate: S_POSS_XDIE9,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_XDIE8
    State {
        sprite: SPR_POSS,
        frame: 20,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_XDIE9
    State {
        sprite: SPR_POSS,
        frame: 10,
        tics: 5,
        action: None,
        nextstate: S_POSS_RAISE2,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_RAISE1
    State {
        sprite: SPR_POSS,
        frame: 9,
        tics: 5,
        action: None,
        nextstate: S_POSS_RAISE3,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_RAISE2
    State {
        sprite: SPR_POSS,
        frame: 8,
        tics: 5,
        action: None,
        nextstate: S_POSS_RAISE4,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_RAISE3
    State {
        sprite: SPR_POSS,
        frame: 7,
        tics: 5,
        action: None,
        nextstate: S_POSS_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_POSS_RAISE4
    State {
        sprite: SPR_SPOS,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_SPOS_STND2,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_STND
    State {
        sprite: SPR_SPOS,
        frame: 1,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_SPOS_STND,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_STND2
    State {
        sprite: SPR_SPOS,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPOS_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_RUN1
    State {
        sprite: SPR_SPOS,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPOS_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_RUN2
    State {
        sprite: SPR_SPOS,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPOS_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_RUN3
    State {
        sprite: SPR_SPOS,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPOS_RUN5,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_RUN4
    State {
        sprite: SPR_SPOS,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPOS_RUN6,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_RUN5
    State {
        sprite: SPR_SPOS,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPOS_RUN7,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_RUN6
    State {
        sprite: SPR_SPOS,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPOS_RUN8,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_RUN7
    State {
        sprite: SPR_SPOS,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPOS_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_RUN8
    State {
        sprite: SPR_SPOS,
        frame: 4,
        tics: 10,
        action: Some(A_FaceTarget),
        nextstate: S_SPOS_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_ATK1
    State {
        sprite: SPR_SPOS,
        frame: 32773,
        tics: 10,
        action: Some(A_SPosAttack),
        nextstate: S_SPOS_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_ATK2
    State {
        sprite: SPR_SPOS,
        frame: 4,
        tics: 10,
        action: None,
        nextstate: S_SPOS_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_ATK3
    State {
        sprite: SPR_SPOS,
        frame: 6,
        tics: 3,
        action: None,
        nextstate: S_SPOS_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_PAIN
    State {
        sprite: SPR_SPOS,
        frame: 6,
        tics: 3,
        action: Some(A_Pain),
        nextstate: S_SPOS_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_PAIN2
    State {
        sprite: SPR_SPOS,
        frame: 7,
        tics: 5,
        action: None,
        nextstate: S_SPOS_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_DIE1
    State {
        sprite: SPR_SPOS,
        frame: 8,
        tics: 5,
        action: Some(A_Scream),
        nextstate: S_SPOS_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_DIE2
    State {
        sprite: SPR_SPOS,
        frame: 9,
        tics: 5,
        action: Some(A_Fall),
        nextstate: S_SPOS_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_DIE3
    State {
        sprite: SPR_SPOS,
        frame: 10,
        tics: 5,
        action: None,
        nextstate: S_SPOS_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_DIE4
    State {
        sprite: SPR_SPOS,
        frame: 11,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_DIE5
    State {
        sprite: SPR_SPOS,
        frame: 12,
        tics: 5,
        action: None,
        nextstate: S_SPOS_XDIE2,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_XDIE1
    State {
        sprite: SPR_SPOS,
        frame: 13,
        tics: 5,
        action: Some(A_XScream),
        nextstate: S_SPOS_XDIE3,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_XDIE2
    State {
        sprite: SPR_SPOS,
        frame: 14,
        tics: 5,
        action: Some(A_Fall),
        nextstate: S_SPOS_XDIE4,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_XDIE3
    State {
        sprite: SPR_SPOS,
        frame: 15,
        tics: 5,
        action: None,
        nextstate: S_SPOS_XDIE5,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_XDIE4
    State {
        sprite: SPR_SPOS,
        frame: 16,
        tics: 5,
        action: None,
        nextstate: S_SPOS_XDIE6,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_XDIE5
    State {
        sprite: SPR_SPOS,
        frame: 17,
        tics: 5,
        action: None,
        nextstate: S_SPOS_XDIE7,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_XDIE6
    State {
        sprite: SPR_SPOS,
        frame: 18,
        tics: 5,
        action: None,
        nextstate: S_SPOS_XDIE8,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_XDIE7
    State {
        sprite: SPR_SPOS,
        frame: 19,
        tics: 5,
        action: None,
        nextstate: S_SPOS_XDIE9,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_XDIE8
    State {
        sprite: SPR_SPOS,
        frame: 20,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_XDIE9
    State {
        sprite: SPR_SPOS,
        frame: 11,
        tics: 5,
        action: None,
        nextstate: S_SPOS_RAISE2,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_RAISE1
    State {
        sprite: SPR_SPOS,
        frame: 10,
        tics: 5,
        action: None,
        nextstate: S_SPOS_RAISE3,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_RAISE2
    State {
        sprite: SPR_SPOS,
        frame: 9,
        tics: 5,
        action: None,
        nextstate: S_SPOS_RAISE4,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_RAISE3
    State {
        sprite: SPR_SPOS,
        frame: 8,
        tics: 5,
        action: None,
        nextstate: S_SPOS_RAISE5,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_RAISE4
    State {
        sprite: SPR_SPOS,
        frame: 7,
        tics: 5,
        action: None,
        nextstate: S_SPOS_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SPOS_RAISE5
    State {
        sprite: SPR_VILE,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_VILE_STND2,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_STND
    State {
        sprite: SPR_VILE,
        frame: 1,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_VILE_STND,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_STND2
    State {
        sprite: SPR_VILE,
        frame: 0,
        tics: 2,
        action: Some(A_VileChase),
        nextstate: S_VILE_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_RUN1
    State {
        sprite: SPR_VILE,
        frame: 0,
        tics: 2,
        action: Some(A_VileChase),
        nextstate: S_VILE_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_RUN2
    State {
        sprite: SPR_VILE,
        frame: 1,
        tics: 2,
        action: Some(A_VileChase),
        nextstate: S_VILE_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_RUN3
    State {
        sprite: SPR_VILE,
        frame: 1,
        tics: 2,
        action: Some(A_VileChase),
        nextstate: S_VILE_RUN5,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_RUN4
    State {
        sprite: SPR_VILE,
        frame: 2,
        tics: 2,
        action: Some(A_VileChase),
        nextstate: S_VILE_RUN6,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_RUN5
    State {
        sprite: SPR_VILE,
        frame: 2,
        tics: 2,
        action: Some(A_VileChase),
        nextstate: S_VILE_RUN7,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_RUN6
    State {
        sprite: SPR_VILE,
        frame: 3,
        tics: 2,
        action: Some(A_VileChase),
        nextstate: S_VILE_RUN8,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_RUN7
    State {
        sprite: SPR_VILE,
        frame: 3,
        tics: 2,
        action: Some(A_VileChase),
        nextstate: S_VILE_RUN9,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_RUN8
    State {
        sprite: SPR_VILE,
        frame: 4,
        tics: 2,
        action: Some(A_VileChase),
        nextstate: S_VILE_RUN10,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_RUN9
    State {
        sprite: SPR_VILE,
        frame: 4,
        tics: 2,
        action: Some(A_VileChase),
        nextstate: S_VILE_RUN11,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_RUN10
    State {
        sprite: SPR_VILE,
        frame: 5,
        tics: 2,
        action: Some(A_VileChase),
        nextstate: S_VILE_RUN12,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_RUN11
    State {
        sprite: SPR_VILE,
        frame: 5,
        tics: 2,
        action: Some(A_VileChase),
        nextstate: S_VILE_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_RUN12
    State {
        sprite: SPR_VILE,
        frame: 32774,
        tics: 0,
        action: Some(A_VileStart),
        nextstate: S_VILE_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_ATK1
    State {
        sprite: SPR_VILE,
        frame: 32774,
        tics: 10,
        action: Some(A_FaceTarget),
        nextstate: S_VILE_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_ATK2
    State {
        sprite: SPR_VILE,
        frame: 32775,
        tics: 8,
        action: Some(A_VileTarget),
        nextstate: S_VILE_ATK4,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_ATK3
    State {
        sprite: SPR_VILE,
        frame: 32776,
        tics: 8,
        action: Some(A_FaceTarget),
        nextstate: S_VILE_ATK5,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_ATK4
    State {
        sprite: SPR_VILE,
        frame: 32777,
        tics: 8,
        action: Some(A_FaceTarget),
        nextstate: S_VILE_ATK6,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_ATK5
    State {
        sprite: SPR_VILE,
        frame: 32778,
        tics: 8,
        action: Some(A_FaceTarget),
        nextstate: S_VILE_ATK7,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_ATK6
    State {
        sprite: SPR_VILE,
        frame: 32779,
        tics: 8,
        action: Some(A_FaceTarget),
        nextstate: S_VILE_ATK8,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_ATK7
    State {
        sprite: SPR_VILE,
        frame: 32780,
        tics: 8,
        action: Some(A_FaceTarget),
        nextstate: S_VILE_ATK9,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_ATK8
    State {
        sprite: SPR_VILE,
        frame: 32781,
        tics: 8,
        action: Some(A_FaceTarget),
        nextstate: S_VILE_ATK10,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_ATK9
    State {
        sprite: SPR_VILE,
        frame: 32782,
        tics: 8,
        action: Some(A_VileAttack),
        nextstate: S_VILE_ATK11,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_ATK10
    State {
        sprite: SPR_VILE,
        frame: 32783,
        tics: 20,
        action: None,
        nextstate: S_VILE_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_ATK11
    State {
        sprite: SPR_VILE,
        frame: 32794,
        tics: 10,
        action: None,
        nextstate: S_VILE_HEAL2,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_HEAL1
    State {
        sprite: SPR_VILE,
        frame: 32795,
        tics: 10,
        action: None,
        nextstate: S_VILE_HEAL3,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_HEAL2
    State {
        sprite: SPR_VILE,
        frame: 32796,
        tics: 10,
        action: None,
        nextstate: S_VILE_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_HEAL3
    State {
        sprite: SPR_VILE,
        frame: 16,
        tics: 5,
        action: None,
        nextstate: S_VILE_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_PAIN
    State {
        sprite: SPR_VILE,
        frame: 16,
        tics: 5,
        action: Some(A_Pain),
        nextstate: S_VILE_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_PAIN2
    State {
        sprite: SPR_VILE,
        frame: 16,
        tics: 7,
        action: None,
        nextstate: S_VILE_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_DIE1
    State {
        sprite: SPR_VILE,
        frame: 17,
        tics: 7,
        action: Some(A_Scream),
        nextstate: S_VILE_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_DIE2
    State {
        sprite: SPR_VILE,
        frame: 18,
        tics: 7,
        action: Some(A_Fall),
        nextstate: S_VILE_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_DIE3
    State {
        sprite: SPR_VILE,
        frame: 19,
        tics: 7,
        action: None,
        nextstate: S_VILE_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_DIE4
    State {
        sprite: SPR_VILE,
        frame: 20,
        tics: 7,
        action: None,
        nextstate: S_VILE_DIE6,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_DIE5
    State {
        sprite: SPR_VILE,
        frame: 21,
        tics: 7,
        action: None,
        nextstate: S_VILE_DIE7,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_DIE6
    State {
        sprite: SPR_VILE,
        frame: 22,
        tics: 7,
        action: None,
        nextstate: S_VILE_DIE8,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_DIE7
    State {
        sprite: SPR_VILE,
        frame: 23,
        tics: 5,
        action: None,
        nextstate: S_VILE_DIE9,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_DIE8
    State {
        sprite: SPR_VILE,
        frame: 24,
        tics: 5,
        action: None,
        nextstate: S_VILE_DIE10,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_DIE9
    State {
        sprite: SPR_VILE,
        frame: 25,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_VILE_DIE10
    State {
        sprite: SPR_FIRE,
        frame: 32768,
        tics: 2,
        action: Some(A_StartFire),
        nextstate: S_FIRE2,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE1
    State {
        sprite: SPR_FIRE,
        frame: 32769,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE3,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE2
    State {
        sprite: SPR_FIRE,
        frame: 32768,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE4,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE3
    State {
        sprite: SPR_FIRE,
        frame: 32769,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE5,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE4
    State {
        sprite: SPR_FIRE,
        frame: 32770,
        tics: 2,
        action: Some(A_FireCrackle),
        nextstate: S_FIRE6,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE5
    State {
        sprite: SPR_FIRE,
        frame: 32769,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE7,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE6
    State {
        sprite: SPR_FIRE,
        frame: 32770,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE8,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE7
    State {
        sprite: SPR_FIRE,
        frame: 32769,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE9,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE8
    State {
        sprite: SPR_FIRE,
        frame: 32770,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE10,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE9
    State {
        sprite: SPR_FIRE,
        frame: 32771,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE11,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE10
    State {
        sprite: SPR_FIRE,
        frame: 32770,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE12,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE11
    State {
        sprite: SPR_FIRE,
        frame: 32771,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE13,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE12
    State {
        sprite: SPR_FIRE,
        frame: 32770,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE14,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE13
    State {
        sprite: SPR_FIRE,
        frame: 32771,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE15,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE14
    State {
        sprite: SPR_FIRE,
        frame: 32772,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE16,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE15
    State {
        sprite: SPR_FIRE,
        frame: 32771,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE17,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE16
    State {
        sprite: SPR_FIRE,
        frame: 32772,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE18,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE17
    State {
        sprite: SPR_FIRE,
        frame: 32771,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE19,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE18
    State {
        sprite: SPR_FIRE,
        frame: 32772,
        tics: 2,
        action: Some(A_FireCrackle),
        nextstate: S_FIRE20,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE19
    State {
        sprite: SPR_FIRE,
        frame: 32773,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE21,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE20
    State {
        sprite: SPR_FIRE,
        frame: 32772,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE22,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE21
    State {
        sprite: SPR_FIRE,
        frame: 32773,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE23,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE22
    State {
        sprite: SPR_FIRE,
        frame: 32772,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE24,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE23
    State {
        sprite: SPR_FIRE,
        frame: 32773,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE25,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE24
    State {
        sprite: SPR_FIRE,
        frame: 32774,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE26,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE25
    State {
        sprite: SPR_FIRE,
        frame: 32775,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE27,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE26
    State {
        sprite: SPR_FIRE,
        frame: 32774,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE28,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE27
    State {
        sprite: SPR_FIRE,
        frame: 32775,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE29,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE28
    State {
        sprite: SPR_FIRE,
        frame: 32774,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_FIRE30,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE29
    State {
        sprite: SPR_FIRE,
        frame: 32775,
        tics: 2,
        action: Some(A_Fire),
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_FIRE30
    State {
        sprite: SPR_PUFF,
        frame: 1,
        tics: 4,
        action: None,
        nextstate: S_SMOKE2,
        misc1: 0,
        misc2: 0,
    }, // S_SMOKE1
    State {
        sprite: SPR_PUFF,
        frame: 2,
        tics: 4,
        action: None,
        nextstate: S_SMOKE3,
        misc1: 0,
        misc2: 0,
    }, // S_SMOKE2
    State {
        sprite: SPR_PUFF,
        frame: 1,
        tics: 4,
        action: None,
        nextstate: S_SMOKE4,
        misc1: 0,
        misc2: 0,
    }, // S_SMOKE3
    State {
        sprite: SPR_PUFF,
        frame: 2,
        tics: 4,
        action: None,
        nextstate: S_SMOKE5,
        misc1: 0,
        misc2: 0,
    }, // S_SMOKE4
    State {
        sprite: SPR_PUFF,
        frame: 3,
        tics: 4,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SMOKE5
    State {
        sprite: SPR_FATB,
        frame: 32768,
        tics: 2,
        action: Some(A_Tracer),
        nextstate: S_TRACER2,
        misc1: 0,
        misc2: 0,
    }, // S_TRACER
    State {
        sprite: SPR_FATB,
        frame: 32769,
        tics: 2,
        action: Some(A_Tracer),
        nextstate: S_TRACER,
        misc1: 0,
        misc2: 0,
    }, // S_TRACER2
    State {
        sprite: SPR_FBXP,
        frame: 32768,
        tics: 8,
        action: None,
        nextstate: S_TRACEEXP2,
        misc1: 0,
        misc2: 0,
    }, // S_TRACEEXP1
    State {
        sprite: SPR_FBXP,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_TRACEEXP3,
        misc1: 0,
        misc2: 0,
    }, // S_TRACEEXP2
    State {
        sprite: SPR_FBXP,
        frame: 32770,
        tics: 4,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_TRACEEXP3
    State {
        sprite: SPR_SKEL,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_SKEL_STND2,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_STND
    State {
        sprite: SPR_SKEL,
        frame: 1,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_SKEL_STND,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_STND2
    State {
        sprite: SPR_SKEL,
        frame: 0,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SKEL_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RUN1
    State {
        sprite: SPR_SKEL,
        frame: 0,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SKEL_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RUN2
    State {
        sprite: SPR_SKEL,
        frame: 1,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SKEL_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RUN3
    State {
        sprite: SPR_SKEL,
        frame: 1,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SKEL_RUN5,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RUN4
    State {
        sprite: SPR_SKEL,
        frame: 2,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SKEL_RUN6,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RUN5
    State {
        sprite: SPR_SKEL,
        frame: 2,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SKEL_RUN7,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RUN6
    State {
        sprite: SPR_SKEL,
        frame: 3,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SKEL_RUN8,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RUN7
    State {
        sprite: SPR_SKEL,
        frame: 3,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SKEL_RUN9,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RUN8
    State {
        sprite: SPR_SKEL,
        frame: 4,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SKEL_RUN10,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RUN9
    State {
        sprite: SPR_SKEL,
        frame: 4,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SKEL_RUN11,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RUN10
    State {
        sprite: SPR_SKEL,
        frame: 5,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SKEL_RUN12,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RUN11
    State {
        sprite: SPR_SKEL,
        frame: 5,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SKEL_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RUN12
    State {
        sprite: SPR_SKEL,
        frame: 6,
        tics: 0,
        action: Some(A_FaceTarget),
        nextstate: S_SKEL_FIST2,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_FIST1
    State {
        sprite: SPR_SKEL,
        frame: 6,
        tics: 6,
        action: Some(A_SkelWhoosh),
        nextstate: S_SKEL_FIST3,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_FIST2
    State {
        sprite: SPR_SKEL,
        frame: 7,
        tics: 6,
        action: Some(A_FaceTarget),
        nextstate: S_SKEL_FIST4,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_FIST3
    State {
        sprite: SPR_SKEL,
        frame: 8,
        tics: 6,
        action: Some(A_SkelFist),
        nextstate: S_SKEL_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_FIST4
    State {
        sprite: SPR_SKEL,
        frame: 32777,
        tics: 0,
        action: Some(A_FaceTarget),
        nextstate: S_SKEL_MISS2,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_MISS1
    State {
        sprite: SPR_SKEL,
        frame: 32777,
        tics: 10,
        action: Some(A_FaceTarget),
        nextstate: S_SKEL_MISS3,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_MISS2
    State {
        sprite: SPR_SKEL,
        frame: 10,
        tics: 10,
        action: Some(A_SkelMissile),
        nextstate: S_SKEL_MISS4,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_MISS3
    State {
        sprite: SPR_SKEL,
        frame: 10,
        tics: 10,
        action: Some(A_FaceTarget),
        nextstate: S_SKEL_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_MISS4
    State {
        sprite: SPR_SKEL,
        frame: 11,
        tics: 5,
        action: None,
        nextstate: S_SKEL_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_PAIN
    State {
        sprite: SPR_SKEL,
        frame: 11,
        tics: 5,
        action: Some(A_Pain),
        nextstate: S_SKEL_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_PAIN2
    State {
        sprite: SPR_SKEL,
        frame: 11,
        tics: 7,
        action: None,
        nextstate: S_SKEL_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_DIE1
    State {
        sprite: SPR_SKEL,
        frame: 12,
        tics: 7,
        action: None,
        nextstate: S_SKEL_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_DIE2
    State {
        sprite: SPR_SKEL,
        frame: 13,
        tics: 7,
        action: Some(A_Scream),
        nextstate: S_SKEL_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_DIE3
    State {
        sprite: SPR_SKEL,
        frame: 14,
        tics: 7,
        action: Some(A_Fall),
        nextstate: S_SKEL_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_DIE4
    State {
        sprite: SPR_SKEL,
        frame: 15,
        tics: 7,
        action: None,
        nextstate: S_SKEL_DIE6,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_DIE5
    State {
        sprite: SPR_SKEL,
        frame: 16,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_DIE6
    State {
        sprite: SPR_SKEL,
        frame: 16,
        tics: 5,
        action: None,
        nextstate: S_SKEL_RAISE2,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RAISE1
    State {
        sprite: SPR_SKEL,
        frame: 15,
        tics: 5,
        action: None,
        nextstate: S_SKEL_RAISE3,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RAISE2
    State {
        sprite: SPR_SKEL,
        frame: 14,
        tics: 5,
        action: None,
        nextstate: S_SKEL_RAISE4,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RAISE3
    State {
        sprite: SPR_SKEL,
        frame: 13,
        tics: 5,
        action: None,
        nextstate: S_SKEL_RAISE5,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RAISE4
    State {
        sprite: SPR_SKEL,
        frame: 12,
        tics: 5,
        action: None,
        nextstate: S_SKEL_RAISE6,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RAISE5
    State {
        sprite: SPR_SKEL,
        frame: 11,
        tics: 5,
        action: None,
        nextstate: S_SKEL_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SKEL_RAISE6
    State {
        sprite: SPR_MANF,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_FATSHOT2,
        misc1: 0,
        misc2: 0,
    }, // S_FATSHOT1
    State {
        sprite: SPR_MANF,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_FATSHOT1,
        misc1: 0,
        misc2: 0,
    }, // S_FATSHOT2
    State {
        sprite: SPR_MISL,
        frame: 32769,
        tics: 8,
        action: None,
        nextstate: S_FATSHOTX2,
        misc1: 0,
        misc2: 0,
    }, // S_FATSHOTX1
    State {
        sprite: SPR_MISL,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_FATSHOTX3,
        misc1: 0,
        misc2: 0,
    }, // S_FATSHOTX2
    State {
        sprite: SPR_MISL,
        frame: 32771,
        tics: 4,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_FATSHOTX3
    State {
        sprite: SPR_FATT,
        frame: 0,
        tics: 15,
        action: Some(A_Look),
        nextstate: S_FATT_STND2,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_STND
    State {
        sprite: SPR_FATT,
        frame: 1,
        tics: 15,
        action: Some(A_Look),
        nextstate: S_FATT_STND,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_STND2
    State {
        sprite: SPR_FATT,
        frame: 0,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_FATT_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RUN1
    State {
        sprite: SPR_FATT,
        frame: 0,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_FATT_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RUN2
    State {
        sprite: SPR_FATT,
        frame: 1,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_FATT_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RUN3
    State {
        sprite: SPR_FATT,
        frame: 1,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_FATT_RUN5,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RUN4
    State {
        sprite: SPR_FATT,
        frame: 2,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_FATT_RUN6,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RUN5
    State {
        sprite: SPR_FATT,
        frame: 2,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_FATT_RUN7,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RUN6
    State {
        sprite: SPR_FATT,
        frame: 3,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_FATT_RUN8,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RUN7
    State {
        sprite: SPR_FATT,
        frame: 3,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_FATT_RUN9,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RUN8
    State {
        sprite: SPR_FATT,
        frame: 4,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_FATT_RUN10,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RUN9
    State {
        sprite: SPR_FATT,
        frame: 4,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_FATT_RUN11,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RUN10
    State {
        sprite: SPR_FATT,
        frame: 5,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_FATT_RUN12,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RUN11
    State {
        sprite: SPR_FATT,
        frame: 5,
        tics: 4,
        action: Some(A_Chase),
        nextstate: S_FATT_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RUN12
    State {
        sprite: SPR_FATT,
        frame: 6,
        tics: 20,
        action: Some(A_FatRaise),
        nextstate: S_FATT_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_ATK1
    State {
        sprite: SPR_FATT,
        frame: 32775,
        tics: 10,
        action: Some(A_FatAttack1),
        nextstate: S_FATT_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_ATK2
    State {
        sprite: SPR_FATT,
        frame: 8,
        tics: 5,
        action: Some(A_FaceTarget),
        nextstate: S_FATT_ATK4,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_ATK3
    State {
        sprite: SPR_FATT,
        frame: 6,
        tics: 5,
        action: Some(A_FaceTarget),
        nextstate: S_FATT_ATK5,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_ATK4
    State {
        sprite: SPR_FATT,
        frame: 32775,
        tics: 10,
        action: Some(A_FatAttack2),
        nextstate: S_FATT_ATK6,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_ATK5
    State {
        sprite: SPR_FATT,
        frame: 8,
        tics: 5,
        action: Some(A_FaceTarget),
        nextstate: S_FATT_ATK7,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_ATK6
    State {
        sprite: SPR_FATT,
        frame: 6,
        tics: 5,
        action: Some(A_FaceTarget),
        nextstate: S_FATT_ATK8,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_ATK7
    State {
        sprite: SPR_FATT,
        frame: 32775,
        tics: 10,
        action: Some(A_FatAttack3),
        nextstate: S_FATT_ATK9,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_ATK8
    State {
        sprite: SPR_FATT,
        frame: 8,
        tics: 5,
        action: Some(A_FaceTarget),
        nextstate: S_FATT_ATK10,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_ATK9
    State {
        sprite: SPR_FATT,
        frame: 6,
        tics: 5,
        action: Some(A_FaceTarget),
        nextstate: S_FATT_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_ATK10
    State {
        sprite: SPR_FATT,
        frame: 9,
        tics: 3,
        action: None,
        nextstate: S_FATT_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_PAIN
    State {
        sprite: SPR_FATT,
        frame: 9,
        tics: 3,
        action: Some(A_Pain),
        nextstate: S_FATT_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_PAIN2
    State {
        sprite: SPR_FATT,
        frame: 10,
        tics: 6,
        action: None,
        nextstate: S_FATT_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_DIE1
    State {
        sprite: SPR_FATT,
        frame: 11,
        tics: 6,
        action: Some(A_Scream),
        nextstate: S_FATT_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_DIE2
    State {
        sprite: SPR_FATT,
        frame: 12,
        tics: 6,
        action: Some(A_Fall),
        nextstate: S_FATT_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_DIE3
    State {
        sprite: SPR_FATT,
        frame: 13,
        tics: 6,
        action: None,
        nextstate: S_FATT_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_DIE4
    State {
        sprite: SPR_FATT,
        frame: 14,
        tics: 6,
        action: None,
        nextstate: S_FATT_DIE6,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_DIE5
    State {
        sprite: SPR_FATT,
        frame: 15,
        tics: 6,
        action: None,
        nextstate: S_FATT_DIE7,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_DIE6
    State {
        sprite: SPR_FATT,
        frame: 16,
        tics: 6,
        action: None,
        nextstate: S_FATT_DIE8,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_DIE7
    State {
        sprite: SPR_FATT,
        frame: 17,
        tics: 6,
        action: None,
        nextstate: S_FATT_DIE9,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_DIE8
    State {
        sprite: SPR_FATT,
        frame: 18,
        tics: 6,
        action: None,
        nextstate: S_FATT_DIE10,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_DIE9
    State {
        sprite: SPR_FATT,
        frame: 19,
        tics: -1,
        action: Some(A_BossDeath),
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_DIE10
    State {
        sprite: SPR_FATT,
        frame: 17,
        tics: 5,
        action: None,
        nextstate: S_FATT_RAISE2,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RAISE1
    State {
        sprite: SPR_FATT,
        frame: 16,
        tics: 5,
        action: None,
        nextstate: S_FATT_RAISE3,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RAISE2
    State {
        sprite: SPR_FATT,
        frame: 15,
        tics: 5,
        action: None,
        nextstate: S_FATT_RAISE4,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RAISE3
    State {
        sprite: SPR_FATT,
        frame: 14,
        tics: 5,
        action: None,
        nextstate: S_FATT_RAISE5,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RAISE4
    State {
        sprite: SPR_FATT,
        frame: 13,
        tics: 5,
        action: None,
        nextstate: S_FATT_RAISE6,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RAISE5
    State {
        sprite: SPR_FATT,
        frame: 12,
        tics: 5,
        action: None,
        nextstate: S_FATT_RAISE7,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RAISE6
    State {
        sprite: SPR_FATT,
        frame: 11,
        tics: 5,
        action: None,
        nextstate: S_FATT_RAISE8,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RAISE7
    State {
        sprite: SPR_FATT,
        frame: 10,
        tics: 5,
        action: None,
        nextstate: S_FATT_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_FATT_RAISE8
    State {
        sprite: SPR_CPOS,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_CPOS_STND2,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_STND
    State {
        sprite: SPR_CPOS,
        frame: 1,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_CPOS_STND,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_STND2
    State {
        sprite: SPR_CPOS,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_CPOS_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_RUN1
    State {
        sprite: SPR_CPOS,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_CPOS_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_RUN2
    State {
        sprite: SPR_CPOS,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_CPOS_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_RUN3
    State {
        sprite: SPR_CPOS,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_CPOS_RUN5,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_RUN4
    State {
        sprite: SPR_CPOS,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_CPOS_RUN6,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_RUN5
    State {
        sprite: SPR_CPOS,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_CPOS_RUN7,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_RUN6
    State {
        sprite: SPR_CPOS,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_CPOS_RUN8,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_RUN7
    State {
        sprite: SPR_CPOS,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_CPOS_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_RUN8
    State {
        sprite: SPR_CPOS,
        frame: 4,
        tics: 10,
        action: Some(A_FaceTarget),
        nextstate: S_CPOS_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_ATK1
    State {
        sprite: SPR_CPOS,
        frame: 32773,
        tics: 4,
        action: Some(A_CPosAttack),
        nextstate: S_CPOS_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_ATK2
    State {
        sprite: SPR_CPOS,
        frame: 32772,
        tics: 4,
        action: Some(A_CPosAttack),
        nextstate: S_CPOS_ATK4,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_ATK3
    State {
        sprite: SPR_CPOS,
        frame: 5,
        tics: 1,
        action: Some(A_CPosRefire),
        nextstate: S_CPOS_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_ATK4
    State {
        sprite: SPR_CPOS,
        frame: 6,
        tics: 3,
        action: None,
        nextstate: S_CPOS_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_PAIN
    State {
        sprite: SPR_CPOS,
        frame: 6,
        tics: 3,
        action: Some(A_Pain),
        nextstate: S_CPOS_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_PAIN2
    State {
        sprite: SPR_CPOS,
        frame: 7,
        tics: 5,
        action: None,
        nextstate: S_CPOS_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_DIE1
    State {
        sprite: SPR_CPOS,
        frame: 8,
        tics: 5,
        action: Some(A_Scream),
        nextstate: S_CPOS_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_DIE2
    State {
        sprite: SPR_CPOS,
        frame: 9,
        tics: 5,
        action: Some(A_Fall),
        nextstate: S_CPOS_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_DIE3
    State {
        sprite: SPR_CPOS,
        frame: 10,
        tics: 5,
        action: None,
        nextstate: S_CPOS_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_DIE4
    State {
        sprite: SPR_CPOS,
        frame: 11,
        tics: 5,
        action: None,
        nextstate: S_CPOS_DIE6,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_DIE5
    State {
        sprite: SPR_CPOS,
        frame: 12,
        tics: 5,
        action: None,
        nextstate: S_CPOS_DIE7,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_DIE6
    State {
        sprite: SPR_CPOS,
        frame: 13,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_DIE7
    State {
        sprite: SPR_CPOS,
        frame: 14,
        tics: 5,
        action: None,
        nextstate: S_CPOS_XDIE2,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_XDIE1
    State {
        sprite: SPR_CPOS,
        frame: 15,
        tics: 5,
        action: Some(A_XScream),
        nextstate: S_CPOS_XDIE3,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_XDIE2
    State {
        sprite: SPR_CPOS,
        frame: 16,
        tics: 5,
        action: Some(A_Fall),
        nextstate: S_CPOS_XDIE4,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_XDIE3
    State {
        sprite: SPR_CPOS,
        frame: 17,
        tics: 5,
        action: None,
        nextstate: S_CPOS_XDIE5,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_XDIE4
    State {
        sprite: SPR_CPOS,
        frame: 18,
        tics: 5,
        action: None,
        nextstate: S_CPOS_XDIE6,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_XDIE5
    State {
        sprite: SPR_CPOS,
        frame: 19,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_XDIE6
    State {
        sprite: SPR_CPOS,
        frame: 13,
        tics: 5,
        action: None,
        nextstate: S_CPOS_RAISE2,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_RAISE1
    State {
        sprite: SPR_CPOS,
        frame: 12,
        tics: 5,
        action: None,
        nextstate: S_CPOS_RAISE3,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_RAISE2
    State {
        sprite: SPR_CPOS,
        frame: 11,
        tics: 5,
        action: None,
        nextstate: S_CPOS_RAISE4,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_RAISE3
    State {
        sprite: SPR_CPOS,
        frame: 10,
        tics: 5,
        action: None,
        nextstate: S_CPOS_RAISE5,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_RAISE4
    State {
        sprite: SPR_CPOS,
        frame: 9,
        tics: 5,
        action: None,
        nextstate: S_CPOS_RAISE6,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_RAISE5
    State {
        sprite: SPR_CPOS,
        frame: 8,
        tics: 5,
        action: None,
        nextstate: S_CPOS_RAISE7,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_RAISE6
    State {
        sprite: SPR_CPOS,
        frame: 7,
        tics: 5,
        action: None,
        nextstate: S_CPOS_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_CPOS_RAISE7
    State {
        sprite: SPR_TROO,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_TROO_STND2,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_STND
    State {
        sprite: SPR_TROO,
        frame: 1,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_TROO_STND,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_STND2
    State {
        sprite: SPR_TROO,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_TROO_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_RUN1
    State {
        sprite: SPR_TROO,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_TROO_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_RUN2
    State {
        sprite: SPR_TROO,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_TROO_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_RUN3
    State {
        sprite: SPR_TROO,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_TROO_RUN5,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_RUN4
    State {
        sprite: SPR_TROO,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_TROO_RUN6,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_RUN5
    State {
        sprite: SPR_TROO,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_TROO_RUN7,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_RUN6
    State {
        sprite: SPR_TROO,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_TROO_RUN8,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_RUN7
    State {
        sprite: SPR_TROO,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_TROO_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_RUN8
    State {
        sprite: SPR_TROO,
        frame: 4,
        tics: 8,
        action: Some(A_FaceTarget),
        nextstate: S_TROO_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_ATK1
    State {
        sprite: SPR_TROO,
        frame: 5,
        tics: 8,
        action: Some(A_FaceTarget),
        nextstate: S_TROO_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_ATK2
    State {
        sprite: SPR_TROO,
        frame: 6,
        tics: 6,
        action: Some(A_TroopAttack),
        nextstate: S_TROO_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_ATK3
    State {
        sprite: SPR_TROO,
        frame: 7,
        tics: 2,
        action: None,
        nextstate: S_TROO_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_PAIN
    State {
        sprite: SPR_TROO,
        frame: 7,
        tics: 2,
        action: Some(A_Pain),
        nextstate: S_TROO_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_PAIN2
    State {
        sprite: SPR_TROO,
        frame: 8,
        tics: 8,
        action: None,
        nextstate: S_TROO_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_DIE1
    State {
        sprite: SPR_TROO,
        frame: 9,
        tics: 8,
        action: Some(A_Scream),
        nextstate: S_TROO_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_DIE2
    State {
        sprite: SPR_TROO,
        frame: 10,
        tics: 6,
        action: None,
        nextstate: S_TROO_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_DIE3
    State {
        sprite: SPR_TROO,
        frame: 11,
        tics: 6,
        action: Some(A_Fall),
        nextstate: S_TROO_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_DIE4
    State {
        sprite: SPR_TROO,
        frame: 12,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_DIE5
    State {
        sprite: SPR_TROO,
        frame: 13,
        tics: 5,
        action: None,
        nextstate: S_TROO_XDIE2,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_XDIE1
    State {
        sprite: SPR_TROO,
        frame: 14,
        tics: 5,
        action: Some(A_XScream),
        nextstate: S_TROO_XDIE3,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_XDIE2
    State {
        sprite: SPR_TROO,
        frame: 15,
        tics: 5,
        action: None,
        nextstate: S_TROO_XDIE4,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_XDIE3
    State {
        sprite: SPR_TROO,
        frame: 16,
        tics: 5,
        action: Some(A_Fall),
        nextstate: S_TROO_XDIE5,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_XDIE4
    State {
        sprite: SPR_TROO,
        frame: 17,
        tics: 5,
        action: None,
        nextstate: S_TROO_XDIE6,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_XDIE5
    State {
        sprite: SPR_TROO,
        frame: 18,
        tics: 5,
        action: None,
        nextstate: S_TROO_XDIE7,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_XDIE6
    State {
        sprite: SPR_TROO,
        frame: 19,
        tics: 5,
        action: None,
        nextstate: S_TROO_XDIE8,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_XDIE7
    State {
        sprite: SPR_TROO,
        frame: 20,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_XDIE8
    State {
        sprite: SPR_TROO,
        frame: 12,
        tics: 8,
        action: None,
        nextstate: S_TROO_RAISE2,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_RAISE1
    State {
        sprite: SPR_TROO,
        frame: 11,
        tics: 8,
        action: None,
        nextstate: S_TROO_RAISE3,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_RAISE2
    State {
        sprite: SPR_TROO,
        frame: 10,
        tics: 6,
        action: None,
        nextstate: S_TROO_RAISE4,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_RAISE3
    State {
        sprite: SPR_TROO,
        frame: 9,
        tics: 6,
        action: None,
        nextstate: S_TROO_RAISE5,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_RAISE4
    State {
        sprite: SPR_TROO,
        frame: 8,
        tics: 6,
        action: None,
        nextstate: S_TROO_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_TROO_RAISE5
    State {
        sprite: SPR_SARG,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_SARG_STND2,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_STND
    State {
        sprite: SPR_SARG,
        frame: 1,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_SARG_STND,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_STND2
    State {
        sprite: SPR_SARG,
        frame: 0,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SARG_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_RUN1
    State {
        sprite: SPR_SARG,
        frame: 0,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SARG_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_RUN2
    State {
        sprite: SPR_SARG,
        frame: 1,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SARG_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_RUN3
    State {
        sprite: SPR_SARG,
        frame: 1,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SARG_RUN5,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_RUN4
    State {
        sprite: SPR_SARG,
        frame: 2,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SARG_RUN6,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_RUN5
    State {
        sprite: SPR_SARG,
        frame: 2,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SARG_RUN7,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_RUN6
    State {
        sprite: SPR_SARG,
        frame: 3,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SARG_RUN8,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_RUN7
    State {
        sprite: SPR_SARG,
        frame: 3,
        tics: 2,
        action: Some(A_Chase),
        nextstate: S_SARG_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_RUN8
    State {
        sprite: SPR_SARG,
        frame: 4,
        tics: 8,
        action: Some(A_FaceTarget),
        nextstate: S_SARG_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_ATK1
    State {
        sprite: SPR_SARG,
        frame: 5,
        tics: 8,
        action: Some(A_FaceTarget),
        nextstate: S_SARG_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_ATK2
    State {
        sprite: SPR_SARG,
        frame: 6,
        tics: 8,
        action: Some(A_SargAttack),
        nextstate: S_SARG_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_ATK3
    State {
        sprite: SPR_SARG,
        frame: 7,
        tics: 2,
        action: None,
        nextstate: S_SARG_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_PAIN
    State {
        sprite: SPR_SARG,
        frame: 7,
        tics: 2,
        action: Some(A_Pain),
        nextstate: S_SARG_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_PAIN2
    State {
        sprite: SPR_SARG,
        frame: 8,
        tics: 8,
        action: None,
        nextstate: S_SARG_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_DIE1
    State {
        sprite: SPR_SARG,
        frame: 9,
        tics: 8,
        action: Some(A_Scream),
        nextstate: S_SARG_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_DIE2
    State {
        sprite: SPR_SARG,
        frame: 10,
        tics: 4,
        action: None,
        nextstate: S_SARG_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_DIE3
    State {
        sprite: SPR_SARG,
        frame: 11,
        tics: 4,
        action: Some(A_Fall),
        nextstate: S_SARG_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_DIE4
    State {
        sprite: SPR_SARG,
        frame: 12,
        tics: 4,
        action: None,
        nextstate: S_SARG_DIE6,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_DIE5
    State {
        sprite: SPR_SARG,
        frame: 13,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_DIE6
    State {
        sprite: SPR_SARG,
        frame: 13,
        tics: 5,
        action: None,
        nextstate: S_SARG_RAISE2,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_RAISE1
    State {
        sprite: SPR_SARG,
        frame: 12,
        tics: 5,
        action: None,
        nextstate: S_SARG_RAISE3,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_RAISE2
    State {
        sprite: SPR_SARG,
        frame: 11,
        tics: 5,
        action: None,
        nextstate: S_SARG_RAISE4,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_RAISE3
    State {
        sprite: SPR_SARG,
        frame: 10,
        tics: 5,
        action: None,
        nextstate: S_SARG_RAISE5,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_RAISE4
    State {
        sprite: SPR_SARG,
        frame: 9,
        tics: 5,
        action: None,
        nextstate: S_SARG_RAISE6,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_RAISE5
    State {
        sprite: SPR_SARG,
        frame: 8,
        tics: 5,
        action: None,
        nextstate: S_SARG_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SARG_RAISE6
    State {
        sprite: SPR_HEAD,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_HEAD_STND,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_STND
    State {
        sprite: SPR_HEAD,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_HEAD_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_RUN1
    State {
        sprite: SPR_HEAD,
        frame: 1,
        tics: 5,
        action: Some(A_FaceTarget),
        nextstate: S_HEAD_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_ATK1
    State {
        sprite: SPR_HEAD,
        frame: 2,
        tics: 5,
        action: Some(A_FaceTarget),
        nextstate: S_HEAD_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_ATK2
    State {
        sprite: SPR_HEAD,
        frame: 32771,
        tics: 5,
        action: Some(A_HeadAttack),
        nextstate: S_HEAD_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_ATK3
    State {
        sprite: SPR_HEAD,
        frame: 4,
        tics: 3,
        action: None,
        nextstate: S_HEAD_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_PAIN
    State {
        sprite: SPR_HEAD,
        frame: 4,
        tics: 3,
        action: Some(A_Pain),
        nextstate: S_HEAD_PAIN3,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_PAIN2
    State {
        sprite: SPR_HEAD,
        frame: 5,
        tics: 6,
        action: None,
        nextstate: S_HEAD_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_PAIN3
    State {
        sprite: SPR_HEAD,
        frame: 6,
        tics: 8,
        action: None,
        nextstate: S_HEAD_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_DIE1
    State {
        sprite: SPR_HEAD,
        frame: 7,
        tics: 8,
        action: Some(A_Scream),
        nextstate: S_HEAD_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_DIE2
    State {
        sprite: SPR_HEAD,
        frame: 8,
        tics: 8,
        action: None,
        nextstate: S_HEAD_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_DIE3
    State {
        sprite: SPR_HEAD,
        frame: 9,
        tics: 8,
        action: None,
        nextstate: S_HEAD_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_DIE4
    State {
        sprite: SPR_HEAD,
        frame: 10,
        tics: 8,
        action: Some(A_Fall),
        nextstate: S_HEAD_DIE6,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_DIE5
    State {
        sprite: SPR_HEAD,
        frame: 11,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_DIE6
    State {
        sprite: SPR_HEAD,
        frame: 11,
        tics: 8,
        action: None,
        nextstate: S_HEAD_RAISE2,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_RAISE1
    State {
        sprite: SPR_HEAD,
        frame: 10,
        tics: 8,
        action: None,
        nextstate: S_HEAD_RAISE3,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_RAISE2
    State {
        sprite: SPR_HEAD,
        frame: 9,
        tics: 8,
        action: None,
        nextstate: S_HEAD_RAISE4,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_RAISE3
    State {
        sprite: SPR_HEAD,
        frame: 8,
        tics: 8,
        action: None,
        nextstate: S_HEAD_RAISE5,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_RAISE4
    State {
        sprite: SPR_HEAD,
        frame: 7,
        tics: 8,
        action: None,
        nextstate: S_HEAD_RAISE6,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_RAISE5
    State {
        sprite: SPR_HEAD,
        frame: 6,
        tics: 8,
        action: None,
        nextstate: S_HEAD_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_HEAD_RAISE6
    State {
        sprite: SPR_BAL7,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_BRBALL2,
        misc1: 0,
        misc2: 0,
    }, // S_BRBALL1
    State {
        sprite: SPR_BAL7,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_BRBALL1,
        misc1: 0,
        misc2: 0,
    }, // S_BRBALL2
    State {
        sprite: SPR_BAL7,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_BRBALLX2,
        misc1: 0,
        misc2: 0,
    }, // S_BRBALLX1
    State {
        sprite: SPR_BAL7,
        frame: 32771,
        tics: 6,
        action: None,
        nextstate: S_BRBALLX3,
        misc1: 0,
        misc2: 0,
    }, // S_BRBALLX2
    State {
        sprite: SPR_BAL7,
        frame: 32772,
        tics: 6,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BRBALLX3
    State {
        sprite: SPR_BOSS,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_BOSS_STND2,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_STND
    State {
        sprite: SPR_BOSS,
        frame: 1,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_BOSS_STND,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_STND2
    State {
        sprite: SPR_BOSS,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOSS_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_RUN1
    State {
        sprite: SPR_BOSS,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOSS_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_RUN2
    State {
        sprite: SPR_BOSS,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOSS_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_RUN3
    State {
        sprite: SPR_BOSS,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOSS_RUN5,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_RUN4
    State {
        sprite: SPR_BOSS,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOSS_RUN6,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_RUN5
    State {
        sprite: SPR_BOSS,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOSS_RUN7,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_RUN6
    State {
        sprite: SPR_BOSS,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOSS_RUN8,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_RUN7
    State {
        sprite: SPR_BOSS,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOSS_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_RUN8
    State {
        sprite: SPR_BOSS,
        frame: 4,
        tics: 8,
        action: Some(A_FaceTarget),
        nextstate: S_BOSS_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_ATK1
    State {
        sprite: SPR_BOSS,
        frame: 5,
        tics: 8,
        action: Some(A_FaceTarget),
        nextstate: S_BOSS_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_ATK2
    State {
        sprite: SPR_BOSS,
        frame: 6,
        tics: 8,
        action: Some(A_BruisAttack),
        nextstate: S_BOSS_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_ATK3
    State {
        sprite: SPR_BOSS,
        frame: 7,
        tics: 2,
        action: None,
        nextstate: S_BOSS_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_PAIN
    State {
        sprite: SPR_BOSS,
        frame: 7,
        tics: 2,
        action: Some(A_Pain),
        nextstate: S_BOSS_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_PAIN2
    State {
        sprite: SPR_BOSS,
        frame: 8,
        tics: 8,
        action: None,
        nextstate: S_BOSS_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_DIE1
    State {
        sprite: SPR_BOSS,
        frame: 9,
        tics: 8,
        action: Some(A_Scream),
        nextstate: S_BOSS_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_DIE2
    State {
        sprite: SPR_BOSS,
        frame: 10,
        tics: 8,
        action: None,
        nextstate: S_BOSS_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_DIE3
    State {
        sprite: SPR_BOSS,
        frame: 11,
        tics: 8,
        action: Some(A_Fall),
        nextstate: S_BOSS_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_DIE4
    State {
        sprite: SPR_BOSS,
        frame: 12,
        tics: 8,
        action: None,
        nextstate: S_BOSS_DIE6,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_DIE5
    State {
        sprite: SPR_BOSS,
        frame: 13,
        tics: 8,
        action: None,
        nextstate: S_BOSS_DIE7,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_DIE6
    State {
        sprite: SPR_BOSS,
        frame: 14,
        tics: -1,
        action: Some(A_BossDeath),
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_DIE7
    State {
        sprite: SPR_BOSS,
        frame: 14,
        tics: 8,
        action: None,
        nextstate: S_BOSS_RAISE2,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_RAISE1
    State {
        sprite: SPR_BOSS,
        frame: 13,
        tics: 8,
        action: None,
        nextstate: S_BOSS_RAISE3,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_RAISE2
    State {
        sprite: SPR_BOSS,
        frame: 12,
        tics: 8,
        action: None,
        nextstate: S_BOSS_RAISE4,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_RAISE3
    State {
        sprite: SPR_BOSS,
        frame: 11,
        tics: 8,
        action: None,
        nextstate: S_BOSS_RAISE5,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_RAISE4
    State {
        sprite: SPR_BOSS,
        frame: 10,
        tics: 8,
        action: None,
        nextstate: S_BOSS_RAISE6,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_RAISE5
    State {
        sprite: SPR_BOSS,
        frame: 9,
        tics: 8,
        action: None,
        nextstate: S_BOSS_RAISE7,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_RAISE6
    State {
        sprite: SPR_BOSS,
        frame: 8,
        tics: 8,
        action: None,
        nextstate: S_BOSS_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_BOSS_RAISE7
    State {
        sprite: SPR_BOS2,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_BOS2_STND2,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_STND
    State {
        sprite: SPR_BOS2,
        frame: 1,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_BOS2_STND,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_STND2
    State {
        sprite: SPR_BOS2,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOS2_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_RUN1
    State {
        sprite: SPR_BOS2,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOS2_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_RUN2
    State {
        sprite: SPR_BOS2,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOS2_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_RUN3
    State {
        sprite: SPR_BOS2,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOS2_RUN5,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_RUN4
    State {
        sprite: SPR_BOS2,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOS2_RUN6,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_RUN5
    State {
        sprite: SPR_BOS2,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOS2_RUN7,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_RUN6
    State {
        sprite: SPR_BOS2,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOS2_RUN8,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_RUN7
    State {
        sprite: SPR_BOS2,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BOS2_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_RUN8
    State {
        sprite: SPR_BOS2,
        frame: 4,
        tics: 8,
        action: Some(A_FaceTarget),
        nextstate: S_BOS2_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_ATK1
    State {
        sprite: SPR_BOS2,
        frame: 5,
        tics: 8,
        action: Some(A_FaceTarget),
        nextstate: S_BOS2_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_ATK2
    State {
        sprite: SPR_BOS2,
        frame: 6,
        tics: 8,
        action: Some(A_BruisAttack),
        nextstate: S_BOS2_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_ATK3
    State {
        sprite: SPR_BOS2,
        frame: 7,
        tics: 2,
        action: None,
        nextstate: S_BOS2_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_PAIN
    State {
        sprite: SPR_BOS2,
        frame: 7,
        tics: 2,
        action: Some(A_Pain),
        nextstate: S_BOS2_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_PAIN2
    State {
        sprite: SPR_BOS2,
        frame: 8,
        tics: 8,
        action: None,
        nextstate: S_BOS2_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_DIE1
    State {
        sprite: SPR_BOS2,
        frame: 9,
        tics: 8,
        action: Some(A_Scream),
        nextstate: S_BOS2_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_DIE2
    State {
        sprite: SPR_BOS2,
        frame: 10,
        tics: 8,
        action: None,
        nextstate: S_BOS2_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_DIE3
    State {
        sprite: SPR_BOS2,
        frame: 11,
        tics: 8,
        action: Some(A_Fall),
        nextstate: S_BOS2_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_DIE4
    State {
        sprite: SPR_BOS2,
        frame: 12,
        tics: 8,
        action: None,
        nextstate: S_BOS2_DIE6,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_DIE5
    State {
        sprite: SPR_BOS2,
        frame: 13,
        tics: 8,
        action: None,
        nextstate: S_BOS2_DIE7,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_DIE6
    State {
        sprite: SPR_BOS2,
        frame: 14,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_DIE7
    State {
        sprite: SPR_BOS2,
        frame: 14,
        tics: 8,
        action: None,
        nextstate: S_BOS2_RAISE2,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_RAISE1
    State {
        sprite: SPR_BOS2,
        frame: 13,
        tics: 8,
        action: None,
        nextstate: S_BOS2_RAISE3,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_RAISE2
    State {
        sprite: SPR_BOS2,
        frame: 12,
        tics: 8,
        action: None,
        nextstate: S_BOS2_RAISE4,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_RAISE3
    State {
        sprite: SPR_BOS2,
        frame: 11,
        tics: 8,
        action: None,
        nextstate: S_BOS2_RAISE5,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_RAISE4
    State {
        sprite: SPR_BOS2,
        frame: 10,
        tics: 8,
        action: None,
        nextstate: S_BOS2_RAISE6,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_RAISE5
    State {
        sprite: SPR_BOS2,
        frame: 9,
        tics: 8,
        action: None,
        nextstate: S_BOS2_RAISE7,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_RAISE6
    State {
        sprite: SPR_BOS2,
        frame: 8,
        tics: 8,
        action: None,
        nextstate: S_BOS2_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_BOS2_RAISE7
    State {
        sprite: SPR_SKUL,
        frame: 32768,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_SKULL_STND2,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_STND
    State {
        sprite: SPR_SKUL,
        frame: 32769,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_SKULL_STND,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_STND2
    State {
        sprite: SPR_SKUL,
        frame: 32768,
        tics: 6,
        action: Some(A_Chase),
        nextstate: S_SKULL_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_RUN1
    State {
        sprite: SPR_SKUL,
        frame: 32769,
        tics: 6,
        action: Some(A_Chase),
        nextstate: S_SKULL_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_RUN2
    State {
        sprite: SPR_SKUL,
        frame: 32770,
        tics: 10,
        action: Some(A_FaceTarget),
        nextstate: S_SKULL_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_ATK1
    State {
        sprite: SPR_SKUL,
        frame: 32771,
        tics: 4,
        action: Some(A_SkullAttack),
        nextstate: S_SKULL_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_ATK2
    State {
        sprite: SPR_SKUL,
        frame: 32770,
        tics: 4,
        action: None,
        nextstate: S_SKULL_ATK4,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_ATK3
    State {
        sprite: SPR_SKUL,
        frame: 32771,
        tics: 4,
        action: None,
        nextstate: S_SKULL_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_ATK4
    State {
        sprite: SPR_SKUL,
        frame: 32772,
        tics: 3,
        action: None,
        nextstate: S_SKULL_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_PAIN
    State {
        sprite: SPR_SKUL,
        frame: 32772,
        tics: 3,
        action: Some(A_Pain),
        nextstate: S_SKULL_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_PAIN2
    State {
        sprite: SPR_SKUL,
        frame: 32773,
        tics: 6,
        action: None,
        nextstate: S_SKULL_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_DIE1
    State {
        sprite: SPR_SKUL,
        frame: 32774,
        tics: 6,
        action: Some(A_Scream),
        nextstate: S_SKULL_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_DIE2
    State {
        sprite: SPR_SKUL,
        frame: 32775,
        tics: 6,
        action: None,
        nextstate: S_SKULL_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_DIE3
    State {
        sprite: SPR_SKUL,
        frame: 32776,
        tics: 6,
        action: Some(A_Fall),
        nextstate: S_SKULL_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_DIE4
    State {
        sprite: SPR_SKUL,
        frame: 9,
        tics: 6,
        action: None,
        nextstate: S_SKULL_DIE6,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_DIE5
    State {
        sprite: SPR_SKUL,
        frame: 10,
        tics: 6,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SKULL_DIE6
    State {
        sprite: SPR_SPID,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_SPID_STND2,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_STND
    State {
        sprite: SPR_SPID,
        frame: 1,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_SPID_STND,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_STND2
    State {
        sprite: SPR_SPID,
        frame: 0,
        tics: 3,
        action: Some(A_Metal),
        nextstate: S_SPID_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_RUN1
    State {
        sprite: SPR_SPID,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPID_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_RUN2
    State {
        sprite: SPR_SPID,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPID_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_RUN3
    State {
        sprite: SPR_SPID,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPID_RUN5,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_RUN4
    State {
        sprite: SPR_SPID,
        frame: 2,
        tics: 3,
        action: Some(A_Metal),
        nextstate: S_SPID_RUN6,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_RUN5
    State {
        sprite: SPR_SPID,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPID_RUN7,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_RUN6
    State {
        sprite: SPR_SPID,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPID_RUN8,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_RUN7
    State {
        sprite: SPR_SPID,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPID_RUN9,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_RUN8
    State {
        sprite: SPR_SPID,
        frame: 4,
        tics: 3,
        action: Some(A_Metal),
        nextstate: S_SPID_RUN10,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_RUN9
    State {
        sprite: SPR_SPID,
        frame: 4,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPID_RUN11,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_RUN10
    State {
        sprite: SPR_SPID,
        frame: 5,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPID_RUN12,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_RUN11
    State {
        sprite: SPR_SPID,
        frame: 5,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SPID_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_RUN12
    State {
        sprite: SPR_SPID,
        frame: 32768,
        tics: 20,
        action: Some(A_FaceTarget),
        nextstate: S_SPID_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_ATK1
    State {
        sprite: SPR_SPID,
        frame: 32774,
        tics: 4,
        action: Some(A_SPosAttack),
        nextstate: S_SPID_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_ATK2
    State {
        sprite: SPR_SPID,
        frame: 32775,
        tics: 4,
        action: Some(A_SPosAttack),
        nextstate: S_SPID_ATK4,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_ATK3
    State {
        sprite: SPR_SPID,
        frame: 32775,
        tics: 1,
        action: Some(A_SpidRefire),
        nextstate: S_SPID_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_ATK4
    State {
        sprite: SPR_SPID,
        frame: 8,
        tics: 3,
        action: None,
        nextstate: S_SPID_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_PAIN
    State {
        sprite: SPR_SPID,
        frame: 8,
        tics: 3,
        action: Some(A_Pain),
        nextstate: S_SPID_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_PAIN2
    State {
        sprite: SPR_SPID,
        frame: 9,
        tics: 20,
        action: Some(A_Scream),
        nextstate: S_SPID_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_DIE1
    State {
        sprite: SPR_SPID,
        frame: 10,
        tics: 10,
        action: Some(A_Fall),
        nextstate: S_SPID_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_DIE2
    State {
        sprite: SPR_SPID,
        frame: 11,
        tics: 10,
        action: None,
        nextstate: S_SPID_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_DIE3
    State {
        sprite: SPR_SPID,
        frame: 12,
        tics: 10,
        action: None,
        nextstate: S_SPID_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_DIE4
    State {
        sprite: SPR_SPID,
        frame: 13,
        tics: 10,
        action: None,
        nextstate: S_SPID_DIE6,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_DIE5
    State {
        sprite: SPR_SPID,
        frame: 14,
        tics: 10,
        action: None,
        nextstate: S_SPID_DIE7,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_DIE6
    State {
        sprite: SPR_SPID,
        frame: 15,
        tics: 10,
        action: None,
        nextstate: S_SPID_DIE8,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_DIE7
    State {
        sprite: SPR_SPID,
        frame: 16,
        tics: 10,
        action: None,
        nextstate: S_SPID_DIE9,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_DIE8
    State {
        sprite: SPR_SPID,
        frame: 17,
        tics: 10,
        action: None,
        nextstate: S_SPID_DIE10,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_DIE9
    State {
        sprite: SPR_SPID,
        frame: 18,
        tics: 30,
        action: None,
        nextstate: S_SPID_DIE11,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_DIE10
    State {
        sprite: SPR_SPID,
        frame: 18,
        tics: -1,
        action: Some(A_BossDeath),
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SPID_DIE11
    State {
        sprite: SPR_BSPI,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_BSPI_STND2,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_STND
    State {
        sprite: SPR_BSPI,
        frame: 1,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_BSPI_STND,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_STND2
    State {
        sprite: SPR_BSPI,
        frame: 0,
        tics: 20,
        action: None,
        nextstate: S_BSPI_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_SIGHT
    State {
        sprite: SPR_BSPI,
        frame: 0,
        tics: 3,
        action: Some(A_BabyMetal),
        nextstate: S_BSPI_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RUN1
    State {
        sprite: SPR_BSPI,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BSPI_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RUN2
    State {
        sprite: SPR_BSPI,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BSPI_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RUN3
    State {
        sprite: SPR_BSPI,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BSPI_RUN5,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RUN4
    State {
        sprite: SPR_BSPI,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BSPI_RUN6,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RUN5
    State {
        sprite: SPR_BSPI,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BSPI_RUN7,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RUN6
    State {
        sprite: SPR_BSPI,
        frame: 3,
        tics: 3,
        action: Some(A_BabyMetal),
        nextstate: S_BSPI_RUN8,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RUN7
    State {
        sprite: SPR_BSPI,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BSPI_RUN9,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RUN8
    State {
        sprite: SPR_BSPI,
        frame: 4,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BSPI_RUN10,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RUN9
    State {
        sprite: SPR_BSPI,
        frame: 4,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BSPI_RUN11,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RUN10
    State {
        sprite: SPR_BSPI,
        frame: 5,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BSPI_RUN12,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RUN11
    State {
        sprite: SPR_BSPI,
        frame: 5,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_BSPI_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RUN12
    State {
        sprite: SPR_BSPI,
        frame: 32768,
        tics: 20,
        action: Some(A_FaceTarget),
        nextstate: S_BSPI_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_ATK1
    State {
        sprite: SPR_BSPI,
        frame: 32774,
        tics: 4,
        action: Some(A_BspiAttack),
        nextstate: S_BSPI_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_ATK2
    State {
        sprite: SPR_BSPI,
        frame: 32775,
        tics: 4,
        action: None,
        nextstate: S_BSPI_ATK4,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_ATK3
    State {
        sprite: SPR_BSPI,
        frame: 32775,
        tics: 1,
        action: Some(A_SpidRefire),
        nextstate: S_BSPI_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_ATK4
    State {
        sprite: SPR_BSPI,
        frame: 8,
        tics: 3,
        action: None,
        nextstate: S_BSPI_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_PAIN
    State {
        sprite: SPR_BSPI,
        frame: 8,
        tics: 3,
        action: Some(A_Pain),
        nextstate: S_BSPI_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_PAIN2
    State {
        sprite: SPR_BSPI,
        frame: 9,
        tics: 20,
        action: Some(A_Scream),
        nextstate: S_BSPI_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_DIE1
    State {
        sprite: SPR_BSPI,
        frame: 10,
        tics: 7,
        action: Some(A_Fall),
        nextstate: S_BSPI_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_DIE2
    State {
        sprite: SPR_BSPI,
        frame: 11,
        tics: 7,
        action: None,
        nextstate: S_BSPI_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_DIE3
    State {
        sprite: SPR_BSPI,
        frame: 12,
        tics: 7,
        action: None,
        nextstate: S_BSPI_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_DIE4
    State {
        sprite: SPR_BSPI,
        frame: 13,
        tics: 7,
        action: None,
        nextstate: S_BSPI_DIE6,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_DIE5
    State {
        sprite: SPR_BSPI,
        frame: 14,
        tics: 7,
        action: None,
        nextstate: S_BSPI_DIE7,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_DIE6
    State {
        sprite: SPR_BSPI,
        frame: 15,
        tics: -1,
        action: Some(A_BossDeath),
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_DIE7
    State {
        sprite: SPR_BSPI,
        frame: 15,
        tics: 5,
        action: None,
        nextstate: S_BSPI_RAISE2,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RAISE1
    State {
        sprite: SPR_BSPI,
        frame: 14,
        tics: 5,
        action: None,
        nextstate: S_BSPI_RAISE3,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RAISE2
    State {
        sprite: SPR_BSPI,
        frame: 13,
        tics: 5,
        action: None,
        nextstate: S_BSPI_RAISE4,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RAISE3
    State {
        sprite: SPR_BSPI,
        frame: 12,
        tics: 5,
        action: None,
        nextstate: S_BSPI_RAISE5,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RAISE4
    State {
        sprite: SPR_BSPI,
        frame: 11,
        tics: 5,
        action: None,
        nextstate: S_BSPI_RAISE6,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RAISE5
    State {
        sprite: SPR_BSPI,
        frame: 10,
        tics: 5,
        action: None,
        nextstate: S_BSPI_RAISE7,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RAISE6
    State {
        sprite: SPR_BSPI,
        frame: 9,
        tics: 5,
        action: None,
        nextstate: S_BSPI_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_BSPI_RAISE7
    State {
        sprite: SPR_APLS,
        frame: 32768,
        tics: 5,
        action: None,
        nextstate: S_ARACH_PLAZ2,
        misc1: 0,
        misc2: 0,
    }, // S_ARACH_PLAZ
    State {
        sprite: SPR_APLS,
        frame: 32769,
        tics: 5,
        action: None,
        nextstate: S_ARACH_PLAZ,
        misc1: 0,
        misc2: 0,
    }, // S_ARACH_PLAZ2
    State {
        sprite: SPR_APBX,
        frame: 32768,
        tics: 5,
        action: None,
        nextstate: S_ARACH_PLEX2,
        misc1: 0,
        misc2: 0,
    }, // S_ARACH_PLEX
    State {
        sprite: SPR_APBX,
        frame: 32769,
        tics: 5,
        action: None,
        nextstate: S_ARACH_PLEX3,
        misc1: 0,
        misc2: 0,
    }, // S_ARACH_PLEX2
    State {
        sprite: SPR_APBX,
        frame: 32770,
        tics: 5,
        action: None,
        nextstate: S_ARACH_PLEX4,
        misc1: 0,
        misc2: 0,
    }, // S_ARACH_PLEX3
    State {
        sprite: SPR_APBX,
        frame: 32771,
        tics: 5,
        action: None,
        nextstate: S_ARACH_PLEX5,
        misc1: 0,
        misc2: 0,
    }, // S_ARACH_PLEX4
    State {
        sprite: SPR_APBX,
        frame: 32772,
        tics: 5,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_ARACH_PLEX5
    State {
        sprite: SPR_CYBR,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_CYBER_STND2,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_STND
    State {
        sprite: SPR_CYBR,
        frame: 1,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_CYBER_STND,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_STND2
    State {
        sprite: SPR_CYBR,
        frame: 0,
        tics: 3,
        action: Some(A_Hoof),
        nextstate: S_CYBER_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_RUN1
    State {
        sprite: SPR_CYBR,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_CYBER_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_RUN2
    State {
        sprite: SPR_CYBR,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_CYBER_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_RUN3
    State {
        sprite: SPR_CYBR,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_CYBER_RUN5,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_RUN4
    State {
        sprite: SPR_CYBR,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_CYBER_RUN6,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_RUN5
    State {
        sprite: SPR_CYBR,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_CYBER_RUN7,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_RUN6
    State {
        sprite: SPR_CYBR,
        frame: 3,
        tics: 3,
        action: Some(A_Metal),
        nextstate: S_CYBER_RUN8,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_RUN7
    State {
        sprite: SPR_CYBR,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_CYBER_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_RUN8
    State {
        sprite: SPR_CYBR,
        frame: 4,
        tics: 6,
        action: Some(A_FaceTarget),
        nextstate: S_CYBER_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_ATK1
    State {
        sprite: SPR_CYBR,
        frame: 5,
        tics: 12,
        action: Some(A_CyberAttack),
        nextstate: S_CYBER_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_ATK2
    State {
        sprite: SPR_CYBR,
        frame: 4,
        tics: 12,
        action: Some(A_FaceTarget),
        nextstate: S_CYBER_ATK4,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_ATK3
    State {
        sprite: SPR_CYBR,
        frame: 5,
        tics: 12,
        action: Some(A_CyberAttack),
        nextstate: S_CYBER_ATK5,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_ATK4
    State {
        sprite: SPR_CYBR,
        frame: 4,
        tics: 12,
        action: Some(A_FaceTarget),
        nextstate: S_CYBER_ATK6,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_ATK5
    State {
        sprite: SPR_CYBR,
        frame: 5,
        tics: 12,
        action: Some(A_CyberAttack),
        nextstate: S_CYBER_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_ATK6
    State {
        sprite: SPR_CYBR,
        frame: 6,
        tics: 10,
        action: Some(A_Pain),
        nextstate: S_CYBER_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_PAIN
    State {
        sprite: SPR_CYBR,
        frame: 7,
        tics: 10,
        action: None,
        nextstate: S_CYBER_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_DIE1
    State {
        sprite: SPR_CYBR,
        frame: 8,
        tics: 10,
        action: Some(A_Scream),
        nextstate: S_CYBER_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_DIE2
    State {
        sprite: SPR_CYBR,
        frame: 9,
        tics: 10,
        action: None,
        nextstate: S_CYBER_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_DIE3
    State {
        sprite: SPR_CYBR,
        frame: 10,
        tics: 10,
        action: None,
        nextstate: S_CYBER_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_DIE4
    State {
        sprite: SPR_CYBR,
        frame: 11,
        tics: 10,
        action: None,
        nextstate: S_CYBER_DIE6,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_DIE5
    State {
        sprite: SPR_CYBR,
        frame: 12,
        tics: 10,
        action: Some(A_Fall),
        nextstate: S_CYBER_DIE7,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_DIE6
    State {
        sprite: SPR_CYBR,
        frame: 13,
        tics: 10,
        action: None,
        nextstate: S_CYBER_DIE8,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_DIE7
    State {
        sprite: SPR_CYBR,
        frame: 14,
        tics: 10,
        action: None,
        nextstate: S_CYBER_DIE9,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_DIE8
    State {
        sprite: SPR_CYBR,
        frame: 15,
        tics: 30,
        action: None,
        nextstate: S_CYBER_DIE10,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_DIE9
    State {
        sprite: SPR_CYBR,
        frame: 15,
        tics: -1,
        action: Some(A_BossDeath),
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_CYBER_DIE10
    State {
        sprite: SPR_PAIN,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_PAIN_STND,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_STND
    State {
        sprite: SPR_PAIN,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_PAIN_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_RUN1
    State {
        sprite: SPR_PAIN,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_PAIN_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_RUN2
    State {
        sprite: SPR_PAIN,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_PAIN_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_RUN3
    State {
        sprite: SPR_PAIN,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_PAIN_RUN5,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_RUN4
    State {
        sprite: SPR_PAIN,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_PAIN_RUN6,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_RUN5
    State {
        sprite: SPR_PAIN,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_PAIN_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_RUN6
    State {
        sprite: SPR_PAIN,
        frame: 3,
        tics: 5,
        action: Some(A_FaceTarget),
        nextstate: S_PAIN_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_ATK1
    State {
        sprite: SPR_PAIN,
        frame: 4,
        tics: 5,
        action: Some(A_FaceTarget),
        nextstate: S_PAIN_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_ATK2
    State {
        sprite: SPR_PAIN,
        frame: 32773,
        tics: 5,
        action: Some(A_FaceTarget),
        nextstate: S_PAIN_ATK4,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_ATK3
    State {
        sprite: SPR_PAIN,
        frame: 32773,
        tics: 0,
        action: Some(A_PainAttack),
        nextstate: S_PAIN_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_ATK4
    State {
        sprite: SPR_PAIN,
        frame: 6,
        tics: 6,
        action: None,
        nextstate: S_PAIN_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_PAIN
    State {
        sprite: SPR_PAIN,
        frame: 6,
        tics: 6,
        action: Some(A_Pain),
        nextstate: S_PAIN_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_PAIN2
    State {
        sprite: SPR_PAIN,
        frame: 32775,
        tics: 8,
        action: None,
        nextstate: S_PAIN_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_DIE1
    State {
        sprite: SPR_PAIN,
        frame: 32776,
        tics: 8,
        action: Some(A_Scream),
        nextstate: S_PAIN_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_DIE2
    State {
        sprite: SPR_PAIN,
        frame: 32777,
        tics: 8,
        action: None,
        nextstate: S_PAIN_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_DIE3
    State {
        sprite: SPR_PAIN,
        frame: 32778,
        tics: 8,
        action: None,
        nextstate: S_PAIN_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_DIE4
    State {
        sprite: SPR_PAIN,
        frame: 32779,
        tics: 8,
        action: Some(A_PainDie),
        nextstate: S_PAIN_DIE6,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_DIE5
    State {
        sprite: SPR_PAIN,
        frame: 32780,
        tics: 8,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_DIE6
    State {
        sprite: SPR_PAIN,
        frame: 12,
        tics: 8,
        action: None,
        nextstate: S_PAIN_RAISE2,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_RAISE1
    State {
        sprite: SPR_PAIN,
        frame: 11,
        tics: 8,
        action: None,
        nextstate: S_PAIN_RAISE3,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_RAISE2
    State {
        sprite: SPR_PAIN,
        frame: 10,
        tics: 8,
        action: None,
        nextstate: S_PAIN_RAISE4,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_RAISE3
    State {
        sprite: SPR_PAIN,
        frame: 9,
        tics: 8,
        action: None,
        nextstate: S_PAIN_RAISE5,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_RAISE4
    State {
        sprite: SPR_PAIN,
        frame: 8,
        tics: 8,
        action: None,
        nextstate: S_PAIN_RAISE6,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_RAISE5
    State {
        sprite: SPR_PAIN,
        frame: 7,
        tics: 8,
        action: None,
        nextstate: S_PAIN_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_PAIN_RAISE6
    State {
        sprite: SPR_SSWV,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_SSWV_STND2,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_STND
    State {
        sprite: SPR_SSWV,
        frame: 1,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_SSWV_STND,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_STND2
    State {
        sprite: SPR_SSWV,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SSWV_RUN2,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_RUN1
    State {
        sprite: SPR_SSWV,
        frame: 0,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SSWV_RUN3,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_RUN2
    State {
        sprite: SPR_SSWV,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SSWV_RUN4,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_RUN3
    State {
        sprite: SPR_SSWV,
        frame: 1,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SSWV_RUN5,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_RUN4
    State {
        sprite: SPR_SSWV,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SSWV_RUN6,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_RUN5
    State {
        sprite: SPR_SSWV,
        frame: 2,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SSWV_RUN7,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_RUN6
    State {
        sprite: SPR_SSWV,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SSWV_RUN8,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_RUN7
    State {
        sprite: SPR_SSWV,
        frame: 3,
        tics: 3,
        action: Some(A_Chase),
        nextstate: S_SSWV_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_RUN8
    State {
        sprite: SPR_SSWV,
        frame: 4,
        tics: 10,
        action: Some(A_FaceTarget),
        nextstate: S_SSWV_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_ATK1
    State {
        sprite: SPR_SSWV,
        frame: 5,
        tics: 10,
        action: Some(A_FaceTarget),
        nextstate: S_SSWV_ATK3,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_ATK2
    State {
        sprite: SPR_SSWV,
        frame: 32774,
        tics: 4,
        action: Some(A_CPosAttack),
        nextstate: S_SSWV_ATK4,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_ATK3
    State {
        sprite: SPR_SSWV,
        frame: 5,
        tics: 6,
        action: Some(A_FaceTarget),
        nextstate: S_SSWV_ATK5,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_ATK4
    State {
        sprite: SPR_SSWV,
        frame: 32774,
        tics: 4,
        action: Some(A_CPosAttack),
        nextstate: S_SSWV_ATK6,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_ATK5
    State {
        sprite: SPR_SSWV,
        frame: 5,
        tics: 1,
        action: Some(A_CPosRefire),
        nextstate: S_SSWV_ATK2,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_ATK6
    State {
        sprite: SPR_SSWV,
        frame: 7,
        tics: 3,
        action: None,
        nextstate: S_SSWV_PAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_PAIN
    State {
        sprite: SPR_SSWV,
        frame: 7,
        tics: 3,
        action: Some(A_Pain),
        nextstate: S_SSWV_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_PAIN2
    State {
        sprite: SPR_SSWV,
        frame: 8,
        tics: 5,
        action: None,
        nextstate: S_SSWV_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_DIE1
    State {
        sprite: SPR_SSWV,
        frame: 9,
        tics: 5,
        action: Some(A_Scream),
        nextstate: S_SSWV_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_DIE2
    State {
        sprite: SPR_SSWV,
        frame: 10,
        tics: 5,
        action: Some(A_Fall),
        nextstate: S_SSWV_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_DIE3
    State {
        sprite: SPR_SSWV,
        frame: 11,
        tics: 5,
        action: None,
        nextstate: S_SSWV_DIE5,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_DIE4
    State {
        sprite: SPR_SSWV,
        frame: 12,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_DIE5
    State {
        sprite: SPR_SSWV,
        frame: 13,
        tics: 5,
        action: None,
        nextstate: S_SSWV_XDIE2,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_XDIE1
    State {
        sprite: SPR_SSWV,
        frame: 14,
        tics: 5,
        action: Some(A_XScream),
        nextstate: S_SSWV_XDIE3,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_XDIE2
    State {
        sprite: SPR_SSWV,
        frame: 15,
        tics: 5,
        action: Some(A_Fall),
        nextstate: S_SSWV_XDIE4,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_XDIE3
    State {
        sprite: SPR_SSWV,
        frame: 16,
        tics: 5,
        action: None,
        nextstate: S_SSWV_XDIE5,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_XDIE4
    State {
        sprite: SPR_SSWV,
        frame: 17,
        tics: 5,
        action: None,
        nextstate: S_SSWV_XDIE6,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_XDIE5
    State {
        sprite: SPR_SSWV,
        frame: 18,
        tics: 5,
        action: None,
        nextstate: S_SSWV_XDIE7,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_XDIE6
    State {
        sprite: SPR_SSWV,
        frame: 19,
        tics: 5,
        action: None,
        nextstate: S_SSWV_XDIE8,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_XDIE7
    State {
        sprite: SPR_SSWV,
        frame: 20,
        tics: 5,
        action: None,
        nextstate: S_SSWV_XDIE9,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_XDIE8
    State {
        sprite: SPR_SSWV,
        frame: 21,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_XDIE9
    State {
        sprite: SPR_SSWV,
        frame: 12,
        tics: 5,
        action: None,
        nextstate: S_SSWV_RAISE2,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_RAISE1
    State {
        sprite: SPR_SSWV,
        frame: 11,
        tics: 5,
        action: None,
        nextstate: S_SSWV_RAISE3,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_RAISE2
    State {
        sprite: SPR_SSWV,
        frame: 10,
        tics: 5,
        action: None,
        nextstate: S_SSWV_RAISE4,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_RAISE3
    State {
        sprite: SPR_SSWV,
        frame: 9,
        tics: 5,
        action: None,
        nextstate: S_SSWV_RAISE5,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_RAISE4
    State {
        sprite: SPR_SSWV,
        frame: 8,
        tics: 5,
        action: None,
        nextstate: S_SSWV_RUN1,
        misc1: 0,
        misc2: 0,
    }, // S_SSWV_RAISE5
    State {
        sprite: SPR_KEEN,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_KEENSTND,
        misc1: 0,
        misc2: 0,
    }, // S_KEENSTND
    State {
        sprite: SPR_KEEN,
        frame: 0,
        tics: 6,
        action: None,
        nextstate: S_COMMKEEN2,
        misc1: 0,
        misc2: 0,
    }, // S_COMMKEEN
    State {
        sprite: SPR_KEEN,
        frame: 1,
        tics: 6,
        action: None,
        nextstate: S_COMMKEEN3,
        misc1: 0,
        misc2: 0,
    }, // S_COMMKEEN2
    State {
        sprite: SPR_KEEN,
        frame: 2,
        tics: 6,
        action: Some(A_Scream),
        nextstate: S_COMMKEEN4,
        misc1: 0,
        misc2: 0,
    }, // S_COMMKEEN3
    State {
        sprite: SPR_KEEN,
        frame: 3,
        tics: 6,
        action: None,
        nextstate: S_COMMKEEN5,
        misc1: 0,
        misc2: 0,
    }, // S_COMMKEEN4
    State {
        sprite: SPR_KEEN,
        frame: 4,
        tics: 6,
        action: None,
        nextstate: S_COMMKEEN6,
        misc1: 0,
        misc2: 0,
    }, // S_COMMKEEN5
    State {
        sprite: SPR_KEEN,
        frame: 5,
        tics: 6,
        action: None,
        nextstate: S_COMMKEEN7,
        misc1: 0,
        misc2: 0,
    }, // S_COMMKEEN6
    State {
        sprite: SPR_KEEN,
        frame: 6,
        tics: 6,
        action: None,
        nextstate: S_COMMKEEN8,
        misc1: 0,
        misc2: 0,
    }, // S_COMMKEEN7
    State {
        sprite: SPR_KEEN,
        frame: 7,
        tics: 6,
        action: None,
        nextstate: S_COMMKEEN9,
        misc1: 0,
        misc2: 0,
    }, // S_COMMKEEN8
    State {
        sprite: SPR_KEEN,
        frame: 8,
        tics: 6,
        action: None,
        nextstate: S_COMMKEEN10,
        misc1: 0,
        misc2: 0,
    }, // S_COMMKEEN9
    State {
        sprite: SPR_KEEN,
        frame: 9,
        tics: 6,
        action: None,
        nextstate: S_COMMKEEN11,
        misc1: 0,
        misc2: 0,
    }, // S_COMMKEEN10
    State {
        sprite: SPR_KEEN,
        frame: 10,
        tics: 6,
        action: Some(A_KeenDie),
        nextstate: S_COMMKEEN12,
        misc1: 0,
        misc2: 0,
    }, // S_COMMKEEN11
    State {
        sprite: SPR_KEEN,
        frame: 11,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_COMMKEEN12
    State {
        sprite: SPR_KEEN,
        frame: 12,
        tics: 4,
        action: None,
        nextstate: S_KEENPAIN2,
        misc1: 0,
        misc2: 0,
    }, // S_KEENPAIN
    State {
        sprite: SPR_KEEN,
        frame: 12,
        tics: 8,
        action: Some(A_Pain),
        nextstate: S_KEENSTND,
        misc1: 0,
        misc2: 0,
    }, // S_KEENPAIN2
    State {
        sprite: SPR_BBRN,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BRAIN
    State {
        sprite: SPR_BBRN,
        frame: 1,
        tics: 36,
        action: Some(A_BrainPain),
        nextstate: S_BRAIN,
        misc1: 0,
        misc2: 0,
    }, // S_BRAIN_PAIN
    State {
        sprite: SPR_BBRN,
        frame: 0,
        tics: 100,
        action: Some(A_BrainScream),
        nextstate: S_BRAIN_DIE2,
        misc1: 0,
        misc2: 0,
    }, // S_BRAIN_DIE1
    State {
        sprite: SPR_BBRN,
        frame: 0,
        tics: 10,
        action: None,
        nextstate: S_BRAIN_DIE3,
        misc1: 0,
        misc2: 0,
    }, // S_BRAIN_DIE2
    State {
        sprite: SPR_BBRN,
        frame: 0,
        tics: 10,
        action: None,
        nextstate: S_BRAIN_DIE4,
        misc1: 0,
        misc2: 0,
    }, // S_BRAIN_DIE3
    State {
        sprite: SPR_BBRN,
        frame: 0,
        tics: -1,
        action: Some(A_BrainDie),
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BRAIN_DIE4
    State {
        sprite: SPR_SSWV,
        frame: 0,
        tics: 10,
        action: Some(A_Look),
        nextstate: S_BRAINEYE,
        misc1: 0,
        misc2: 0,
    }, // S_BRAINEYE
    State {
        sprite: SPR_SSWV,
        frame: 0,
        tics: 181,
        action: Some(A_BrainAwake),
        nextstate: S_BRAINEYE1,
        misc1: 0,
        misc2: 0,
    }, // S_BRAINEYESEE
    State {
        sprite: SPR_SSWV,
        frame: 0,
        tics: 150,
        action: Some(A_BrainSpit),
        nextstate: S_BRAINEYE1,
        misc1: 0,
        misc2: 0,
    }, // S_BRAINEYE1
    State {
        sprite: SPR_BOSF,
        frame: 32768,
        tics: 3,
        action: Some(A_SpawnSound),
        nextstate: S_SPAWN2,
        misc1: 0,
        misc2: 0,
    }, // S_SPAWN1
    State {
        sprite: SPR_BOSF,
        frame: 32769,
        tics: 3,
        action: Some(A_SpawnFly),
        nextstate: S_SPAWN3,
        misc1: 0,
        misc2: 0,
    }, // S_SPAWN2
    State {
        sprite: SPR_BOSF,
        frame: 32770,
        tics: 3,
        action: Some(A_SpawnFly),
        nextstate: S_SPAWN4,
        misc1: 0,
        misc2: 0,
    }, // S_SPAWN3
    State {
        sprite: SPR_BOSF,
        frame: 32771,
        tics: 3,
        action: Some(A_SpawnFly),
        nextstate: S_SPAWN1,
        misc1: 0,
        misc2: 0,
    }, // S_SPAWN4
    State {
        sprite: SPR_FIRE,
        frame: 32768,
        tics: 4,
        action: Some(A_Fire),
        nextstate: S_SPAWNFIRE2,
        misc1: 0,
        misc2: 0,
    }, // S_SPAWNFIRE1
    State {
        sprite: SPR_FIRE,
        frame: 32769,
        tics: 4,
        action: Some(A_Fire),
        nextstate: S_SPAWNFIRE3,
        misc1: 0,
        misc2: 0,
    }, // S_SPAWNFIRE2
    State {
        sprite: SPR_FIRE,
        frame: 32770,
        tics: 4,
        action: Some(A_Fire),
        nextstate: S_SPAWNFIRE4,
        misc1: 0,
        misc2: 0,
    }, // S_SPAWNFIRE3
    State {
        sprite: SPR_FIRE,
        frame: 32771,
        tics: 4,
        action: Some(A_Fire),
        nextstate: S_SPAWNFIRE5,
        misc1: 0,
        misc2: 0,
    }, // S_SPAWNFIRE4
    State {
        sprite: SPR_FIRE,
        frame: 32772,
        tics: 4,
        action: Some(A_Fire),
        nextstate: S_SPAWNFIRE6,
        misc1: 0,
        misc2: 0,
    }, // S_SPAWNFIRE5
    State {
        sprite: SPR_FIRE,
        frame: 32773,
        tics: 4,
        action: Some(A_Fire),
        nextstate: S_SPAWNFIRE7,
        misc1: 0,
        misc2: 0,
    }, // S_SPAWNFIRE6
    State {
        sprite: SPR_FIRE,
        frame: 32774,
        tics: 4,
        action: Some(A_Fire),
        nextstate: S_SPAWNFIRE8,
        misc1: 0,
        misc2: 0,
    }, // S_SPAWNFIRE7
    State {
        sprite: SPR_FIRE,
        frame: 32775,
        tics: 4,
        action: Some(A_Fire),
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SPAWNFIRE8
    State {
        sprite: SPR_MISL,
        frame: 32769,
        tics: 10,
        action: None,
        nextstate: S_BRAINEXPLODE2,
        misc1: 0,
        misc2: 0,
    }, // S_BRAINEXPLODE1
    State {
        sprite: SPR_MISL,
        frame: 32770,
        tics: 10,
        action: None,
        nextstate: S_BRAINEXPLODE3,
        misc1: 0,
        misc2: 0,
    }, // S_BRAINEXPLODE2
    State {
        sprite: SPR_MISL,
        frame: 32771,
        tics: 10,
        action: Some(A_BrainExplode),
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BRAINEXPLODE3
    State {
        sprite: SPR_ARM1,
        frame: 0,
        tics: 6,
        action: None,
        nextstate: S_ARM1A,
        misc1: 0,
        misc2: 0,
    }, // S_ARM1
    State {
        sprite: SPR_ARM1,
        frame: 32769,
        tics: 7,
        action: None,
        nextstate: S_ARM1,
        misc1: 0,
        misc2: 0,
    }, // S_ARM1A
    State {
        sprite: SPR_ARM2,
        frame: 0,
        tics: 6,
        action: None,
        nextstate: S_ARM2A,
        misc1: 0,
        misc2: 0,
    }, // S_ARM2
    State {
        sprite: SPR_ARM2,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_ARM2,
        misc1: 0,
        misc2: 0,
    }, // S_ARM2A
    State {
        sprite: SPR_BAR1,
        frame: 0,
        tics: 6,
        action: None,
        nextstate: S_BAR2,
        misc1: 0,
        misc2: 0,
    }, // S_BAR1
    State {
        sprite: SPR_BAR1,
        frame: 1,
        tics: 6,
        action: None,
        nextstate: S_BAR1,
        misc1: 0,
        misc2: 0,
    }, // S_BAR2
    State {
        sprite: SPR_BEXP,
        frame: 32768,
        tics: 5,
        action: None,
        nextstate: S_BEXP2,
        misc1: 0,
        misc2: 0,
    }, // S_BEXP
    State {
        sprite: SPR_BEXP,
        frame: 32769,
        tics: 5,
        action: Some(A_Scream),
        nextstate: S_BEXP3,
        misc1: 0,
        misc2: 0,
    }, // S_BEXP2
    State {
        sprite: SPR_BEXP,
        frame: 32770,
        tics: 5,
        action: None,
        nextstate: S_BEXP4,
        misc1: 0,
        misc2: 0,
    }, // S_BEXP3
    State {
        sprite: SPR_BEXP,
        frame: 32771,
        tics: 10,
        action: Some(A_Explode),
        nextstate: S_BEXP5,
        misc1: 0,
        misc2: 0,
    }, // S_BEXP4
    State {
        sprite: SPR_BEXP,
        frame: 32772,
        tics: 10,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BEXP5
    State {
        sprite: SPR_FCAN,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_BBAR2,
        misc1: 0,
        misc2: 0,
    }, // S_BBAR1
    State {
        sprite: SPR_FCAN,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_BBAR3,
        misc1: 0,
        misc2: 0,
    }, // S_BBAR2
    State {
        sprite: SPR_FCAN,
        frame: 32770,
        tics: 4,
        action: None,
        nextstate: S_BBAR1,
        misc1: 0,
        misc2: 0,
    }, // S_BBAR3
    State {
        sprite: SPR_BON1,
        frame: 0,
        tics: 6,
        action: None,
        nextstate: S_BON1A,
        misc1: 0,
        misc2: 0,
    }, // S_BON1
    State {
        sprite: SPR_BON1,
        frame: 1,
        tics: 6,
        action: None,
        nextstate: S_BON1B,
        misc1: 0,
        misc2: 0,
    }, // S_BON1A
    State {
        sprite: SPR_BON1,
        frame: 2,
        tics: 6,
        action: None,
        nextstate: S_BON1C,
        misc1: 0,
        misc2: 0,
    }, // S_BON1B
    State {
        sprite: SPR_BON1,
        frame: 3,
        tics: 6,
        action: None,
        nextstate: S_BON1D,
        misc1: 0,
        misc2: 0,
    }, // S_BON1C
    State {
        sprite: SPR_BON1,
        frame: 2,
        tics: 6,
        action: None,
        nextstate: S_BON1E,
        misc1: 0,
        misc2: 0,
    }, // S_BON1D
    State {
        sprite: SPR_BON1,
        frame: 1,
        tics: 6,
        action: None,
        nextstate: S_BON1,
        misc1: 0,
        misc2: 0,
    }, // S_BON1E
    State {
        sprite: SPR_BON2,
        frame: 0,
        tics: 6,
        action: None,
        nextstate: S_BON2A,
        misc1: 0,
        misc2: 0,
    }, // S_BON2
    State {
        sprite: SPR_BON2,
        frame: 1,
        tics: 6,
        action: None,
        nextstate: S_BON2B,
        misc1: 0,
        misc2: 0,
    }, // S_BON2A
    State {
        sprite: SPR_BON2,
        frame: 2,
        tics: 6,
        action: None,
        nextstate: S_BON2C,
        misc1: 0,
        misc2: 0,
    }, // S_BON2B
    State {
        sprite: SPR_BON2,
        frame: 3,
        tics: 6,
        action: None,
        nextstate: S_BON2D,
        misc1: 0,
        misc2: 0,
    }, // S_BON2C
    State {
        sprite: SPR_BON2,
        frame: 2,
        tics: 6,
        action: None,
        nextstate: S_BON2E,
        misc1: 0,
        misc2: 0,
    }, // S_BON2D
    State {
        sprite: SPR_BON2,
        frame: 1,
        tics: 6,
        action: None,
        nextstate: S_BON2,
        misc1: 0,
        misc2: 0,
    }, // S_BON2E
    State {
        sprite: SPR_BKEY,
        frame: 0,
        tics: 10,
        action: None,
        nextstate: S_BKEY2,
        misc1: 0,
        misc2: 0,
    }, // S_BKEY
    State {
        sprite: SPR_BKEY,
        frame: 32769,
        tics: 10,
        action: None,
        nextstate: S_BKEY,
        misc1: 0,
        misc2: 0,
    }, // S_BKEY2
    State {
        sprite: SPR_RKEY,
        frame: 0,
        tics: 10,
        action: None,
        nextstate: S_RKEY2,
        misc1: 0,
        misc2: 0,
    }, // S_RKEY
    State {
        sprite: SPR_RKEY,
        frame: 32769,
        tics: 10,
        action: None,
        nextstate: S_RKEY,
        misc1: 0,
        misc2: 0,
    }, // S_RKEY2
    State {
        sprite: SPR_YKEY,
        frame: 0,
        tics: 10,
        action: None,
        nextstate: S_YKEY2,
        misc1: 0,
        misc2: 0,
    }, // S_YKEY
    State {
        sprite: SPR_YKEY,
        frame: 32769,
        tics: 10,
        action: None,
        nextstate: S_YKEY,
        misc1: 0,
        misc2: 0,
    }, // S_YKEY2
    State {
        sprite: SPR_BSKU,
        frame: 0,
        tics: 10,
        action: None,
        nextstate: S_BSKULL2,
        misc1: 0,
        misc2: 0,
    }, // S_BSKULL
    State {
        sprite: SPR_BSKU,
        frame: 32769,
        tics: 10,
        action: None,
        nextstate: S_BSKULL,
        misc1: 0,
        misc2: 0,
    }, // S_BSKULL2
    State {
        sprite: SPR_RSKU,
        frame: 0,
        tics: 10,
        action: None,
        nextstate: S_RSKULL2,
        misc1: 0,
        misc2: 0,
    }, // S_RSKULL
    State {
        sprite: SPR_RSKU,
        frame: 32769,
        tics: 10,
        action: None,
        nextstate: S_RSKULL,
        misc1: 0,
        misc2: 0,
    }, // S_RSKULL2
    State {
        sprite: SPR_YSKU,
        frame: 0,
        tics: 10,
        action: None,
        nextstate: S_YSKULL2,
        misc1: 0,
        misc2: 0,
    }, // S_YSKULL
    State {
        sprite: SPR_YSKU,
        frame: 32769,
        tics: 10,
        action: None,
        nextstate: S_YSKULL,
        misc1: 0,
        misc2: 0,
    }, // S_YSKULL2
    State {
        sprite: SPR_STIM,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_STIM
    State {
        sprite: SPR_MEDI,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_MEDI
    State {
        sprite: SPR_SOUL,
        frame: 32768,
        tics: 6,
        action: None,
        nextstate: S_SOUL2,
        misc1: 0,
        misc2: 0,
    }, // S_SOUL
    State {
        sprite: SPR_SOUL,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_SOUL3,
        misc1: 0,
        misc2: 0,
    }, // S_SOUL2
    State {
        sprite: SPR_SOUL,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_SOUL4,
        misc1: 0,
        misc2: 0,
    }, // S_SOUL3
    State {
        sprite: SPR_SOUL,
        frame: 32771,
        tics: 6,
        action: None,
        nextstate: S_SOUL5,
        misc1: 0,
        misc2: 0,
    }, // S_SOUL4
    State {
        sprite: SPR_SOUL,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_SOUL6,
        misc1: 0,
        misc2: 0,
    }, // S_SOUL5
    State {
        sprite: SPR_SOUL,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_SOUL,
        misc1: 0,
        misc2: 0,
    }, // S_SOUL6
    State {
        sprite: SPR_PINV,
        frame: 32768,
        tics: 6,
        action: None,
        nextstate: S_PINV2,
        misc1: 0,
        misc2: 0,
    }, // S_PINV
    State {
        sprite: SPR_PINV,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_PINV3,
        misc1: 0,
        misc2: 0,
    }, // S_PINV2
    State {
        sprite: SPR_PINV,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_PINV4,
        misc1: 0,
        misc2: 0,
    }, // S_PINV3
    State {
        sprite: SPR_PINV,
        frame: 32771,
        tics: 6,
        action: None,
        nextstate: S_PINV,
        misc1: 0,
        misc2: 0,
    }, // S_PINV4
    State {
        sprite: SPR_PSTR,
        frame: 32768,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_PSTR
    State {
        sprite: SPR_PINS,
        frame: 32768,
        tics: 6,
        action: None,
        nextstate: S_PINS2,
        misc1: 0,
        misc2: 0,
    }, // S_PINS
    State {
        sprite: SPR_PINS,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_PINS3,
        misc1: 0,
        misc2: 0,
    }, // S_PINS2
    State {
        sprite: SPR_PINS,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_PINS4,
        misc1: 0,
        misc2: 0,
    }, // S_PINS3
    State {
        sprite: SPR_PINS,
        frame: 32771,
        tics: 6,
        action: None,
        nextstate: S_PINS,
        misc1: 0,
        misc2: 0,
    }, // S_PINS4
    State {
        sprite: SPR_MEGA,
        frame: 32768,
        tics: 6,
        action: None,
        nextstate: S_MEGA2,
        misc1: 0,
        misc2: 0,
    }, // S_MEGA
    State {
        sprite: SPR_MEGA,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_MEGA3,
        misc1: 0,
        misc2: 0,
    }, // S_MEGA2
    State {
        sprite: SPR_MEGA,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_MEGA4,
        misc1: 0,
        misc2: 0,
    }, // S_MEGA3
    State {
        sprite: SPR_MEGA,
        frame: 32771,
        tics: 6,
        action: None,
        nextstate: S_MEGA,
        misc1: 0,
        misc2: 0,
    }, // S_MEGA4
    State {
        sprite: SPR_SUIT,
        frame: 32768,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SUIT
    State {
        sprite: SPR_PMAP,
        frame: 32768,
        tics: 6,
        action: None,
        nextstate: S_PMAP2,
        misc1: 0,
        misc2: 0,
    }, // S_PMAP
    State {
        sprite: SPR_PMAP,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_PMAP3,
        misc1: 0,
        misc2: 0,
    }, // S_PMAP2
    State {
        sprite: SPR_PMAP,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_PMAP4,
        misc1: 0,
        misc2: 0,
    }, // S_PMAP3
    State {
        sprite: SPR_PMAP,
        frame: 32771,
        tics: 6,
        action: None,
        nextstate: S_PMAP5,
        misc1: 0,
        misc2: 0,
    }, // S_PMAP4
    State {
        sprite: SPR_PMAP,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_PMAP6,
        misc1: 0,
        misc2: 0,
    }, // S_PMAP5
    State {
        sprite: SPR_PMAP,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_PMAP,
        misc1: 0,
        misc2: 0,
    }, // S_PMAP6
    State {
        sprite: SPR_PVIS,
        frame: 32768,
        tics: 6,
        action: None,
        nextstate: S_PVIS2,
        misc1: 0,
        misc2: 0,
    }, // S_PVIS
    State {
        sprite: SPR_PVIS,
        frame: 1,
        tics: 6,
        action: None,
        nextstate: S_PVIS,
        misc1: 0,
        misc2: 0,
    }, // S_PVIS2
    State {
        sprite: SPR_CLIP,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_CLIP
    State {
        sprite: SPR_AMMO,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_AMMO
    State {
        sprite: SPR_ROCK,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_ROCK
    State {
        sprite: SPR_BROK,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BROK
    State {
        sprite: SPR_CELL,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_CELL
    State {
        sprite: SPR_CELP,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_CELP
    State {
        sprite: SPR_SHEL,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SHEL
    State {
        sprite: SPR_SBOX,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SBOX
    State {
        sprite: SPR_BPAK,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BPAK
    State {
        sprite: SPR_BFUG,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BFUG
    State {
        sprite: SPR_MGUN,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_MGUN
    State {
        sprite: SPR_CSAW,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_CSAW
    State {
        sprite: SPR_LAUN,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_LAUN
    State {
        sprite: SPR_PLAS,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_PLAS
    State {
        sprite: SPR_SHOT,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SHOT
    State {
        sprite: SPR_SGN2,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SHOT2
    State {
        sprite: SPR_COLU,
        frame: 32768,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_COLU
    State {
        sprite: SPR_SMT2,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_STALAG
    State {
        sprite: SPR_GOR1,
        frame: 0,
        tics: 10,
        action: None,
        nextstate: S_BLOODYTWITCH2,
        misc1: 0,
        misc2: 0,
    }, // S_BLOODYTWITCH
    State {
        sprite: SPR_GOR1,
        frame: 1,
        tics: 15,
        action: None,
        nextstate: S_BLOODYTWITCH3,
        misc1: 0,
        misc2: 0,
    }, // S_BLOODYTWITCH2
    State {
        sprite: SPR_GOR1,
        frame: 2,
        tics: 8,
        action: None,
        nextstate: S_BLOODYTWITCH4,
        misc1: 0,
        misc2: 0,
    }, // S_BLOODYTWITCH3
    State {
        sprite: SPR_GOR1,
        frame: 1,
        tics: 6,
        action: None,
        nextstate: S_BLOODYTWITCH,
        misc1: 0,
        misc2: 0,
    }, // S_BLOODYTWITCH4
    State {
        sprite: SPR_PLAY,
        frame: 13,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_DEADTORSO
    State {
        sprite: SPR_PLAY,
        frame: 18,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_DEADBOTTOM
    State {
        sprite: SPR_POL2,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_HEADSONSTICK
    State {
        sprite: SPR_POL5,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_GIBS
    State {
        sprite: SPR_POL4,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_HEADONASTICK
    State {
        sprite: SPR_POL3,
        frame: 32768,
        tics: 6,
        action: None,
        nextstate: S_HEADCANDLES2,
        misc1: 0,
        misc2: 0,
    }, // S_HEADCANDLES
    State {
        sprite: SPR_POL3,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_HEADCANDLES,
        misc1: 0,
        misc2: 0,
    }, // S_HEADCANDLES2
    State {
        sprite: SPR_POL1,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_DEADSTICK
    State {
        sprite: SPR_POL6,
        frame: 0,
        tics: 6,
        action: None,
        nextstate: S_LIVESTICK2,
        misc1: 0,
        misc2: 0,
    }, // S_LIVESTICK
    State {
        sprite: SPR_POL6,
        frame: 1,
        tics: 8,
        action: None,
        nextstate: S_LIVESTICK,
        misc1: 0,
        misc2: 0,
    }, // S_LIVESTICK2
    State {
        sprite: SPR_GOR2,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_MEAT2
    State {
        sprite: SPR_GOR3,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_MEAT3
    State {
        sprite: SPR_GOR4,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_MEAT4
    State {
        sprite: SPR_GOR5,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_MEAT5
    State {
        sprite: SPR_SMIT,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_STALAGTITE
    State {
        sprite: SPR_COL1,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_TALLGRNCOL
    State {
        sprite: SPR_COL2,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SHRTGRNCOL
    State {
        sprite: SPR_COL3,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_TALLREDCOL
    State {
        sprite: SPR_COL4,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SHRTREDCOL
    State {
        sprite: SPR_CAND,
        frame: 32768,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_CANDLESTIK
    State {
        sprite: SPR_CBRA,
        frame: 32768,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_CANDELABRA
    State {
        sprite: SPR_COL6,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SKULLCOL
    State {
        sprite: SPR_TRE1,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_TORCHTREE
    State {
        sprite: SPR_TRE2,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BIGTREE
    State {
        sprite: SPR_ELEC,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_TECHPILLAR
    State {
        sprite: SPR_CEYE,
        frame: 32768,
        tics: 6,
        action: None,
        nextstate: S_EVILEYE2,
        misc1: 0,
        misc2: 0,
    }, // S_EVILEYE
    State {
        sprite: SPR_CEYE,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_EVILEYE3,
        misc1: 0,
        misc2: 0,
    }, // S_EVILEYE2
    State {
        sprite: SPR_CEYE,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_EVILEYE4,
        misc1: 0,
        misc2: 0,
    }, // S_EVILEYE3
    State {
        sprite: SPR_CEYE,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_EVILEYE,
        misc1: 0,
        misc2: 0,
    }, // S_EVILEYE4
    State {
        sprite: SPR_FSKU,
        frame: 32768,
        tics: 6,
        action: None,
        nextstate: S_FLOATSKULL2,
        misc1: 0,
        misc2: 0,
    }, // S_FLOATSKULL
    State {
        sprite: SPR_FSKU,
        frame: 32769,
        tics: 6,
        action: None,
        nextstate: S_FLOATSKULL3,
        misc1: 0,
        misc2: 0,
    }, // S_FLOATSKULL2
    State {
        sprite: SPR_FSKU,
        frame: 32770,
        tics: 6,
        action: None,
        nextstate: S_FLOATSKULL,
        misc1: 0,
        misc2: 0,
    }, // S_FLOATSKULL3
    State {
        sprite: SPR_COL5,
        frame: 0,
        tics: 14,
        action: None,
        nextstate: S_HEARTCOL2,
        misc1: 0,
        misc2: 0,
    }, // S_HEARTCOL
    State {
        sprite: SPR_COL5,
        frame: 1,
        tics: 14,
        action: None,
        nextstate: S_HEARTCOL,
        misc1: 0,
        misc2: 0,
    }, // S_HEARTCOL2
    State {
        sprite: SPR_TBLU,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_BLUETORCH2,
        misc1: 0,
        misc2: 0,
    }, // S_BLUETORCH
    State {
        sprite: SPR_TBLU,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_BLUETORCH3,
        misc1: 0,
        misc2: 0,
    }, // S_BLUETORCH2
    State {
        sprite: SPR_TBLU,
        frame: 32770,
        tics: 4,
        action: None,
        nextstate: S_BLUETORCH4,
        misc1: 0,
        misc2: 0,
    }, // S_BLUETORCH3
    State {
        sprite: SPR_TBLU,
        frame: 32771,
        tics: 4,
        action: None,
        nextstate: S_BLUETORCH,
        misc1: 0,
        misc2: 0,
    }, // S_BLUETORCH4
    State {
        sprite: SPR_TGRN,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_GREENTORCH2,
        misc1: 0,
        misc2: 0,
    }, // S_GREENTORCH
    State {
        sprite: SPR_TGRN,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_GREENTORCH3,
        misc1: 0,
        misc2: 0,
    }, // S_GREENTORCH2
    State {
        sprite: SPR_TGRN,
        frame: 32770,
        tics: 4,
        action: None,
        nextstate: S_GREENTORCH4,
        misc1: 0,
        misc2: 0,
    }, // S_GREENTORCH3
    State {
        sprite: SPR_TGRN,
        frame: 32771,
        tics: 4,
        action: None,
        nextstate: S_GREENTORCH,
        misc1: 0,
        misc2: 0,
    }, // S_GREENTORCH4
    State {
        sprite: SPR_TRED,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_REDTORCH2,
        misc1: 0,
        misc2: 0,
    }, // S_REDTORCH
    State {
        sprite: SPR_TRED,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_REDTORCH3,
        misc1: 0,
        misc2: 0,
    }, // S_REDTORCH2
    State {
        sprite: SPR_TRED,
        frame: 32770,
        tics: 4,
        action: None,
        nextstate: S_REDTORCH4,
        misc1: 0,
        misc2: 0,
    }, // S_REDTORCH3
    State {
        sprite: SPR_TRED,
        frame: 32771,
        tics: 4,
        action: None,
        nextstate: S_REDTORCH,
        misc1: 0,
        misc2: 0,
    }, // S_REDTORCH4
    State {
        sprite: SPR_SMBT,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_BTORCHSHRT2,
        misc1: 0,
        misc2: 0,
    }, // S_BTORCHSHRT
    State {
        sprite: SPR_SMBT,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_BTORCHSHRT3,
        misc1: 0,
        misc2: 0,
    }, // S_BTORCHSHRT2
    State {
        sprite: SPR_SMBT,
        frame: 32770,
        tics: 4,
        action: None,
        nextstate: S_BTORCHSHRT4,
        misc1: 0,
        misc2: 0,
    }, // S_BTORCHSHRT3
    State {
        sprite: SPR_SMBT,
        frame: 32771,
        tics: 4,
        action: None,
        nextstate: S_BTORCHSHRT,
        misc1: 0,
        misc2: 0,
    }, // S_BTORCHSHRT4
    State {
        sprite: SPR_SMGT,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_GTORCHSHRT2,
        misc1: 0,
        misc2: 0,
    }, // S_GTORCHSHRT
    State {
        sprite: SPR_SMGT,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_GTORCHSHRT3,
        misc1: 0,
        misc2: 0,
    }, // S_GTORCHSHRT2
    State {
        sprite: SPR_SMGT,
        frame: 32770,
        tics: 4,
        action: None,
        nextstate: S_GTORCHSHRT4,
        misc1: 0,
        misc2: 0,
    }, // S_GTORCHSHRT3
    State {
        sprite: SPR_SMGT,
        frame: 32771,
        tics: 4,
        action: None,
        nextstate: S_GTORCHSHRT,
        misc1: 0,
        misc2: 0,
    }, // S_GTORCHSHRT4
    State {
        sprite: SPR_SMRT,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_RTORCHSHRT2,
        misc1: 0,
        misc2: 0,
    }, // S_RTORCHSHRT
    State {
        sprite: SPR_SMRT,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_RTORCHSHRT3,
        misc1: 0,
        misc2: 0,
    }, // S_RTORCHSHRT2
    State {
        sprite: SPR_SMRT,
        frame: 32770,
        tics: 4,
        action: None,
        nextstate: S_RTORCHSHRT4,
        misc1: 0,
        misc2: 0,
    }, // S_RTORCHSHRT3
    State {
        sprite: SPR_SMRT,
        frame: 32771,
        tics: 4,
        action: None,
        nextstate: S_RTORCHSHRT,
        misc1: 0,
        misc2: 0,
    }, // S_RTORCHSHRT4
    State {
        sprite: SPR_HDB1,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_HANGNOGUTS
    State {
        sprite: SPR_HDB2,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_HANGBNOBRAIN
    State {
        sprite: SPR_HDB3,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_HANGTLOOKDN
    State {
        sprite: SPR_HDB4,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_HANGTSKULL
    State {
        sprite: SPR_HDB5,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_HANGTLOOKUP
    State {
        sprite: SPR_HDB6,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_HANGTNOBRAIN
    State {
        sprite: SPR_POB1,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_COLONGIBS
    State {
        sprite: SPR_POB2,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_SMALLPOOL
    State {
        sprite: SPR_BRS1,
        frame: 0,
        tics: -1,
        action: None,
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }, // S_BRAINSTEM
    State {
        sprite: SPR_TLMP,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_TECHLAMP2,
        misc1: 0,
        misc2: 0,
    }, // S_TECHLAMP
    State {
        sprite: SPR_TLMP,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_TECHLAMP3,
        misc1: 0,
        misc2: 0,
    }, // S_TECHLAMP2
    State {
        sprite: SPR_TLMP,
        frame: 32770,
        tics: 4,
        action: None,
        nextstate: S_TECHLAMP4,
        misc1: 0,
        misc2: 0,
    }, // S_TECHLAMP3
    State {
        sprite: SPR_TLMP,
        frame: 32771,
        tics: 4,
        action: None,
        nextstate: S_TECHLAMP,
        misc1: 0,
        misc2: 0,
    }, // S_TECHLAMP4
    State {
        sprite: SPR_TLP2,
        frame: 32768,
        tics: 4,
        action: None,
        nextstate: S_TECH2LAMP2,
        misc1: 0,
        misc2: 0,
    }, // S_TECH2LAMP
    State {
        sprite: SPR_TLP2,
        frame: 32769,
        tics: 4,
        action: None,
        nextstate: S_TECH2LAMP3,
        misc1: 0,
        misc2: 0,
    }, // S_TECH2LAMP2
    State {
        sprite: SPR_TLP2,
        frame: 32770,
        tics: 4,
        action: None,
        nextstate: S_TECH2LAMP4,
        misc1: 0,
        misc2: 0,
    }, // S_TECH2LAMP3
    State {
        sprite: SPR_TLP2,
        frame: 32771,
        tics: 4,
        action: None,
        nextstate: S_TECH2LAMP,
        misc1: 0,
        misc2: 0,
    }, // S_TECH2LAMP4
];

#[no_mangle]
pub static mut mobjinfo: [MobjInfo; NUMMOBJTYPES] = [
    MobjInfo {
        doomednum: -1,
        spawnstate: S_PLAY,
        spawnhealth: 100,
        seestate: S_PLAY_RUN1,
        seesound: sfx_None,
        reactiontime: 0,
        attacksound: sfx_None,
        painstate: S_PLAY_PAIN,
        painchance: 255,
        painsound: sfx_plpain,
        meleestate: S_NULL,
        missilestate: S_PLAY_ATK1,
        deathstate: S_PLAY_DIE1,
        xdeathstate: S_PLAY_XDIE1,
        deathsound: sfx_pldeth,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 56 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID | MF_SHOOTABLE | MF_DROPOFF | MF_PICKUP | MF_NOTDMATCH,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 3004,
        spawnstate: S_POSS_STND,
        spawnhealth: 20,
        seestate: S_POSS_RUN1,
        seesound: sfx_posit1,
        reactiontime: 8,
        attacksound: sfx_pistol,
        painstate: S_POSS_PAIN,
        painchance: 200,
        painsound: sfx_popain,
        meleestate: S_NULL,
        missilestate: S_POSS_ATK1,
        deathstate: S_POSS_DIE1,
        xdeathstate: S_POSS_XDIE1,
        deathsound: sfx_podth1,
        speed: 8,
        radius: 20 * FRACUNIT,
        height: 56 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_posact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
        raisestate: S_POSS_RAISE1,
    },
    MobjInfo {
        doomednum: 9,
        spawnstate: S_SPOS_STND,
        spawnhealth: 30,
        seestate: S_SPOS_RUN1,
        seesound: sfx_posit2,
        reactiontime: 8,
        attacksound: 0,
        painstate: S_SPOS_PAIN,
        painchance: 170,
        painsound: sfx_popain,
        meleestate: S_NULL,
        missilestate: S_SPOS_ATK1,
        deathstate: S_SPOS_DIE1,
        xdeathstate: S_SPOS_XDIE1,
        deathsound: sfx_podth2,
        speed: 8,
        radius: 20 * FRACUNIT,
        height: 56 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_posact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
        raisestate: S_SPOS_RAISE1,
    },
    MobjInfo {
        doomednum: 64,
        spawnstate: S_VILE_STND,
        spawnhealth: 700,
        seestate: S_VILE_RUN1,
        seesound: sfx_vilsit,
        reactiontime: 8,
        attacksound: 0,
        painstate: S_VILE_PAIN,
        painchance: 10,
        painsound: sfx_vipain,
        meleestate: S_NULL,
        missilestate: S_VILE_ATK1,
        deathstate: S_VILE_DIE1,
        xdeathstate: S_NULL,
        deathsound: sfx_vildth,
        speed: 15,
        radius: 20 * FRACUNIT,
        height: 56 * FRACUNIT,
        mass: 500,
        damage: 0,
        activesound: sfx_vilact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_FIRE1,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 66,
        spawnstate: S_SKEL_STND,
        spawnhealth: 300,
        seestate: S_SKEL_RUN1,
        seesound: sfx_skesit,
        reactiontime: 8,
        attacksound: 0,
        painstate: S_SKEL_PAIN,
        painchance: 100,
        painsound: sfx_popain,
        meleestate: S_SKEL_FIST1,
        missilestate: S_SKEL_MISS1,
        deathstate: S_SKEL_DIE1,
        xdeathstate: S_NULL,
        deathsound: sfx_skedth,
        speed: 10,
        radius: 20 * FRACUNIT,
        height: 56 * FRACUNIT,
        mass: 500,
        damage: 0,
        activesound: sfx_skeact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
        raisestate: S_SKEL_RAISE1,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_TRACER,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_skeatk,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_TRACEEXP1,
        xdeathstate: S_NULL,
        deathsound: sfx_barexp,
        speed: 10 * FRACUNIT,
        radius: 11 * FRACUNIT,
        height: 8 * FRACUNIT,
        mass: 100,
        damage: 10,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_SMOKE1,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 67,
        spawnstate: S_FATT_STND,
        spawnhealth: 600,
        seestate: S_FATT_RUN1,
        seesound: sfx_mansit,
        reactiontime: 8,
        attacksound: 0,
        painstate: S_FATT_PAIN,
        painchance: 80,
        painsound: sfx_mnpain,
        meleestate: S_NULL,
        missilestate: S_FATT_ATK1,
        deathstate: S_FATT_DIE1,
        xdeathstate: S_NULL,
        deathsound: sfx_mandth,
        speed: 8,
        radius: 48 * FRACUNIT,
        height: 64 * FRACUNIT,
        mass: 1000,
        damage: 0,
        activesound: sfx_posact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
        raisestate: S_FATT_RAISE1,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_FATSHOT1,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_firsht,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_FATSHOTX1,
        xdeathstate: S_NULL,
        deathsound: sfx_firxpl,
        speed: 20 * FRACUNIT,
        radius: 6 * FRACUNIT,
        height: 8 * FRACUNIT,
        mass: 100,
        damage: 8,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 65,
        spawnstate: S_CPOS_STND,
        spawnhealth: 70,
        seestate: S_CPOS_RUN1,
        seesound: sfx_posit2,
        reactiontime: 8,
        attacksound: 0,
        painstate: S_CPOS_PAIN,
        painchance: 170,
        painsound: sfx_popain,
        meleestate: S_NULL,
        missilestate: S_CPOS_ATK1,
        deathstate: S_CPOS_DIE1,
        xdeathstate: S_CPOS_XDIE1,
        deathsound: sfx_podth2,
        speed: 8,
        radius: 20 * FRACUNIT,
        height: 56 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_posact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
        raisestate: S_CPOS_RAISE1,
    },
    MobjInfo {
        doomednum: 3001,
        spawnstate: S_TROO_STND,
        spawnhealth: 60,
        seestate: S_TROO_RUN1,
        seesound: sfx_bgsit1,
        reactiontime: 8,
        attacksound: 0,
        painstate: S_TROO_PAIN,
        painchance: 200,
        painsound: sfx_popain,
        meleestate: S_TROO_ATK1,
        missilestate: S_TROO_ATK1,
        deathstate: S_TROO_DIE1,
        xdeathstate: S_TROO_XDIE1,
        deathsound: sfx_bgdth1,
        speed: 8,
        radius: 20 * FRACUNIT,
        height: 56 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_bgact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
        raisestate: S_TROO_RAISE1,
    },
    MobjInfo {
        doomednum: 3002,
        spawnstate: S_SARG_STND,
        spawnhealth: 150,
        seestate: S_SARG_RUN1,
        seesound: sfx_sgtsit,
        reactiontime: 8,
        attacksound: sfx_sgtatk,
        painstate: S_SARG_PAIN,
        painchance: 180,
        painsound: sfx_dmpain,
        meleestate: S_SARG_ATK1,
        missilestate: S_NULL,
        deathstate: S_SARG_DIE1,
        xdeathstate: S_NULL,
        deathsound: sfx_sgtdth,
        speed: 10,
        radius: 30 * FRACUNIT,
        height: 56 * FRACUNIT,
        mass: 400,
        damage: 0,
        activesound: sfx_dmact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
        raisestate: S_SARG_RAISE1,
    },
    MobjInfo {
        doomednum: 58,
        spawnstate: S_SARG_STND,
        spawnhealth: 150,
        seestate: S_SARG_RUN1,
        seesound: sfx_sgtsit,
        reactiontime: 8,
        attacksound: sfx_sgtatk,
        painstate: S_SARG_PAIN,
        painchance: 180,
        painsound: sfx_dmpain,
        meleestate: S_SARG_ATK1,
        missilestate: S_NULL,
        deathstate: S_SARG_DIE1,
        xdeathstate: S_NULL,
        deathsound: sfx_sgtdth,
        speed: 10,
        radius: 30 * FRACUNIT,
        height: 56 * FRACUNIT,
        mass: 400,
        damage: 0,
        activesound: sfx_dmact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_SHADOW | MF_COUNTKILL,
        raisestate: S_SARG_RAISE1,
    },
    MobjInfo {
        doomednum: 3005,
        spawnstate: S_HEAD_STND,
        spawnhealth: 400,
        seestate: S_HEAD_RUN1,
        seesound: sfx_cacsit,
        reactiontime: 8,
        attacksound: 0,
        painstate: S_HEAD_PAIN,
        painchance: 128,
        painsound: sfx_dmpain,
        meleestate: S_NULL,
        missilestate: S_HEAD_ATK1,
        deathstate: S_HEAD_DIE1,
        xdeathstate: S_NULL,
        deathsound: sfx_cacdth,
        speed: 8,
        radius: 31 * FRACUNIT,
        height: 56 * FRACUNIT,
        mass: 400,
        damage: 0,
        activesound: sfx_dmact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_FLOAT | MF_NOGRAVITY | MF_COUNTKILL,
        raisestate: S_HEAD_RAISE1,
    },
    MobjInfo {
        doomednum: 3003,
        spawnstate: S_BOSS_STND,
        spawnhealth: 1000,
        seestate: S_BOSS_RUN1,
        seesound: sfx_brssit,
        reactiontime: 8,
        attacksound: 0,
        painstate: S_BOSS_PAIN,
        painchance: 50,
        painsound: sfx_dmpain,
        meleestate: S_BOSS_ATK1,
        missilestate: S_BOSS_ATK1,
        deathstate: S_BOSS_DIE1,
        xdeathstate: S_NULL,
        deathsound: sfx_brsdth,
        speed: 8,
        radius: 24 * FRACUNIT,
        height: 64 * FRACUNIT,
        mass: 1000,
        damage: 0,
        activesound: sfx_dmact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
        raisestate: S_BOSS_RAISE1,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_BRBALL1,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_firsht,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_BRBALLX1,
        xdeathstate: S_NULL,
        deathsound: sfx_firxpl,
        speed: 15 * FRACUNIT,
        radius: 6 * FRACUNIT,
        height: 8 * FRACUNIT,
        mass: 100,
        damage: 8,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 69,
        spawnstate: S_BOS2_STND,
        spawnhealth: 500,
        seestate: S_BOS2_RUN1,
        seesound: sfx_kntsit,
        reactiontime: 8,
        attacksound: 0,
        painstate: S_BOS2_PAIN,
        painchance: 50,
        painsound: sfx_dmpain,
        meleestate: S_BOS2_ATK1,
        missilestate: S_BOS2_ATK1,
        deathstate: S_BOS2_DIE1,
        xdeathstate: S_NULL,
        deathsound: sfx_kntdth,
        speed: 8,
        radius: 24 * FRACUNIT,
        height: 64 * FRACUNIT,
        mass: 1000,
        damage: 0,
        activesound: sfx_dmact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
        raisestate: S_BOS2_RAISE1,
    },
    MobjInfo {
        doomednum: 3006,
        spawnstate: S_SKULL_STND,
        spawnhealth: 100,
        seestate: S_SKULL_RUN1,
        seesound: 0,
        reactiontime: 8,
        attacksound: sfx_sklatk,
        painstate: S_SKULL_PAIN,
        painchance: 256,
        painsound: sfx_dmpain,
        meleestate: S_NULL,
        missilestate: S_SKULL_ATK1,
        deathstate: S_SKULL_DIE1,
        xdeathstate: S_NULL,
        deathsound: sfx_firxpl,
        speed: 8,
        radius: 16 * FRACUNIT,
        height: 56 * FRACUNIT,
        mass: 50,
        damage: 3,
        activesound: sfx_dmact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_FLOAT | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 7,
        spawnstate: S_SPID_STND,
        spawnhealth: 3000,
        seestate: S_SPID_RUN1,
        seesound: sfx_spisit,
        reactiontime: 8,
        attacksound: sfx_shotgn,
        painstate: S_SPID_PAIN,
        painchance: 40,
        painsound: sfx_dmpain,
        meleestate: S_NULL,
        missilestate: S_SPID_ATK1,
        deathstate: S_SPID_DIE1,
        xdeathstate: S_NULL,
        deathsound: sfx_spidth,
        speed: 12,
        radius: 128 * FRACUNIT,
        height: 100 * FRACUNIT,
        mass: 1000,
        damage: 0,
        activesound: sfx_dmact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 68,
        spawnstate: S_BSPI_STND,
        spawnhealth: 500,
        seestate: S_BSPI_SIGHT,
        seesound: sfx_bspsit,
        reactiontime: 8,
        attacksound: 0,
        painstate: S_BSPI_PAIN,
        painchance: 128,
        painsound: sfx_dmpain,
        meleestate: S_NULL,
        missilestate: S_BSPI_ATK1,
        deathstate: S_BSPI_DIE1,
        xdeathstate: S_NULL,
        deathsound: sfx_bspdth,
        speed: 12,
        radius: 64 * FRACUNIT,
        height: 64 * FRACUNIT,
        mass: 600,
        damage: 0,
        activesound: sfx_bspact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
        raisestate: S_BSPI_RAISE1,
    },
    MobjInfo {
        doomednum: 16,
        spawnstate: S_CYBER_STND,
        spawnhealth: 4000,
        seestate: S_CYBER_RUN1,
        seesound: sfx_cybsit,
        reactiontime: 8,
        attacksound: 0,
        painstate: S_CYBER_PAIN,
        painchance: 20,
        painsound: sfx_dmpain,
        meleestate: S_NULL,
        missilestate: S_CYBER_ATK1,
        deathstate: S_CYBER_DIE1,
        xdeathstate: S_NULL,
        deathsound: sfx_cybdth,
        speed: 16,
        radius: 40 * FRACUNIT,
        height: 110 * FRACUNIT,
        mass: 1000,
        damage: 0,
        activesound: sfx_dmact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 71,
        spawnstate: S_PAIN_STND,
        spawnhealth: 400,
        seestate: S_PAIN_RUN1,
        seesound: sfx_pesit,
        reactiontime: 8,
        attacksound: 0,
        painstate: S_PAIN_PAIN,
        painchance: 128,
        painsound: sfx_pepain,
        meleestate: S_NULL,
        missilestate: S_PAIN_ATK1,
        deathstate: S_PAIN_DIE1,
        xdeathstate: S_NULL,
        deathsound: sfx_pedth,
        speed: 8,
        radius: 31 * FRACUNIT,
        height: 56 * FRACUNIT,
        mass: 400,
        damage: 0,
        activesound: sfx_dmact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_FLOAT | MF_NOGRAVITY | MF_COUNTKILL,
        raisestate: S_PAIN_RAISE1,
    },
    MobjInfo {
        doomednum: 84,
        spawnstate: S_SSWV_STND,
        spawnhealth: 50,
        seestate: S_SSWV_RUN1,
        seesound: sfx_sssit,
        reactiontime: 8,
        attacksound: 0,
        painstate: S_SSWV_PAIN,
        painchance: 170,
        painsound: sfx_popain,
        meleestate: S_NULL,
        missilestate: S_SSWV_ATK1,
        deathstate: S_SSWV_DIE1,
        xdeathstate: S_SSWV_XDIE1,
        deathsound: sfx_ssdth,
        speed: 8,
        radius: 20 * FRACUNIT,
        height: 56 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_posact,
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
        raisestate: S_SSWV_RAISE1,
    },
    MobjInfo {
        doomednum: 72,
        spawnstate: S_KEENSTND,
        spawnhealth: 100,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_KEENPAIN,
        painchance: 256,
        painsound: sfx_keenpn,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_COMMKEEN,
        xdeathstate: S_NULL,
        deathsound: sfx_keendt,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 72 * FRACUNIT,
        mass: 10000000,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY | MF_SHOOTABLE | MF_COUNTKILL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 88,
        spawnstate: S_BRAIN,
        spawnhealth: 250,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_BRAIN_PAIN,
        painchance: 255,
        painsound: sfx_bospn,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_BRAIN_DIE1,
        xdeathstate: S_NULL,
        deathsound: sfx_bosdth,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 10000000,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID | MF_SHOOTABLE,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 89,
        spawnstate: S_BRAINEYE,
        spawnhealth: 1000,
        seestate: S_BRAINEYESEE,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 32 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_NOSECTOR,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 87,
        spawnstate: S_NULL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 32 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_NOSECTOR,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_SPAWN1,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_bospit,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_firxpl,
        speed: 10 * FRACUNIT,
        radius: 6 * FRACUNIT,
        height: 32 * FRACUNIT,
        mass: 100,
        damage: 3,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY | MF_NOCLIP,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_SPAWNFIRE1,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2035,
        spawnstate: S_BAR1,
        spawnhealth: 20,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_BEXP,
        xdeathstate: S_NULL,
        deathsound: sfx_barexp,
        speed: 0,
        radius: 10 * FRACUNIT,
        height: 42 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID | MF_SHOOTABLE | MF_NOBLOOD,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_TBALL1,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_firsht,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_TBALLX1,
        xdeathstate: S_NULL,
        deathsound: sfx_firxpl,
        speed: 10 * FRACUNIT,
        radius: 6 * FRACUNIT,
        height: 8 * FRACUNIT,
        mass: 100,
        damage: 3,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_RBALL1,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_firsht,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_RBALLX1,
        xdeathstate: S_NULL,
        deathsound: sfx_firxpl,
        speed: 10 * FRACUNIT,
        radius: 6 * FRACUNIT,
        height: 8 * FRACUNIT,
        mass: 100,
        damage: 5,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_ROCKET,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_rlaunc,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_EXPLODE1,
        xdeathstate: S_NULL,
        deathsound: sfx_barexp,
        speed: 20 * FRACUNIT,
        radius: 11 * FRACUNIT,
        height: 8 * FRACUNIT,
        mass: 100,
        damage: 20,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_PLASBALL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_plasma,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_PLASEXP,
        xdeathstate: S_NULL,
        deathsound: sfx_firxpl,
        speed: 25 * FRACUNIT,
        radius: 13 * FRACUNIT,
        height: 8 * FRACUNIT,
        mass: 100,
        damage: 5,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_BFGSHOT,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: 0,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_BFGLAND,
        xdeathstate: S_NULL,
        deathsound: sfx_rxplod,
        speed: 25 * FRACUNIT,
        radius: 13 * FRACUNIT,
        height: 8 * FRACUNIT,
        mass: 100,
        damage: 100,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_ARACH_PLAZ,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_plasma,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_ARACH_PLEX,
        xdeathstate: S_NULL,
        deathsound: sfx_firxpl,
        speed: 25 * FRACUNIT,
        radius: 13 * FRACUNIT,
        height: 8 * FRACUNIT,
        mass: 100,
        damage: 5,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_PUFF1,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_BLOOD1,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_TFOG,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_IFOG,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 14,
        spawnstate: S_NULL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_NOSECTOR,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: -1,
        spawnstate: S_BFGEXP,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2018,
        spawnstate: S_ARM1,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2019,
        spawnstate: S_ARM2,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2014,
        spawnstate: S_BON1,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL | MF_COUNTITEM,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2015,
        spawnstate: S_BON2,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL | MF_COUNTITEM,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 5,
        spawnstate: S_BKEY,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL | MF_NOTDMATCH,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 13,
        spawnstate: S_RKEY,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL | MF_NOTDMATCH,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 6,
        spawnstate: S_YKEY,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL | MF_NOTDMATCH,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 39,
        spawnstate: S_YSKULL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL | MF_NOTDMATCH,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 38,
        spawnstate: S_RSKULL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL | MF_NOTDMATCH,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 40,
        spawnstate: S_BSKULL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL | MF_NOTDMATCH,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2011,
        spawnstate: S_STIM,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2012,
        spawnstate: S_MEDI,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2013,
        spawnstate: S_SOUL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL | MF_COUNTITEM,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2022,
        spawnstate: S_PINV,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL | MF_COUNTITEM,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2023,
        spawnstate: S_PSTR,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL | MF_COUNTITEM,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2024,
        spawnstate: S_PINS,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL | MF_COUNTITEM,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2025,
        spawnstate: S_SUIT,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2026,
        spawnstate: S_PMAP,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL | MF_COUNTITEM,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2045,
        spawnstate: S_PVIS,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL | MF_COUNTITEM,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 83,
        spawnstate: S_MEGA,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL | MF_COUNTITEM,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2007,
        spawnstate: S_CLIP,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2048,
        spawnstate: S_AMMO,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2010,
        spawnstate: S_ROCK,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2046,
        spawnstate: S_BROK,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2047,
        spawnstate: S_CELL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 17,
        spawnstate: S_CELP,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2008,
        spawnstate: S_SHEL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2049,
        spawnstate: S_SBOX,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 8,
        spawnstate: S_BPAK,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2006,
        spawnstate: S_BFUG,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2002,
        spawnstate: S_MGUN,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2005,
        spawnstate: S_CSAW,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2003,
        spawnstate: S_LAUN,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2004,
        spawnstate: S_PLAS,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2001,
        spawnstate: S_SHOT,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 82,
        spawnstate: S_SHOT2,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPECIAL,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 85,
        spawnstate: S_TECHLAMP,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 86,
        spawnstate: S_TECH2LAMP,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 2028,
        spawnstate: S_COLU,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 30,
        spawnstate: S_TALLGRNCOL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 31,
        spawnstate: S_SHRTGRNCOL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 32,
        spawnstate: S_TALLREDCOL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 33,
        spawnstate: S_SHRTREDCOL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 37,
        spawnstate: S_SKULLCOL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 36,
        spawnstate: S_HEARTCOL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 41,
        spawnstate: S_EVILEYE,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 42,
        spawnstate: S_FLOATSKULL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 43,
        spawnstate: S_TORCHTREE,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 44,
        spawnstate: S_BLUETORCH,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 45,
        spawnstate: S_GREENTORCH,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 46,
        spawnstate: S_REDTORCH,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 55,
        spawnstate: S_BTORCHSHRT,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 56,
        spawnstate: S_GTORCHSHRT,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 57,
        spawnstate: S_RTORCHSHRT,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 47,
        spawnstate: S_STALAGTITE,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 48,
        spawnstate: S_TECHPILLAR,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 34,
        spawnstate: S_CANDLESTIK,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: 0,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 35,
        spawnstate: S_CANDELABRA,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 49,
        spawnstate: S_BLOODYTWITCH,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 68 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 50,
        spawnstate: S_MEAT2,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 84 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 51,
        spawnstate: S_MEAT3,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 84 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 52,
        spawnstate: S_MEAT4,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 68 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 53,
        spawnstate: S_MEAT5,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 52 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 59,
        spawnstate: S_MEAT2,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 84 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 60,
        spawnstate: S_MEAT4,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 68 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 61,
        spawnstate: S_MEAT3,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 52 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 62,
        spawnstate: S_MEAT5,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 52 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 63,
        spawnstate: S_BLOODYTWITCH,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 68 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 22,
        spawnstate: S_HEAD_DIE6,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: 0,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 15,
        spawnstate: S_PLAY_DIE7,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: 0,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 18,
        spawnstate: S_POSS_DIE5,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: 0,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 21,
        spawnstate: S_SARG_DIE6,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: 0,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 23,
        spawnstate: S_SKULL_DIE6,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: 0,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 20,
        spawnstate: S_TROO_DIE5,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: 0,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 19,
        spawnstate: S_SPOS_DIE5,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: 0,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 10,
        spawnstate: S_PLAY_XDIE9,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: 0,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 12,
        spawnstate: S_PLAY_XDIE9,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: 0,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 28,
        spawnstate: S_HEADSONSTICK,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 24,
        spawnstate: S_GIBS,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: 0,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 27,
        spawnstate: S_HEADONASTICK,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 29,
        spawnstate: S_HEADCANDLES,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 25,
        spawnstate: S_DEADSTICK,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 26,
        spawnstate: S_LIVESTICK,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 54,
        spawnstate: S_BIGTREE,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 32 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 70,
        spawnstate: S_BBAR1,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 73,
        spawnstate: S_HANGNOGUTS,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 88 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 74,
        spawnstate: S_HANGBNOBRAIN,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 88 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 75,
        spawnstate: S_HANGTLOOKDN,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 64 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 76,
        spawnstate: S_HANGTSKULL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 64 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 77,
        spawnstate: S_HANGTLOOKUP,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 64 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 78,
        spawnstate: S_HANGTNOBRAIN,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 16 * FRACUNIT,
        height: 64 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 79,
        spawnstate: S_COLONGIBS,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 80,
        spawnstate: S_SMALLPOOL,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP,
        raisestate: S_NULL,
    },
    MobjInfo {
        doomednum: 81,
        spawnstate: S_BRAINSTEM,
        spawnhealth: 1000,
        seestate: S_NULL,
        seesound: sfx_None,
        reactiontime: 8,
        attacksound: sfx_None,
        painstate: S_NULL,
        painchance: 0,
        painsound: sfx_None,
        meleestate: S_NULL,
        missilestate: S_NULL,
        deathstate: S_NULL,
        xdeathstate: S_NULL,
        deathsound: sfx_None,
        speed: 0,
        radius: 20 * FRACUNIT,
        height: 16 * FRACUNIT,
        mass: 100,
        damage: 0,
        activesound: sfx_None,
        flags: MF_NOBLOCKMAP,
        raisestate: S_NULL,
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// Parse `vendor/doomgeneric/info.h` and extract the `mobjtype_t` enum
    /// values, then assert that every Rust `MT_*` constant matches the C
    /// source exactly.  This prevents ordering drift: if a new entry is
    /// inserted into the middle of the enum, the index-based Rust constants
    /// would silently shift every subsequent value.
    #[test]
    fn mt_enum_matches_c_header() {
        let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = std::path::Path::new(&manifest).join("../vendor/doomgeneric/info.h");
        let h = fs::read_to_string(path).unwrap();
        let mut found = Vec::new();
        let mut inside = false;
        for line in h.lines() {
            if line.contains("MT_PLAYER,") {
                inside = true;
            }
            if inside {
                if let Some(name) = line.trim().strip_suffix(",") {
                    found.push(name.to_string());
                } else if line.trim() == "} mobjtype_t;" {
                    break;
                }
            }
        }
        assert!(
            !found.is_empty(),
            "could not find mobjtype_t enum in info.h"
        );
        for (i, name) in found.iter().enumerate() {
            let expected = i as c_int;
            let actual = match name.as_str() {
                "MT_PLAYER" => MT_PLAYER,
                "MT_POSSESSED" => MT_POSSESSED,
                "MT_SHOTGUY" => MT_SHOTGUY,
                "MT_VILE" => MT_VILE,
                "MT_FIRE" => MT_FIRE,
                "MT_UNDEAD" => MT_UNDEAD,
                "MT_TRACER" => MT_TRACER,
                "MT_SMOKE" => MT_SMOKE,
                "MT_FATSO" => MT_FATSO,
                "MT_FATSHOT" => MT_FATSHOT,
                "MT_CHAINGUY" => MT_CHAINGUY,
                "MT_TROOP" => MT_TROOP,
                "MT_SERGEANT" => MT_SERGEANT,
                "MT_SHADOWS" => MT_SHADOWS,
                "MT_HEAD" => MT_HEAD,
                "MT_BRUISER" => MT_BRUISER,
                "MT_BRUISERSHOT" => MT_BRUISERSHOT,
                "MT_KNIGHT" => MT_KNIGHT,
                "MT_SKULL" => MT_SKULL,
                "MT_SPIDER" => MT_SPIDER,
                "MT_BABY" => MT_BABY,
                "MT_CYBORG" => MT_CYBORG,
                "MT_PAIN" => MT_PAIN,
                "MT_WOLFSS" => MT_WOLFSS,
                "MT_KEEN" => MT_KEEN,
                "MT_BOSSBRAIN" => MT_BOSSBRAIN,
                "MT_BOSSSPIT" => MT_BOSSSPIT,
                "MT_BOSSTARGET" => MT_BOSSTARGET,
                "MT_SPAWNSHOT" => MT_SPAWNSHOT,
                "MT_SPAWNFIRE" => MT_SPAWNFIRE,
                "MT_BARREL" => MT_BARREL,
                "MT_TROOPSHOT" => MT_TROOPSHOT,
                "MT_HEADSHOT" => MT_HEADSHOT,
                "MT_ROCKET" => MT_ROCKET,
                "MT_PLASMA" => MT_PLASMA,
                "MT_BFG" => MT_BFG,
                "MT_ARACHPLAZ" => MT_ARACHPLAZ,
                "MT_PUFF" => MT_PUFF,
                "MT_BLOOD" => MT_BLOOD,
                "MT_TFOG" => MT_TFOG,
                "MT_IFOG" => MT_IFOG,
                "MT_TELEPORTMAN" => MT_TELEPORTMAN,
                "MT_EXTRABFG" => MT_EXTRABFG,
                "MT_MISC0" => MT_MISC0,
                "MT_MISC1" => MT_MISC1,
                "MT_MISC2" => MT_MISC2,
                "MT_MISC3" => MT_MISC3,
                "MT_MISC4" => MT_MISC4,
                "MT_MISC5" => MT_MISC5,
                "MT_MISC6" => MT_MISC6,
                "MT_MISC7" => MT_MISC7,
                "MT_MISC8" => MT_MISC8,
                "MT_MISC9" => MT_MISC9,
                "MT_MISC10" => MT_MISC10,
                "MT_MISC11" => MT_MISC11,
                "MT_MISC12" => MT_MISC12,
                "MT_INV" => MT_INV,
                "MT_MISC13" => MT_MISC13,
                "MT_INS" => MT_INS,
                "MT_MISC14" => MT_MISC14,
                "MT_MISC15" => MT_MISC15,
                "MT_MISC16" => MT_MISC16,
                "MT_MEGA" => MT_MEGA,
                "MT_CLIP" => MT_CLIP,
                "MT_MISC17" => MT_MISC17,
                "MT_MISC18" => MT_MISC18,
                "MT_MISC19" => MT_MISC19,
                "MT_MISC20" => MT_MISC20,
                "MT_MISC21" => MT_MISC21,
                "MT_MISC22" => MT_MISC22,
                "MT_MISC23" => MT_MISC23,
                "MT_MISC24" => MT_MISC24,
                "MT_MISC25" => MT_MISC25,
                "MT_CHAINGUN" => MT_CHAINGUN,
                "MT_MISC26" => MT_MISC26,
                "MT_MISC27" => MT_MISC27,
                "MT_MISC28" => MT_MISC28,
                "MT_SHOTGUN" => MT_SHOTGUN,
                "MT_SUPERSHOTGUN" => MT_SUPERSHOTGUN,
                "MT_MISC29" => MT_MISC29,
                "MT_MISC30" => MT_MISC30,
                "MT_MISC31" => MT_MISC31,
                "MT_MISC32" => MT_MISC32,
                "MT_MISC33" => MT_MISC33,
                "MT_MISC34" => MT_MISC34,
                "MT_MISC35" => MT_MISC35,
                "MT_MISC36" => MT_MISC36,
                "MT_MISC37" => MT_MISC37,
                "MT_MISC38" => MT_MISC38,
                "MT_MISC39" => MT_MISC39,
                "MT_MISC40" => MT_MISC40,
                "MT_MISC41" => MT_MISC41,
                "MT_MISC42" => MT_MISC42,
                "MT_MISC43" => MT_MISC43,
                "MT_MISC44" => MT_MISC44,
                "MT_MISC45" => MT_MISC45,
                "MT_MISC46" => MT_MISC46,
                "MT_MISC47" => MT_MISC47,
                "MT_MISC48" => MT_MISC48,
                "MT_MISC49" => MT_MISC49,
                "MT_MISC50" => MT_MISC50,
                "MT_MISC51" => MT_MISC51,
                "MT_MISC52" => MT_MISC52,
                "MT_MISC53" => MT_MISC53,
                "MT_MISC54" => MT_MISC54,
                "MT_MISC55" => MT_MISC55,
                "MT_MISC56" => MT_MISC56,
                "MT_MISC57" => MT_MISC57,
                "MT_MISC58" => MT_MISC58,
                "MT_MISC59" => MT_MISC59,
                "MT_MISC60" => MT_MISC60,
                "MT_MISC61" => MT_MISC61,
                "MT_MISC62" => MT_MISC62,
                "MT_MISC63" => MT_MISC63,
                "MT_MISC64" => MT_MISC64,
                "MT_MISC65" => MT_MISC65,
                "MT_MISC66" => MT_MISC66,
                "MT_MISC67" => MT_MISC67,
                "MT_MISC68" => MT_MISC68,
                "MT_MISC69" => MT_MISC69,
                "MT_MISC70" => MT_MISC70,
                "MT_MISC71" => MT_MISC71,
                "MT_MISC72" => MT_MISC72,
                "MT_MISC73" => MT_MISC73,
                "MT_MISC74" => MT_MISC74,
                "MT_MISC75" => MT_MISC75,
                "MT_MISC76" => MT_MISC76,
                "MT_MISC77" => MT_MISC77,
                "MT_MISC78" => MT_MISC78,
                "MT_MISC79" => MT_MISC79,
                "MT_MISC80" => MT_MISC80,
                "MT_MISC81" => MT_MISC81,
                "MT_MISC82" => MT_MISC82,
                "MT_MISC83" => MT_MISC83,
                "MT_MISC84" => MT_MISC84,
                "MT_MISC85" => MT_MISC85,
                "MT_MISC86" => MT_MISC86,
                _ => panic!("unknown mobjtype enum member: {}", name),
            };
            assert_eq!(
                actual, expected,
                "{} should be {} (C enum order)",
                name, expected
            );
        }
        assert_eq!(
            found.len() as c_int,
            NUMMOBJTYPES as c_int,
            "mobjtype_t enum size mismatch"
        );
    }

    /// Parse `vendor/doomgeneric/p_mobj.h` and extract the `mobjflag_t` enum
    /// values, then assert that every Rust `MF_*` constant matches the C
    /// source exactly.  A mismatch here silently corrupts every `flags & MF_*`
    /// test in the game because `mobjinfo[].flags` is initialised with these
    /// bit patterns.
    #[test]
    fn mf_flags_match_c_header() {
        let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = std::path::Path::new(&manifest).join("../vendor/doomgeneric/p_mobj.h");
        let h = fs::read_to_string(path).unwrap();
        let mut inside = false;
        for line in h.lines() {
            if line.contains("typedef enum") && !inside {
                // The first enum in p_mobj.h is mobjflag_t
                inside = true;
                continue;
            }
            if inside {
                if line.trim() == "} mobjflag_t;" {
                    break;
                }
                let trimmed = line.trim();
                if trimmed.starts_with("MF_") {
                    let parts: Vec<&str> = trimmed.split('=').collect();
                    let name = parts[0].trim();
                    let val_str = parts[1].trim().trim_end_matches(',');
                    let expected = if val_str.starts_with("0x") {
                        c_int::from_str_radix(&val_str[2..], 16).unwrap()
                    } else {
                        val_str.parse::<c_int>().unwrap()
                    };
                    let actual = match name {
                        "MF_SPECIAL" => MF_SPECIAL,
                        "MF_SOLID" => MF_SOLID,
                        "MF_SHOOTABLE" => MF_SHOOTABLE,
                        "MF_NOSECTOR" => MF_NOSECTOR,
                        "MF_NOBLOCKMAP" => MF_NOBLOCKMAP,
                        "MF_AMBUSH" => MF_AMBUSH,
                        "MF_JUSTHIT" => MF_JUSTHIT,
                        "MF_JUSTATTACKED" => MF_JUSTATTACKED,
                        "MF_SPAWNCEILING" => MF_SPAWNCEILING,
                        "MF_NOGRAVITY" => MF_NOGRAVITY,
                        "MF_DROPOFF" => MF_DROPOFF,
                        "MF_PICKUP" => MF_PICKUP,
                        "MF_NOCLIP" => MF_NOCLIP,
                        "MF_SLIDE" => MF_SLIDE,
                        "MF_FLOAT" => MF_FLOAT,
                        "MF_TELEPORT" => MF_TELEPORT,
                        "MF_MISSILE" => MF_MISSILE,
                        "MF_DROPPED" => MF_DROPPED,
                        "MF_SHADOW" => MF_SHADOW,
                        "MF_NOBLOOD" => MF_NOBLOOD,
                        "MF_CORPSE" => MF_CORPSE,
                        "MF_INFLOAT" => MF_INFLOAT,
                        "MF_COUNTKILL" => MF_COUNTKILL,
                        "MF_COUNTITEM" => MF_COUNTITEM,
                        "MF_SKULLFLY" => MF_SKULLFLY,
                        "MF_NOTDMATCH" => MF_NOTDMATCH,
                        "MF_TRANSLATION" => MF_TRANSLATION,
                        "MF_TRANSSHIFT" => MF_TRANSSHIFT,
                        _ => panic!("unknown flag: {}", name),
                    };
                    assert_eq!(actual, expected, "{} mismatch against p_mobj.h", name);
                }
            }
        }
    }

    /// Verify that the mobjinfo table is internally consistent:
    /// the spawnstate of each well-known mobj type should point to a state
    /// whose sprite matches the expected SPR_* constant.  This catches
    /// index-vs-table drift (e.g. MT_PUFF pointing at S_BLOOD1 because
    /// the MT enum or states table was edited incorrectly).
    #[test]
    fn mobjinfo_spawnstate_sprite_cross_reference() {
        unsafe {
            let cases: [(c_int, c_int, c_int, &str); 7] = [
                (MT_PUFF, S_PUFF1, SPR_PUFF, "MT_PUFF → S_PUFF1 → SPR_PUFF"),
                (
                    MT_BLOOD,
                    S_BLOOD1,
                    SPR_BLUD,
                    "MT_BLOOD → S_BLOOD1 → SPR_BLUD",
                ),
                (MT_TFOG, S_TFOG, SPR_TFOG, "MT_TFOG → S_TFOG → SPR_TFOG"),
                (MT_IFOG, S_IFOG, SPR_IFOG, "MT_IFOG → S_IFOG → SPR_IFOG"),
                (MT_PLAYER, S_PLAY, SPR_PLAY, "MT_PLAYER → S_PLAY → SPR_PLAY"),
                (
                    MT_ROCKET,
                    S_ROCKET,
                    SPR_MISL,
                    "MT_ROCKET → S_ROCKET → SPR_MISL",
                ),
                (
                    MT_PLASMA,
                    S_PLASBALL,
                    SPR_PLSS,
                    "MT_PLASMA → S_PLASBALL → SPR_PLSS",
                ),
            ];
            for (mt, expected_state, expected_sprite, desc) in cases {
                let info = &mobjinfo[mt as usize];
                assert_eq!(
                    info.spawnstate, expected_state,
                    "{}: spawnstate mismatch",
                    desc
                );
                let st = &states[info.spawnstate as usize];
                assert_eq!(st.sprite, expected_sprite, "{}: sprite mismatch", desc);
            }
        }
    }

    /// Verify that the mobjinfo table entries for items have the expected
    /// MF_SPECIAL flag.  MT_INV and MT_INS have MF_SPECIAL (they are
    /// pick-up items) but are explicitly excluded from the respawn queue
    /// in P_RemoveMobj.
    #[test]
    fn mobjinfo_respawn_flags_consistent() {
        unsafe {
            // Power-ups have MF_SPECIAL despite being excluded from respawn
            assert!(
                mobjinfo[MT_INV as usize].flags & MF_SPECIAL != 0,
                "MT_INV should have MF_SPECIAL"
            );
            assert!(
                mobjinfo[MT_INS as usize].flags & MF_SPECIAL != 0,
                "MT_INS should have MF_SPECIAL"
            );

            // A few representative items that should also have it
            assert!(
                mobjinfo[MT_CLIP as usize].flags & MF_SPECIAL != 0,
                "MT_CLIP should have MF_SPECIAL"
            );
            assert!(
                mobjinfo[MT_MISC10 as usize].flags & MF_SPECIAL != 0,
                "MT_MISC10 (stimpack) should have MF_SPECIAL"
            );
            assert!(
                mobjinfo[MT_MISC11 as usize].flags & MF_SPECIAL != 0,
                "MT_MISC11 (medikit) should have MF_SPECIAL"
            );
        }
    }
}
