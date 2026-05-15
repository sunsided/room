//! Rust port of vendor/doomgeneric/d_mode.c.
//!
//! Provides game mode/mission validation functions and string constants.

#![allow(non_snake_case, non_upper_case_globals)]

use std::ffi::{c_char, c_int};

use crate::types::Boolean;

pub const none: c_int = 9;
pub const doom: c_int = 0;
pub const doom2: c_int = 1;
pub const pack_tnt: c_int = 2;
pub const pack_plut: c_int = 3;
pub const pack_chex: c_int = 4;
pub const pack_hacx: c_int = 5;
pub const heretic: c_int = 6;
pub const hexen: c_int = 7;
pub const strife: c_int = 8;

pub const shareware: c_int = 0;
pub const registered: c_int = 1;
pub const commercial: c_int = 2;
pub const retail: c_int = 3;
pub const indetermined: c_int = 4;

pub const exe_doom_1_2: c_int = 0;
pub const exe_doom_1_666: c_int = 1;
pub const exe_doom_1_7: c_int = 2;
pub const exe_doom_1_8: c_int = 3;
pub const exe_doom_1_9: c_int = 4;
pub const exe_hacx: c_int = 5;
pub const exe_ultimate: c_int = 6;
pub const exe_final: c_int = 7;
pub const exe_final2: c_int = 8;
pub const exe_chex: c_int = 9;
pub const exe_heretic_1_3: c_int = 10;
pub const exe_hexen_1_1: c_int = 11;
pub const exe_strife_1_2: c_int = 12;
pub const exe_strife_1_31: c_int = 13;

struct ValidMode {
    mission: c_int,
    mode: c_int,
    episode: c_int,
    map: c_int,
}

static VALID_MODES: [ValidMode; 13] = [
    ValidMode {
        mission: pack_chex,
        mode: shareware,
        episode: 1,
        map: 5,
    },
    ValidMode {
        mission: doom,
        mode: shareware,
        episode: 1,
        map: 9,
    },
    ValidMode {
        mission: doom,
        mode: registered,
        episode: 3,
        map: 9,
    },
    ValidMode {
        mission: doom,
        mode: retail,
        episode: 4,
        map: 9,
    },
    ValidMode {
        mission: doom2,
        mode: commercial,
        episode: 1,
        map: 32,
    },
    ValidMode {
        mission: pack_tnt,
        mode: commercial,
        episode: 1,
        map: 32,
    },
    ValidMode {
        mission: pack_plut,
        mode: commercial,
        episode: 1,
        map: 32,
    },
    ValidMode {
        mission: pack_hacx,
        mode: commercial,
        episode: 1,
        map: 32,
    },
    ValidMode {
        mission: heretic,
        mode: shareware,
        episode: 1,
        map: 9,
    },
    ValidMode {
        mission: heretic,
        mode: registered,
        episode: 3,
        map: 9,
    },
    ValidMode {
        mission: heretic,
        mode: retail,
        episode: 5,
        map: 9,
    },
    ValidMode {
        mission: hexen,
        mode: commercial,
        episode: 1,
        map: 60,
    },
    ValidMode {
        mission: strife,
        mode: commercial,
        episode: 1,
        map: 34,
    },
];

struct ValidVersion {
    mission: c_int,
    version: c_int,
}

static VALID_VERSIONS: [ValidVersion; 10] = [
    ValidVersion {
        mission: doom,
        version: exe_doom_1_9,
    },
    ValidVersion {
        mission: doom,
        version: exe_hacx,
    },
    ValidVersion {
        mission: doom,
        version: exe_ultimate,
    },
    ValidVersion {
        mission: doom,
        version: exe_final,
    },
    ValidVersion {
        mission: doom,
        version: exe_final2,
    },
    ValidVersion {
        mission: doom,
        version: exe_chex,
    },
    ValidVersion {
        mission: heretic,
        version: exe_heretic_1_3,
    },
    ValidVersion {
        mission: hexen,
        version: exe_hexen_1_1,
    },
    ValidVersion {
        mission: strife,
        version: exe_strife_1_2,
    },
    ValidVersion {
        mission: strife,
        version: exe_strife_1_31,
    },
];

#[no_mangle]
pub extern "C" fn D_ValidGameMode(mission: c_int, mode: c_int) -> Boolean {
    for vm in &VALID_MODES {
        if vm.mission == mission && vm.mode == mode {
            return Boolean::TRUE;
        }
    }
    Boolean::FALSE
}

