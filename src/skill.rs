mod canto;
pub mod flag;
pub mod map;
pub mod overlap;
use engage::gamedata::{skill::SkillData, unit::Unit};
use unity::prelude::OptionalMethod;
pub trait SkillTrait {
    fn is_condition_true(&self, current: &Unit, reverse: Option<&Unit>) -> bool;
    fn get_move_self(&self) -> i32;
    fn get_move_target(&self) -> i32;
    fn get_range_i(&self) -> i32;
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

pub fn install() {
    canto::install();
    overlap::install();
    map::install();
}
