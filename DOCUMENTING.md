# Documentation Progress

Tracks `///` / `//!` doc-comment coverage across all ported modules in `room/`.

Legend: `[ ]` undone - `[~]` in-progress - `[x]` done

## How to Claim a File

1. Pick a `[ ]` entry, change it to `[~]`, commit:
   `docs: claim <file> for documentation`
2. Open the Rust file alongside its `vendor/doomgeneric/` C counterpart.
3. Add doc comments to **every** item (see `docs/superpowers/specs/2026-05-17-doc-commenting-design.md`).
4. Flag behavioral discrepancies with inline `// FIXME: <description>`.
5. Change `[~]` to `[x]`, commit:
   `docs(<module>): document <file>`

Multiple agents may work in parallel - each touches a different line, so git
conflicts resolve trivially.

---

## Core / Game Logic (`d_*`, `g_*`, misc)

- [ ] doom/d_event.rs (114 lines, 0 docs)
- [ ] doom/d_items.rs (161 lines, 6 docs)
- [ ] doom/d_iwad.rs (374 lines, 0 docs)
- [ ] doom/d_loop.rs (383 lines, 0 docs)
- [ ] doom/d_main.rs (1808 lines, 53 docs)
- [ ] doom/d_mode.rs (452 lines, 12 docs)
- [ ] doom/d_net.rs (307 lines, 0 docs)
- [ ] doom/d_player.rs (133 lines, 3 docs)
- [ ] doom/doomgeneric.rs (81 lines, 0 docs)
- [ ] doom/doomkeys.rs (108 lines, 1 doc)
- [ ] doom/doomstat.rs (48 lines, 0 docs)
- [ ] doom/dstrings.rs (131 lines, 3 docs)
- [ ] doom/dummy.rs (52 lines, 0 docs)
- [ ] doom/g_game.rs (2583 lines, 35 docs)

## Renderer (`r_*`)

- [ ] doom/r_bsp.rs (927 lines, 0 docs)
- [ ] doom/r_data.rs (834 lines, 0 docs)
- [ ] doom/r_draw.rs (1360 lines, 4 docs)
- [ ] doom/r_main.rs (864 lines, 12 docs)
- [ ] doom/r_plane.rs (584 lines, 1 doc)
- [ ] doom/r_segs.rs (672 lines, 0 docs)
- [ ] doom/r_sky.rs (65 lines, 7 docs)
- [ ] doom/r_things.rs (991 lines, 14 docs)

## Map / Physics (`p_*`)

- [ ] doom/p_ceilng.rs (318 lines, 0 docs)
- [ ] doom/p_doors.rs (590 lines, 2 docs)
- [ ] doom/p_enemy.rs (1639 lines, 0 docs)
- [ ] doom/p_floor.rs (597 lines, 2 docs)
- [ ] doom/p_inter.rs (992 lines, 0 docs)
- [ ] doom/p_lights.rs (456 lines, 2 docs)
- [ ] doom/p_map.rs (1052 lines, 0 docs)
- [ ] doom/p_maputl.rs (751 lines, 0 docs)
- [ ] doom/p_mobj.rs (883 lines, 0 docs)
- [ ] doom/p_plats.rs (396 lines, 0 docs)
- [ ] doom/p_pspr.rs (928 lines, 42 docs)
- [ ] doom/p_saveg.rs (1725 lines, 6 docs)
- [ ] doom/p_setup.rs (1238 lines, 51 docs)
- [ ] doom/p_sight.rs (507 lines, 10 docs)
- [ ] doom/p_spec.rs (1168 lines, 0 docs)
- [ ] doom/p_switch.rs (736 lines, 5 docs)
- [ ] doom/p_telept.rs (291 lines, 2 docs)
- [ ] doom/p_tick.rs (184 lines, 3 docs)
- [ ] doom/p_user.rs (450 lines, 4 docs)

## HUD / UI (`hu_*`, `st_*`, `wi_*`, `am_map`, `f_*`, `m_menu`)

- [ ] doom/am_map.rs (1429 lines, 0 docs)
- [ ] doom/f_finale.rs (1108 lines, 1 doc)
- [ ] doom/f_wipe.rs (258 lines, 2 docs)
- [ ] doom/hu_lib.rs (606 lines, 6 docs)
- [ ] doom/hu_stuff.rs (625 lines, 2 docs)
- [ ] doom/m_menu.rs (1731 lines, 0 docs)
- [ ] doom/st_lib.rs (350 lines, 2 docs)
- [ ] doom/st_stuff.rs (1116 lines, 0 docs)
- [ ] doom/wi_stuff.rs (1725 lines, 16 docs)

## System / I/O (`i_*`, `w_*`, misc)