#[no_mangle]
pub extern "C" fn D_ValidEpisodeMap(
    mission: c_int,
    mode: c_int,
    episode: c_int,
    map: c_int,
) -> Boolean {
    // Hacks for Heretic secret episodes
    if mission == heretic {
        if mode == retail && episode == 6 {
            return Boolean::from((1..=3).contains(&map));
        } else if mode == registered && episode == 4 {
            return Boolean::from(map == 1);
        }
    }

    for vm in &VALID_MODES {
        if mission == vm.mission && mode == vm.mode {
            return Boolean::from(
                episode >= 1 && episode <= vm.episode && map >= 1 && map <= vm.map,
            );
        }
    }

    Boolean::FALSE
}

#[no_mangle]
pub extern "C" fn D_GetNumEpisodes(mission: c_int, mode: c_int) -> c_int {
    let mut episode = 1;
    while D_ValidEpisodeMap(mission, mode, episode, 1).is_truthy() {
        episode += 1;
    }
    episode - 1
}

#[no_mangle]
pub extern "C" fn D_ValidGameVersion(mission: c_int, version: c_int) -> Boolean {
    let mission = if mission == doom2
        || mission == pack_plut
        || mission == pack_tnt
        || mission == pack_hacx
        || mission == pack_chex
    {
        doom
    } else {
        mission
    };

    for vv in &VALID_VERSIONS {
        if vv.mission == mission && vv.version == version {
            return Boolean::TRUE;
        }
    }

    Boolean::FALSE
}

#[no_mangle]
pub extern "C" fn D_IsEpisodeMap(mission: c_int) -> Boolean {
    match mission {
        doom | heretic | pack_chex => Boolean::TRUE,
        _ => Boolean::FALSE,
    }
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *mut c_char
    };
}

