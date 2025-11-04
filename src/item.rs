pub mod check_use;
pub mod flag;
mod heal_override;
pub mod kind;
pub mod use_type;
use engage::gamedata::item::{ItemData, UnitItem, UnitItemList};
use unity::prelude::*;

use crate::util::bitmask::BitMask;
pub trait ItemTrait {
    fn is_engage_weapon(&self) -> bool;
    fn is_magic_weapon(&self) -> bool;
    fn is_silence_target(&self) -> bool;
    fn is_tiki_blessing(&self) -> bool;
    fn get_heal_overridden(&self) -> Option<i32>;
}

impl ItemTrait for ItemData {
    fn is_engage_weapon(&self) -> bool {
        self.flag.value.contains(flag::ENGAGE_WEAPON)
    }
    fn is_magic_weapon(&self) -> bool {
        let magic = self.kind == kind::MAGIC;
        let reversed = self.flag.value.contains(flag::REVERSE_ATTRIBUTE);
        magic ^ reversed
    }
    fn is_silence_target(&self) -> bool {
        match self.kind {
            kind::ROD => true,
            _ => self.is_magic_weapon(),
        }
    }
    fn is_tiki_blessing(&self) -> bool {
        let skills = self.get_equip_skills();
        skills.find_sid(Il2CppString::new("SID_祝福")).is_some()
    }
    fn get_heal_overridden(&self) -> Option<i32> {
        let item_skills = self.get_equip_skills();
        let mut heal_override: Option<i32> = None;
        for skill in item_skills.iter() {
            if let Some(skill_data) = skill.get_skill() {
                let sid = skill_data.sid.to_string();
                if let Some(number) = sid.strip_prefix("SID_HealOverride_") {
                    heal_override = number.parse::<i32>().ok();
                    break;
                }
            }
        }
        heal_override
    }
}

pub trait ItemListTrait {
    fn get_equipped_item(&self) -> Option<&UnitItem>;
    fn get_equipped_index(&self) -> Option<i32>;
    fn equip(&self, idx: i32);
    fn take_off(&self, idx: i32) -> bool;
}

impl ItemListTrait for UnitItemList {
    fn get_equipped_item(&self) -> Option<&UnitItem> {
        unsafe { unit_item_list_get_equipped_item(self, None) }
    }
    fn get_equipped_index(&self) -> Option<i32> {
        let idx = unsafe { unit_item_list_get_equipped_index(self, None) };
        if idx == -1 {
            None
        } else {
            Some(idx)
        }
    }
    fn equip(&self, idx: i32) {
        unsafe {
            unit_item_list_equip(self, idx, None);
        }
    }

    fn take_off(&self, idx: i32) -> bool {
        unsafe { unit_item_list_take_off(self, idx, None) }
    }
}

#[skyline::from_offset(0x01FB4C90)]
fn unit_item_list_get_equipped_item(
    unit_item_list: &UnitItemList,
    method: OptionalMethod,
) -> Option<&UnitItem>;
#[skyline::from_offset(0x01FB47E0)]
fn unit_item_list_get_equipped_index(unit_item_list: &UnitItemList, method: OptionalMethod) -> i32;
#[skyline::from_offset(0x01FB4760)]
fn unit_item_list_equip(unit_item_list: &UnitItemList, idx: i32, method: OptionalMethod);
#[skyline::from_offset(0x01FB4900)]
fn unit_item_list_take_off(unit_item_list: &UnitItemList, idx: i32, method: OptionalMethod)
    -> bool;

pub fn install() {
    check_use::install();
    heal_override::install();
}
