use crate::skill::SkillArrayTrait;
use engage::gamedata::skill::SkillData;
use engage::gamedata::unit::Unit;
use engage::gamedata::Gamedata;
use skyline::patching::Patch;
use unity::prelude::{Il2CppString, OptionalMethod};

pub trait UnitSkillTrait {
    fn add_skill(&self, skill: &SkillData);
    fn add_sid(&self, sid: impl AsRef<str>);
    fn remove_private_sid(&self, sid: impl AsRef<str>);
    fn has_whole_sid(&self, sid: impl AsRef<str>) -> bool;
    fn remove_equip_sid(&self, sid: impl AsRef<str>);
    fn remove_equip_skill(&self, skill: &SkillData);
    fn remove_equip_pool_sid(&self, sid: impl AsRef<str>);
    fn remove_equip_pool_skill(&self, skill: &SkillData);
}

impl UnitSkillTrait for Unit {
    fn add_skill(&self, skill: &SkillData) {
        unsafe { unit_add_skill(self, skill, None) }
    }
    fn add_sid(&self, sid: impl AsRef<str>) {
        let skill = SkillData::get(sid.as_ref());
        if let Some(skill) = skill {
            self.add_skill(skill);
        }
    }
    fn remove_private_sid(&self, sid: impl AsRef<str>) {
        unsafe { unit_remove_private_sid(self, sid.into(), None) }
    }

    fn has_whole_sid(&self, sid: impl AsRef<str>) -> bool {
        self.has_sid(sid.as_ref().into())
            || self.private_skill.contains_sid(sid.as_ref())
            || self.receive_skill.contains_sid(sid.as_ref())
            || self.supported_skill.contains_sid(sid.as_ref())
    }

    fn remove_equip_sid(&self, sid: impl AsRef<str>) {
        unsafe { unit_remove_equip_sid(self, sid.into(), None) }
    }
    fn remove_equip_skill(&self, skill: &SkillData) {
        unsafe { unit_remove_equip_skill(self, skill, None) }
    }

    fn remove_equip_pool_sid(&self, sid: impl AsRef<str>) {
        unsafe { unit_remove_equip_pool_sid(self, sid.into(), None) }
    }

    fn remove_equip_pool_skill(&self, skill: &SkillData) {
        unsafe { unit_remove_equip_pool_skill(self, skill, None) }
    }
}

#[skyline::from_offset(0x01A5D430)]
fn unit_add_skill(unit: &Unit, skill_data: &SkillData, method: OptionalMethod);

#[skyline::from_offset(0x01A38090)]
fn unit_remove_private_sid(unit: &Unit, sid: &Il2CppString, method: OptionalMethod);

#[skyline::from_offset(0x01a36e80)]
fn unit_remove_equip_sid(unit: &Unit, sid: &Il2CppString, method: OptionalMethod);

#[skyline::from_offset(0x01a36f10)]
fn unit_remove_equip_skill(unit: &Unit, skill: &SkillData, method: OptionalMethod);

#[skyline::from_offset(0x01a38ab0)]
fn unit_remove_equip_pool_sid(unit: &Unit, sid: &Il2CppString, method: OptionalMethod);

#[skyline::from_offset(0x01a38b40)]
fn unit_remove_equip_pool_skill(unit: &Unit, skill: &SkillData, method: OptionalMethod);

pub fn install() {
    // NOP. Allow skills with cost=0 to be inherited.
    Patch::in_text(0x01a35fa4)
        .bytes([0x1F, 0x20, 0x03, 0xD5])
        .unwrap();
    Patch::in_text(0x01a36588)
        .bytes([0x1F, 0x20, 0x03, 0xD5])
        .unwrap();
    Patch::in_text(0x01a36f34)
        .bytes([0x1F, 0x20, 0x03, 0xD5])
        .unwrap();
    Patch::in_text(0x01a38b68)
        .bytes([0x1F, 0x20, 0x03, 0xD5])
        .unwrap();
}
