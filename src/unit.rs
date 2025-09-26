use crate::{class::ClassTrait, skill::flag, terrain::TerrainTrait, util::bitmask::BitMask};
use engage::gamedata::{skill::SkillData, terrain::TerrainData, unit::Unit};
use unity::prelude::*;

pub mod capability;
pub mod status;
const MALE_GENDER: i32 = 1;
const FEMALE_GENDER: i32 = 2;
pub trait UnitTrait {
    fn add_skill(&self, skill: &SkillData);
    fn get_engage_meter(&self) -> i32;
    fn get_engage_meter_limit(&self) -> i32;
    fn is_engage_meter_full(&self) -> bool;
    fn get_hp(&self) -> i32;
    fn get_maxhp(&self) -> i32;
    fn is_fly(&self) -> bool;
    fn has_winged_shield(&self) -> bool;
    fn is_terrain_valid(&self, terrain: &TerrainData) -> bool;
    fn get_center_cell(&self) -> Option<(i32, i32)>;
    fn can_be_moved(&self) -> bool;
    fn can_revive(&self) -> bool;
}

impl UnitTrait for Unit {
    fn add_skill(&self, skill: &SkillData) {
        unsafe { unit_add_skill(self, skill, None) }
    }
    fn get_engage_meter(&self) -> i32 {
        unsafe { unit_get_engage_count(self, None) }
    }
    fn get_engage_meter_limit(&self) -> i32 {
        unsafe { unit_get_engage_count_limit(self, None) }
    }
    fn is_engage_meter_full(&self) -> bool {
        self.get_engage_meter() >= self.get_engage_meter_limit()
    }
    fn get_hp(&self) -> i32 {
        unsafe { unit_get_hp(self, None) }
    }
    fn get_maxhp(&self) -> i32 {
        unsafe { unit_get_max_hp(self, None) }
    }
    fn is_fly(&self) -> bool {
        if self.job.is_fly() {
            true
        } else {
            if let Some(mask) = self.mask_skill {
                mask.flags.contains(flag::MOVE_FLY)
            } else {
                false
            }
        }
    }
    fn has_winged_shield(&self) -> bool {
        self.has_sid(Il2CppString::new("SID_翼盾"))
    }
    fn is_terrain_valid(&self, terrain: &TerrainData) -> bool {
        if terrain.is_fly_enabled() {
            true
        } else if self.is_fly() {
            self.has_winged_shield()
        } else {
            true
        }
    }
    fn get_center_cell(&self) -> Option<(i32, i32)> {
        let map_size = self.get_person().get_bmap_size();
        if map_size % 2 == 0 {
            None
        } else {
            let addition = (map_size - 1) / 2;
            let x = self.x + addition;
            let z = self.z + addition;
            Some((x as i32, z as i32))
        }
    }
    fn can_be_moved(&self) -> bool {
        unsafe { unit_can_external_move(self, None) }
    }
    fn can_revive(&self) -> bool {
        unsafe { unit_can_revive(self, None) }
    }
}

#[skyline::from_offset(0x01A5D430)]
fn unit_add_skill(unit: &Unit, skill_data: &SkillData, method: OptionalMethod);

#[unity::from_offset("App", "Unit", "get_EngageCount")]
fn unit_get_engage_count(this: &Unit, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Unit", "GetEngageCountLimit")]
fn unit_get_engage_count_limit(this: &Unit, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Unit", "GetMaxHp")]
fn unit_get_max_hp(this: &Unit, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Unit", "get_Hp")]
fn unit_get_hp(this: &Unit, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x01A29F10)]
fn unit_can_external_move(unit: &Unit, method: OptionalMethod) -> bool;

#[skyline::from_offset(0x01A4F860)]
fn unit_can_revive(unit: &Unit, method: OptionalMethod) -> bool;

pub fn install() {
    capability::install();
}
