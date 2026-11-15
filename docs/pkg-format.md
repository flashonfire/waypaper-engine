# Wallpaper Engine scene package (`.pkg`) format

Scene wallpapers ship as a single archive next to `project.json`. Its name comes from the
project's `file` field: `scene.json` → `scene.pkg`, `gifscene.json` → `gifscene.pkg`.

## Layout

All integers are little-endian `u32`. The layout is the same in every version listed below.

| Field | Size | Rule |
|---|---|---|
| `magic_len` | 4 | always `8` |
| `magic` | 8 | `PKGV` + 4 ASCII digits, see [versions](#versions) |
| `entry_count` | 4 | `>= 1` |
| entry table | | `entry_count` × the 4 fields below |
| &nbsp;&nbsp;`name_len` | 4 | `> 0`, counted in **bytes** |
| &nbsp;&nbsp;`name` | `name_len` | UTF-8, often non-ASCII (CJK, Cyrillic, `‘`...) |
| &nbsp;&nbsp;`offset` | 4 | relative to the end of the entry table |
| &nbsp;&nbsp;`size` | 4 | |
| data | | entries back to back, in table order |


## Versions

The number is not a container format version. It's a content-compatibility stamp:

- The editor (`wallpaperui.exe`) contains a single version string, `PKGV0024`, next to
  `packProject`. It stamps its own current version on every package it writes.
- The renderer (`wallpaper64.exe`) contains no `PKGV` bytes at all. It does contain the texture and
  model magics (`TEXV0005`, `TEXB0004`, `MDLV0023`...).
- The mobile export calls `checkWallpaperPKGVersions` with the phone app's supported level. It skips
  wallpapers that "require the latest mobile app version".

### How each version is pinned

- **Samples:** the workshop items on disk, dated by Steam's `timeupdated`. A workshop update re-packs
  and re-stamps the file. Several items were created in one era and updated in a later one, and
  each carries the version of its update date.
- **Window:** from the last sample of the previous version to the first sample of this one. A
  release is *pinned* when it's the only Steam announcement inside its window.
- **Content:** a feature or format that first appears in this version's samples. ✅ marks a feature
  the release's own notes announce.

| Version | Introduced by (announcement date) | Pinned | Content first seen in samples | Samples (packages / entries) | Sample dates |
|---|---|---|---|---|---|
| PKGV0001 | baseline, scenes as of late 2016 | — | — | 21 / 581 | 2016-12-18 – 2018-12-04 |
| PKGV0002 | 1.1.31, user properties for scenes (2018-12-09) | yes | — | 9 / 270 | 2018-12-23 – 2019-08-01 |
| PKGV0003 | 1.1.232, text layers (2019-09-22) | yes | — | 3 / 64 | 2019-09-25 – 2019-10-04 |
| PKGV0004 | 1.1.271, fixes only: text layers, script properties, particles, GIF import (2019-10-25) | yes | — | 1 / 19 | 2019-11-16 |
| PKGV0005 | 1.1.301 (2019-12-13) | yes | user-property bindings (`*.user`) | 4 / 148 | 2019-12-17 – 2020-02-11 |
| PKGV0006 | 1.1.341, VHS effect, per-layer LED limit (2020-02-14) | yes | — | 5 / 196 | 2020-03-11 – 2020-04-22 |
| PKGV0007 | 1.2.6 Creator's Update (2020-05-15) **or** 1.2.41 (2020-05-24) | no, 2 candidates | — | 5 / 284 | 2020-06-16 – 2020-07-08 |
| PKGV0008 | 1.3.2, scene flipping, foliage sway (2020-07-11) | yes | — | 1 / 27 | 2020-07-24 |
| PKGV0009 | 1.3.75, scene zoom option (2020-08-08) | yes | ✅ `scene.general.zoom` | 18 / 511 | 2020-08-17 – 2020-12-14 |
| PKGV0010 | 1.4.107, puppet warp, timeline animations, HDR bloom (2020-12-15) | yes | ✅ `bloomhdr*`, `scene.version` | 3 / 115 | 2021-01-07 – 2021-01-09 |
| PKGV0011 | 1.4.172, parallax for 3D models (2021-01-11) | yes | first `.mdl` (`MDLV0013`), `models.puppet` | 3 / 120 | 2021-01-23 – 2021-02-03 |
| PKGV0012 | 1.5.2, iris effect (2021-02-08) | yes | ✅ `effects/iris` | 6 / 182 | 2021-04-02 – 2021-06-08 |
| PKGV0013 | 1.6, real-time lighting and reflections (2021-06-12) | yes | ✅ `LIGHTING` / `REFLECTION` combos | 4 / 136 | 2021-06-20 – 2021-09-09 |
| PKGV0014 | 1.7, video textures in scenes (2021-09-23) | yes | — | 4 / 92 | 2021-10-19 – 2021-11-29 |
| PKGV0015 | 2.0.44, mobile export changes (2021-12-03) | yes¹ | — | 3 / 90 | 2021-12-21 – 2022-01-23 |
| PKGV0016 | 2.0.97, puppet warp limb sorting (2022-01-27) | yes | light objects | 6 / 149 | 2022-04-24 – 2022-06-04 |
| PKGV0017 | 2.1, inverse kinematics, user texture import (2022-06-04) | yes² | `MDLV0014` | 4 / 173 | 2022-06-11 – 2022-09-02 |
| PKGV0018 | 2.2, album covers, fluid simulation (2022-10-07) | yes | `MDLV0016` | 5 / 120 | 2022-11-09 – 2023-08-03 |
| PKGV0019 | 2.3, new 3D model support (2023-09-28) | yes | embedded fonts | 3 / 240 | 2023-11-23 – 2024-01-08 |
| PKGV0020 | 2.4, lighting and shader improvements (2024-02-15) | yes | `usertextures` | 9 / 261 | 2024-03-22 – 2024-05-20 |
| PKGV0021 | 2.5, light shafts, puppet warp model storage overhaul (2024-05-30) | yes | ✅ `effects/lightshafts` | 5 / 340 | 2024-08-16 – 2024-12-02 |
| PKGV0022 | 2.6, clipping masks, rope physics (2025-02-10) | yes | `TEXB0004` textures, `MDLV0019` | 2 / 265 | 2025-03-09 – 2025-06-14 |
| PKGV0023 | 2.7, particle editor updates (2025-09-04) | yes | — | 1 / 75 | 2025-12-19 |
| PKGV0024 | 2.8, dynamic model generation, text rendering (2026-05-28) | yes | — | 1 / 192 | 2026-05-30 |

¹ 2.0 (announced 2021-11-26 15:10 UTC) most likely still wrote PKGV0014

² The last PKGV0016 sample (2816747389) was packed at 13:26 UTC on 2022-06-04, 25 minutes after
the 2.1 announcement (13:01 UTC). Most likely that author's client hadn't updated yet. No other
release was announced before the first PKGV0017 sample (2022-06-11).

"—" means the samples don't use anything new from that release. The bump is pinned by date alone.

