# Details

Date : 2025-09-28 09:40:08

Directory c:\\Users\\Mercedes\\Documents\\VSCode Projects\\IFmodPlugins

Total : 58 files,  2444 codes, 69 comments, 263 blanks, all 2776 lines

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [.idea/IFmodPlugins.iml](/.idea/IFmodPlugins.iml) | XML | 11 | 0 | 0 | 11 |
| [.idea/modules.xml](/.idea/modules.xml) | XML | 8 | 0 | 0 | 8 |
| [.idea/vcs.xml](/.idea/vcs.xml) | XML | 6 | 0 | 0 | 6 |
| [src/calculator.rs](/src/calculator.rs) | Rust | 15 | 0 | 3 | 18 |
| [src/calculator/command.rs](/src/calculator/command.rs) | Rust | 7 | 0 | 1 | 8 |
| [src/calculator/item.rs](/src/calculator/item.rs) | Rust | 51 | 1 | 4 | 56 |
| [src/calculator/terrain.rs](/src/calculator/terrain.rs) | Rust | 142 | 3 | 12 | 157 |
| [src/calculator/util.rs](/src/calculator/util.rs) | Rust | 97 | 0 | 10 | 107 |
| [src/class.rs](/src/class.rs) | Rust | 79 | 0 | 8 | 87 |
| [src/class/change.rs](/src/class/change.rs) | Rust | 6 | 0 | 0 | 6 |
| [src/class/change/gender\_lock.rs](/src/class/change/gender_lock.rs) | Rust | 28 | 0 | 5 | 33 |
| [src/class/change/level.rs](/src/class/change/level.rs) | Rust | 115 | 1 | 15 | 131 |
| [src/class/flag.rs](/src/class/flag.rs) | Rust | 6 | 0 | 1 | 7 |
| [src/class/move\_type.rs](/src/class/move_type.rs) | Rust | 4 | 0 | 1 | 5 |
| [src/class/skill.rs](/src/class/skill.rs) | Rust | 23 | 1 | 6 | 30 |
| [src/combat.rs](/src/combat.rs) | Rust | 12 | 0 | 1 | 13 |
| [src/combat/ai.rs](/src/combat/ai.rs) | Rust | 18 | 0 | 1 | 19 |
| [src/combat/battle\_info.rs](/src/combat/battle_info.rs) | Rust | 37 | 0 | 7 | 44 |
| [src/combat/engage\_count.rs](/src/combat/engage_count.rs) | Rust | 9 | 0 | 2 | 11 |
| [src/combat/no\_dynlv.rs](/src/combat/no_dynlv.rs) | Rust | 17 | 1 | 2 | 20 |
| [src/combat/silence.rs](/src/combat/silence.rs) | Rust | 53 | 3 | 4 | 60 |
| [src/combat/skill\_point.rs](/src/combat/skill_point.rs) | Rust | 9 | 0 | 1 | 10 |
| [src/item.rs](/src/item.rs) | Rust | 92 | 0 | 8 | 100 |
| [src/item/check\_use.rs](/src/item/check_use.rs) | Rust | 50 | 1 | 4 | 55 |
| [src/item/flag.rs](/src/item/flag.rs) | Rust | 2 | 0 | 0 | 2 |
| [src/item/heal\_override.rs](/src/item/heal_override.rs) | Rust | 14 | 2 | 1 | 17 |
| [src/item/kind.rs](/src/item/kind.rs) | Rust | 21 | 0 | 1 | 22 |
| [src/item/use\_type.rs](/src/item/use_type.rs) | Rust | 43 | 0 | 1 | 44 |
| [src/lib.rs](/src/lib.rs) | Rust | 43 | 23 | 6 | 72 |
| [src/map.rs](/src/map.rs) | Rust | 128 | 9 | 23 | 160 |
| [src/menu.rs](/src/menu.rs) | Rust | 10 | 0 | 1 | 11 |
| [src/misc.rs](/src/misc.rs) | Rust | 12 | 0 | 2 | 14 |
| [src/misc/no\_investment.rs](/src/misc/no_investment.rs) | Rust | 9 | 0 | 2 | 11 |
| [src/misc/no\_skirmishes.rs](/src/misc/no_skirmishes.rs) | Rust | 11 | 1 | 2 | 14 |
| [src/misc/no\_well.rs](/src/misc/no_well.rs) | Rust | 9 | 0 | 3 | 12 |
| [src/misc/patch\_msg.rs](/src/misc/patch_msg.rs) | Rust | 31 | 0 | 3 | 34 |
| [src/misc/save\_header.rs](/src/misc/save_header.rs) | Rust | 8 | 0 | 2 | 10 |
| [src/skill.rs](/src/skill.rs) | Rust | 55 | 0 | 8 | 63 |
| [src/skill/bad\_states.rs](/src/skill/bad_states.rs) | Rust | 2 | 0 | 1 | 3 |
| [src/skill/canto.rs](/src/skill/canto.rs) | Rust | 59 | 0 | 5 | 64 |
| [src/skill/flag.rs](/src/skill/flag.rs) | Rust | 3 | 0 | 1 | 4 |
| [src/skill/map.rs](/src/skill/map.rs) | Rust | 287 | 23 | 19 | 329 |
| [src/skill/overlap.rs](/src/skill/overlap.rs) | Rust | 30 | 0 | 1 | 31 |
| [src/skill/weapon\_expert.rs](/src/skill/weapon_expert.rs) | Rust | 20 | 0 | 2 | 22 |
| [src/skill/winged\_sheild.rs](/src/skill/winged_sheild.rs) | Rust | 58 | 0 | 7 | 65 |
| [src/terrain.rs](/src/terrain.rs) | Rust | 24 | 0 | 4 | 28 |
| [src/terrain/limit\_expansion.rs](/src/terrain/limit_expansion.rs) | Rust | 8 | 0 | 1 | 9 |
| [src/terrain/percentage.rs](/src/terrain/percentage.rs) | Rust | 32 | 0 | 3 | 35 |
| [src/unit.rs](/src/unit.rs) | Rust | 169 | 0 | 14 | 183 |
| [src/unit/capability.rs](/src/unit/capability.rs) | Rust | 20 | 0 | 3 | 23 |
| [src/unit/capability/limit.rs](/src/unit/capability/limit.rs) | Rust | 181 | 0 | 24 | 205 |
| [src/unit/capability/multiplier.rs](/src/unit/capability/multiplier.rs) | Rust | 117 | 0 | 3 | 120 |
| [src/unit/status.rs](/src/unit/status.rs) | Rust | 1 | 0 | 1 | 2 |
| [src/unit/terrain.rs](/src/unit/terrain.rs) | Rust | 79 | 0 | 9 | 88 |
| [src/util.rs](/src/util.rs) | Rust | 3 | 0 | 0 | 3 |
| [src/util/bitmask.rs](/src/util/bitmask.rs) | Rust | 21 | 0 | 5 | 26 |
| [src/util/language.rs](/src/util/language.rs) | Rust | 7 | 0 | 2 | 9 |
| [src/util/thread\_safe.rs](/src/util/thread_safe.rs) | Rust | 32 | 0 | 7 | 39 |

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)