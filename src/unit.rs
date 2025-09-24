use engage::gamedata::{skill::SkillData, unit::Unit};
use unity::prelude::*;

pub mod capability;
pub mod status;
const MALE_GENDER: i32 = 1;
const FEMALE_GENDER: i32 = 2;
pub trait UnitTrait {
    fn add_skill(&self, skill: &SkillData);
}

impl UnitTrait for Unit {
    fn add_skill(&self, skill: &SkillData) {
        unsafe { unit_add_skill(self, skill, None) }
    }
}

#[skyline::from_offset(0x01A5D430)]
pub fn unit_add_skill(unit: &Unit, skill_data: &SkillData, method: OptionalMethod);

pub fn install() {
    capability::install();
}