- [ ] doom/i_cdmus.rs (55 lines, 0 docs)
- [ ] doom/i_endoom.rs (4 lines, 0 docs)
- [ ] doom/i_input.rs (126 lines, 0 docs)
- [ ] doom/i_joystick.rs (77 lines, 0 docs)
- [ ] doom/i_scale.rs (902 lines, 0 docs)
- [ ] doom/i_sound.rs (359 lines, 0 docs)
- [ ] doom/i_system.rs (320 lines, 1 doc)
- [ ] doom/i_timer.rs (50 lines, 0 docs)
- [ ] doom/i_video.rs (375 lines, 0 docs)
- [ ] doom/memio.rs (410 lines, 8 docs)
- [ ] doom/sha1.rs (453 lines, 9 docs)
- [ ] doom/statdump.rs (102 lines, 3 docs)
- [ ] doom/w_checksum.rs (65 lines, 0 docs)
- [ ] doom/w_file.rs (100 lines, 0 docs)
- [ ] doom/w_main.rs (38 lines, 0 docs)
- [ ] doom/w_wad.rs (552 lines, 0 docs)

## Utilities (`m_*`, `tables`, `sounds`, `s_sound`, `info`, `statenum`, `z_zone`)

- [ ] doom/info.rs (13071 lines, 19 docs)
- [ ] doom/m_argv.rs (172 lines, 0 docs)
- [ ] doom/m_bbox.rs (90 lines, 10 docs)
- [ ] doom/m_cheat.rs (200 lines, 29 docs)
- [ ] doom/m_config.rs (686 lines, 0 docs)
- [ ] doom/m_controls.rs (861 lines, 0 docs)
- [ ] doom/m_fixed.rs (154 lines, 17 docs)
- [ ] doom/m_misc.rs (875 lines, 30 docs)
- [ ] doom/m_random.rs (166 lines, 9 docs)
- [ ] doom/s_sound.rs (520 lines, 0 docs)
- [ ] doom/sounds.rs (928 lines, 5 docs)
- [ ] doom/statenum.rs (971 lines, 0 docs)
- [ ] doom/tables.rs (1634 lines, 0 docs)
- [ ] doom/z_zone.rs (470 lines, 1 doc)

## Platform Layer (`audio/`, `platform/`, `types/`, top-level)

- [ ] audio/mod.rs (80 lines, 0 docs)
- [ ] audio/music.rs (491 lines, 0 docs)
- [ ] audio/sfx.rs (266 lines, 0 docs)
- [ ] bin/struct_sizes.rs (70 lines, 0 docs)
- [ ] doom/c_ffi.rs (739 lines, 145 docs)
- [ ] doom.rs (92 lines, 0 docs)
- [ ] gpu.rs (403 lines, 47 docs)
- [ ] headless.rs (33 lines, 0 docs)
- [ ] lib.rs (24 lines, 0 docs)
- [ ] main.rs (260 lines, 25 docs)
- [ ] platform/keys.rs (95 lines, 6 docs)
- [ ] platform/mod.rs (200 lines, 47 docs)
- [ ] types.rs (16 lines, 0 docs)
- [ ] types/doom_bool.rs (214 lines, 22 docs)

## Test Harness (`doom/c_tests/`)

- [ ] doom/c_tests/am_map_c.rs (137 lines, 16 docs)
- [ ] doom/c_tests/d_loop_c.rs (92 lines, 8 docs)
- [ ] doom/c_tests/d_main_c.rs (183 lines, 20 docs)
- [ ] doom/c_tests/f_finale_c.rs (158 lines, 14 docs)
- [ ] doom/c_tests/g_game_c.rs (360 lines, 41 docs)
- [ ] doom/c_tests/harness.rs (29 lines, 2 docs)
- [ ] doom/c_tests/hu_stuff_c.rs (282 lines, 38 docs)
- [ ] doom/c_tests/i_scale_c.rs (425 lines, 50 docs)
- [ ] doom/c_tests/lookup_tables.rs (65 lines, 0 docs)
- [ ] doom/c_tests/map_data_structs.rs (362 lines, 22 docs)
- [ ] doom/c_tests/mod.rs (31 lines, 0 docs)
- [ ] doom/c_tests/p_enemy_c.rs (180 lines, 23 docs)
- [ ] doom/c_tests/p_inter_c.rs (178 lines, 18 docs)
- [ ] doom/c_tests/p_map_c.rs (209 lines, 26 docs)
- [ ] doom/c_tests/p_maputl_c.rs (898 lines, 0 docs)
- [ ] doom/c_tests/p_mobj_c.rs (148 lines, 17 docs)
- [ ] doom/c_tests/p_pspr_c.rs (132 lines, 23 docs)
- [ ] doom/c_tests/p_saveg_c.rs (99 lines, 11 docs)
- [ ] doom/c_tests/p_spec_c.rs (183 lines, 23 docs)
- [ ] doom/c_tests/p_switch_c.rs (102 lines, 9 docs)
- [ ] doom/c_tests/r_data_c.rs (73 lines, 7 docs)
- [ ] doom/c_tests/r_draw_c.rs (238 lines, 1 doc)
- [ ] doom/c_tests/r_segs_c.rs (226 lines, 14 docs)
- [ ] doom/c_tests/r_things_c.rs (246 lines, 34 docs)
- [ ] doom/c_tests/struct_layouts.rs (271 lines, 0 docs)
- [ ] doom/c_tests/st_stuff_c.rs (191 lines, 26 docs)
- [ ] doom/c_tests/wi_stuff_c.rs (138 lines, 17 docs)
- [ ] doom/c_tests/wrapping_arithmetic.rs (205 lines, 0 docs)
