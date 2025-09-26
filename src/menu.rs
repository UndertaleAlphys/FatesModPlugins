use engage::gamedata::skill::SkillData;
use unity::prelude::*;
pub const ENABLED: i32 = 1;
pub const DISABLED: i32 = 2;
pub const HIDEN: i32 = 4;
#[unity::class("App", "MapUnitCommandMenu_OverlapSkillMenuItem")]
pub struct OverlapSkillMenuItem {
    _junk1: [u8; 0x58],
    pub skill: Option<&'static SkillData>,
}
