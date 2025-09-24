pub mod flag;
pub mod kind;
pub mod use_type;
use engage::gamedata::item::{ItemData, UnitItem, UnitItemList};
use unity::prelude::*;

use crate::util::bitmask::BitMask;
pub trait ItemTrait {
    fn is_engage_weapon(&self) -> bool;
    fn is_magic_weapon(&self) -> bool;
    fn is_silence_target(&self) -> bool;
}

impl ItemTrait for ItemData {
    fn is_engage_weapon(&self) -> bool {
        return self.flag.value.contains(flag::ENGAGE_WEAPON);
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
}

pub trait ItemListTrait {
    fn get_equipped_item(&self) -> Option<&UnitItem>;
    fn get_equipped_index(&self) -> Option<i32>;
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
}

#[skyline::from_offset(0x01FB4C90)]
pub fn unit_item_list_get_equipped_item(
    unit_item_list: &UnitItemList,
    method: OptionalMethod,
) -> Option<&UnitItem>;

#[skyline::from_offset(0x01FB47E0)]
pub fn unit_item_list_get_equipped_index(
    unit_item_list: &UnitItemList,
    method: OptionalMethod,
) -> i32;
