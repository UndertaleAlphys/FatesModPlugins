use crate::util::bitmask::BitMask;
use engage::gamedata::{skill::SkillData, unit::Gender, Gamedata, JobData};

pub mod change;
pub mod flag;
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
