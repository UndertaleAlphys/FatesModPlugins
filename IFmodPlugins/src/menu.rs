use crate::util::class::CanGetClass;
use engage::gamedata::skill::SkillData;
use unity::prelude::*;

pub const ENABLED: i32 = 1;
pub const DISABLED: i32 = 2;
pub const HIDEN: i32 = 4;

#[unity::class("App", "BasicMenuItem")]
pub struct BasicMenuItem {
    _junk: [u8; 0x58],
}
#[unity::class("App", "MapUnitCommandMenu_OverlapSkillMenuItem")]
pub struct OverlapSkillMenuItem {
    _junk1: BasicMenuItemFields,
    pub skill: Option<&'static SkillData>,
}

#[unity::class(
    "App",
    "MainMenuSequence_DifficultySelectMenuSequence_Menu_NormalMenuItem"
)]
pub struct MainMenuNormal {
    _junk1: BasicMenuItemFields,
    pub difficulty: i32,
}

#[unity::class(
    "App",
    "MainMenuSequence_DifficultySelectMenuSequence_Menu_HardMenuItem"
)]
pub struct MainMenuHard {
    _junk1: BasicMenuItemFields,
    pub difficulty: i32,
}

impl CanGetClass for MainMenuNormal {
    fn get_class_mut_if_mod(&mut self) -> &mut Il2CppClass {
        self.get_class_mut()
    }
    fn get_class_if_mod(&mut self) -> &Il2CppClass {
        self.get_class()
    }
}

impl CanGetClass for MainMenuHard {
    fn get_class_mut_if_mod(&mut self) -> &mut Il2CppClass {
        self.get_class_mut()
    }
    fn get_class_if_mod(&mut self) -> &Il2CppClass {
        self.get_class()
    }
}