#[no_mangle]
pub extern "C" fn D_GameMissionString(mission: c_int) -> *mut c_char {
    match mission {
        doom => cstr!("doom"),
        doom2 => cstr!("doom2"),
        pack_tnt => cstr!("tnt"),
        pack_plut => cstr!("plutonia"),
        pack_hacx => cstr!("hacx"),
        pack_chex => cstr!("chex"),
        heretic => cstr!("heretic"),
        hexen => cstr!("hexen"),
        strife => cstr!("strife"),
        _ => cstr!("none"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Boolean;
    use std::ffi::CStr;

    #[test]
    fn valid_game_mode_doom_shareware() {
        assert_eq!(D_ValidGameMode(doom, shareware), Boolean::TRUE);
    }

    #[test]
    fn valid_game_mode_doom2_shareware_invalid() {
        assert_eq!(D_ValidGameMode(doom2, shareware), Boolean::FALSE);
    }

    #[test]
    fn get_num_episodes_doom_retail() {
        assert_eq!(D_GetNumEpisodes(doom, retail), 4);
    }

    #[test]
    fn is_episode_map_doom2_false() {
        assert_eq!(D_IsEpisodeMap(doom2), Boolean::FALSE);
    }

    #[test]
    fn is_episode_map_doom_true() {
        assert_eq!(D_IsEpisodeMap(doom), Boolean::TRUE);
    }

    #[test]
    fn game_mission_string_heretic() {
        let ptr = D_GameMissionString(heretic);
        let s = unsafe { CStr::from_ptr(ptr) };
        assert_eq!(s.to_str().unwrap(), "heretic");
    }

    #[test]
    fn game_mission_string_unknown() {
        let ptr = D_GameMissionString(42);
        let s = unsafe { CStr::from_ptr(ptr) };
        assert_eq!(s.to_str().unwrap(), "none");
    }

    #[test]
    fn valid_game_version_doom_final2() {
        assert_eq!(D_ValidGameVersion(doom, exe_final2), Boolean::TRUE);
    }

    #[test]
    fn valid_game_version_doom2_mapped_to_doom() {
        assert_eq!(D_ValidGameVersion(doom2, exe_final2), Boolean::TRUE);
    }

    #[test]
    fn valid_episode_map_doom_retail_ep4_map9() {
        assert_eq!(D_ValidEpisodeMap(doom, retail, 4, 9), Boolean::TRUE);
    }

    #[test]
    fn valid_episode_map_doom_retail_ep5_map1_invalid() {
        assert_eq!(D_ValidEpisodeMap(doom, retail, 5, 1), Boolean::FALSE);
    }

    /// Heretic retail secret episode 6 allows maps 1-3 only.
    #[test]
    fn valid_episode_map_heretic_retail_ep6_maps_1_to_3() {
        assert_eq!(D_ValidEpisodeMap(heretic, retail, 6, 1), Boolean::TRUE);
        assert_eq!(D_ValidEpisodeMap(heretic, retail, 6, 2), Boolean::TRUE);
        assert_eq!(D_ValidEpisodeMap(heretic, retail, 6, 3), Boolean::TRUE);
        assert_eq!(D_ValidEpisodeMap(heretic, retail, 6, 4), Boolean::FALSE);
        assert_eq!(D_ValidEpisodeMap(heretic, retail, 6, 0), Boolean::FALSE);
    }

    /// Heretic registered secret episode 4 allows only map 1.
    #[test]
    fn valid_episode_map_heretic_registered_ep4_map1_only() {
        assert_eq!(D_ValidEpisodeMap(heretic, registered, 4, 1), Boolean::TRUE);
        assert_eq!(D_ValidEpisodeMap(heretic, registered, 4, 2), Boolean::FALSE);
        assert_eq!(D_ValidEpisodeMap(heretic, registered, 4, 0), Boolean::FALSE);
    }

    /// Doom 2 only has episode 1; requesting episode 2 should fail.
    #[test]
    fn valid_episode_map_doom2_ep2_invalid() {
        assert_eq!(D_ValidEpisodeMap(doom2, commercial, 2, 1), Boolean::FALSE);
        assert_eq!(D_ValidEpisodeMap(doom2, commercial, 1, 1), Boolean::TRUE);
        assert_eq!(D_ValidEpisodeMap(doom2, commercial, 1, 32), Boolean::TRUE);
        assert_eq!(D_ValidEpisodeMap(doom2, commercial, 1, 33), Boolean::FALSE);
    }

    /// Map 0 is always invalid.
    #[test]
    fn valid_episode_map_map_zero_invalid() {
        assert_eq!(D_ValidEpisodeMap(doom, retail, 1, 0), Boolean::FALSE);
        assert_eq!(D_ValidEpisodeMap(doom2, commercial, 1, 0), Boolean::FALSE);
    }

    /// D_GetNumEpisodes for doom shareware has 1 episode.
    #[test]
    fn get_num_episodes_doom_shareware() {
        assert_eq!(D_GetNumEpisodes(doom, shareware), 1);
    }

    /// D_GetNumEpisodes for doom2 (commercial) returns 1.
    #[test]
    fn get_num_episodes_doom2() {
        assert_eq!(D_GetNumEpisodes(doom2, commercial), 1);
    }

    /// D_GetNumEpisodes for doom registered returns 3.
    #[test]
    fn get_num_episodes_doom_registered() {
        assert_eq!(D_GetNumEpisodes(doom, registered), 3);
    }

    /// D_IsEpisodeMap is true for pack_chex (chex.wad is episode-based).
    #[test]
    fn is_episode_map_pack_chex_true() {
        assert_eq!(D_IsEpisodeMap(pack_chex), Boolean::TRUE);
    }

    /// pack_tnt / pack_plut are commercial (MAP01–MAP32), not episode-based.
    #[test]
    fn is_episode_map_pack_tnt_false() {
        assert_eq!(D_IsEpisodeMap(pack_tnt), Boolean::FALSE);
        assert_eq!(D_IsEpisodeMap(pack_plut), Boolean::FALSE);
    }

    /// D_ValidGameMode: all expected valid combinations succeed.
    #[test]
    fn valid_game_mode_all_missions() {
        assert_eq!(D_ValidGameMode(doom, retail), Boolean::TRUE);
        assert_eq!(D_ValidGameMode(doom, registered), Boolean::TRUE);
        assert_eq!(D_ValidGameMode(doom2, commercial), Boolean::TRUE);
        assert_eq!(D_ValidGameMode(heretic, shareware), Boolean::TRUE);
        assert_eq!(D_ValidGameMode(hexen, commercial), Boolean::TRUE);
        assert_eq!(D_ValidGameMode(strife, commercial), Boolean::TRUE);
    }

    /// D_ValidGameVersion: doom2 maps to doom for version checks.
    #[test]
    fn valid_game_version_all_doom_variants_map_to_doom() {
        for mission in [doom2, pack_plut, pack_tnt, pack_hacx, pack_chex] {
            assert_eq!(
                D_ValidGameVersion(mission, exe_doom_1_9),
                Boolean::TRUE,
                "mission {mission} should accept exe_doom_1_9"
            );
        }
    }

    /// D_GameMissionString returns the right string for every known mission.
    #[test]
    fn game_mission_string_all_missions() {
        use std::ffi::CStr;
        let cases: &[(c_int, &str)] = &[
            (doom, "doom"),
            (doom2, "doom2"),
            (pack_tnt, "tnt"),
            (pack_plut, "plutonia"),
            (pack_hacx, "hacx"),
            (pack_chex, "chex"),
            (heretic, "heretic"),
            (hexen, "hexen"),
            (strife, "strife"),
            (none, "none"),
        ];
        for (mission, expected) in cases {
            let ptr = D_GameMissionString(*mission);
            let s = unsafe { CStr::from_ptr(ptr) };
            assert_eq!(s.to_str().unwrap(), *expected, "mission={mission}");
        }
    }
}
