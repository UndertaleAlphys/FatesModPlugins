pub mod bad_states;
mod canto;
pub mod flag;
pub mod map;
pub mod overlap;
mod weapon_expert;
mod winged_sheild;
use engage::gamedata::skill::SkillArray;
use engage::gamedata::{skill::SkillData, unit::Unit};
use unity::prelude::{Il2CppArray, OptionalMethod};
pub trait SkillTrait {
    fn is_condition_true(&self, current: &Unit, reverse: Option<&Unit>) -> bool;
    fn get_move_self(&self) -> i32;
    fn get_move_target(&self) -> i32;
    fn get_range_i(&self) -> i32;
    fn get_range_add(&self) -> i32;
}

impl SkillTrait for SkillData {
    fn is_condition_true(&self, current: &Unit, reverse: Option<&Unit>) -> bool {
        unsafe { skill_is_condition(self, current, reverse, None) }
    }
    fn get_move_self(&self) -> i32 {
        unsafe { skill_get_move_self(self, None) }
    }
    fn get_move_target(&self) -> i32 {
        unsafe { skill_get_move_target(self, None) }
    }
    fn get_range_i(&self) -> i32 {
        unsafe { skill_get_range_i(self, None) }
    }
    fn get_range_add(&self) -> i32 {
        unsafe { skill_get_range_add(self, None) }
    }
}

pub trait SkillArrayTrait {
    fn get_weapon_level(&self, kind: i32) -> i32;
}

impl SkillArrayTrait for SkillArray {
    fn get_weapon_level(&self, kind: i32) -> i32 {
        let weapon_levels = unsafe { skill_array_get_weapon_levels(self, None) };
        weapon_levels.levels[kind as usize] as i32
    }
}

#[skyline::from_offset(0x0248E370)]
fn skill_is_condition(
    this: &SkillData,
    current: &Unit,
    reverse: Option<&Unit>,
    method: OptionalMethod,
) -> bool;

#[skyline::from_offset(0x0248A1F0)]
fn skill_get_move_self(this: &SkillData, method: OptionalMethod) -> i32;

#[skyline::from_offset(0x0248A210)]
fn skill_get_move_target(this: &SkillData, method: OptionalMethod) -> i32;

#[skyline::from_offset(0x0248A170)]
fn skill_get_range_i(this: &SkillData, method: OptionalMethod) -> i32;

#[skyline::from_offset(0x0248A1B0)]
fn skill_get_range_add(this: &SkillData, method: OptionalMethod) -> i32;

#[skyline::from_offset(0x024897D0)]
fn skill_array_get_weapon_levels(this: &SkillArray, method: OptionalMethod) -> &WeaponLevels;

#[repr(C)]
#[unity::class("App", "WeaponLevels")]
struct WeaponLevels {
    pub levels: &'static Il2CppArray<i8>,
}

pub fn install() {
    canto::install();
    overlap::install();
    map::install();
    weapon_expert::install();
    winged_sheild::install();
}
