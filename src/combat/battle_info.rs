use engage::{
    battle::{BattleInfo, BattleInfoSide},
    gamedata::item::UnitItem,
};
use unity::prelude::*;
pub trait BattleInfoTrait {
    fn get_side(&self) -> &BattleInfoSide;
}

impl BattleInfoTrait for BattleInfo {
    fn get_side(&self) -> &BattleInfoSide {
        unsafe { battle_info_get_side(self, 0, None) }
    }
}

pub trait BattleInfoSideTrait {
    fn get_unit_item(&self) -> &UnitItem;
}
impl BattleInfoSideTrait for BattleInfoSide {
    fn get_unit_item(&self) -> &UnitItem {
        unsafe { battle_info_side_get_unit_item(self, None) }
    }
}

#[skyline::from_offset(0x01E7F210)]
pub fn battle_info_get_side(this: &BattleInfo, t: i32, method: OptionalMethod) -> &BattleInfoSide;

#[skyline::from_offset(0x01E74CD0)]
pub fn battle_info_side_get_unit_item(this: &BattleInfoSide, method: OptionalMethod) -> &UnitItem;
