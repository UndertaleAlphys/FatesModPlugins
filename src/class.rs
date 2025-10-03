use crate::util::bitmask::BitMask;
use engage::gamedata::{skill::SkillData, unit::Gender, Gamedata, JobData, WeaponMask};
use unity::prelude::OptionalMethod;

pub mod change;
pub mod flag;
pub mod move_type;
pub mod skill;

#[derive(PartialEq)]
pub enum ClassRank {
    Base,
    Advanced,
    Special,
}

pub trait ClassTrait {
    fn get_class_rank(&self) -> ClassRank;
    fn has_class_skill(&self) -> bool;
    fn get_class_learn_skill_level(&self) -> i32;
    fn get_gender_lock(&self) -> Option<Gender>;
    fn is_fly(&self) -> bool;
    fn get_max_weapon_level_with_aptitude(&self, kind: i32, aptitude: &WeaponMask) -> i32;
}

impl ClassTrait for JobData {
    fn has_class_skill(&self) -> bool {
        if let Some(skill_iid) = self.learn_skill {
            if let Some(_) = SkillData::get(skill_iid) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    fn get_class_rank(&self) -> ClassRank {
        if self.is_high() {
            ClassRank::Advanced
        } else {
            if self.max_level == 20 {
                ClassRank::Base
            } else {
                ClassRank::Special
            }
        }
    }
    fn get_class_learn_skill_level(&self) -> i32 {
        match self.get_class_rank() {
            ClassRank::Base => 1,
            ClassRank::Advanced => 21,
            ClassRank::Special => 1,
        }
    }
    fn get_gender_lock(&self) -> Option<Gender> {
        let flag_value = self.flag.value;
        if flag_value.contains(flag::FEMALE_ONLY) {
            Some(Gender::Female)
        } else if flag_value.contains(flag::MALE_ONLY) {
            Some(Gender::Male)
        } else {
            None
        }
    }
    fn is_fly(&self) -> bool {
        self.move_type == move_type::FLY
    }
    fn get_max_weapon_level_with_aptitude(&self, kind: i32, aptitude: &WeaponMask) -> i32 {
        unsafe { job_data_get_max_weapon_level_with_aptitude(self, kind, aptitude, None) }
    }
}

pub trait GetClassGenderLock {
    fn get_class_gender_lock(self) -> i32;
}

impl GetClassGenderLock for Gender {
    fn get_class_gender_lock(self) -> i32 {
        match self {
            Gender::Female => flag::FEMALE_ONLY,
            Gender::Male => flag::MALE_ONLY,
            _ => flag::NONE,
        }
    }
}

pub fn install() {
    change::install();
    skill::install();
}

#[skyline::from_offset(0x02056C30)]
fn job_data_get_max_weapon_level_with_aptitude(
    class: &JobData,
    kind: i32,
    aptitude: &WeaponMask,
    method: OptionalMethod,
) -> i32;
