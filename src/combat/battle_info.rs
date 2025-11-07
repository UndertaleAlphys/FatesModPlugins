use engage::{
    battle::{BattleInfo, BattleInfoSide},
    gamedata::{item::UnitItem, terrain::TerrainData, unit::Unit},
};
use unity::prelude::*;
pub trait BattleInfoTrait {
    fn get_offense_side(&self) -> &BattleInfoSide;
    fn get_defense_side(&self) -> &BattleInfoSide;
}

impl BattleInfoTrait for BattleInfo {
    fn get_offense_side(&self) -> &BattleInfoSide {
        unsafe { battle_info_get_side(self, 0, None) }
    }
    fn get_defense_side(&self) -> &BattleInfoSide {
        unsafe { battle_info_get_side(self, 1, None) }
    }
}

pub trait BattleInfoSideTrait {
    fn get_unit(&self) -> Option<&Unit>;
    fn get_unit_item(&self) -> &UnitItem;
    fn get_terrain(&self) -> &TerrainData;
}
impl BattleInfoSideTrait for BattleInfoSide {
    fn get_unit(&self) -> Option<&Unit> {
        unsafe { battle_info_side_get_unit(self, None) }
    }
    fn get_unit_item(&self) -> &UnitItem {
        unsafe { battle_info_side_get_unit_item(self, None) }
    }
    fn get_terrain(&self) -> &TerrainData {
        unsafe { battle_info_side_get_terrain(self, None) }
    }
}

#[skyline::from_offset(0x01E7F210)]
pub fn battle_info_get_side(this: &BattleInfo, t: i32, method: OptionalMethod) -> &BattleInfoSide;

#[skyline::from_offset(0x01E8B250)]
pub fn battle_info_side_get_unit(this: &BattleInfoSide, method: OptionalMethod) -> Option<&Unit>;

#[skyline::from_offset(0x01E74CD0)]
pub fn battle_info_side_get_unit_item(this: &BattleInfoSide, method: OptionalMethod) -> &UnitItem;

#[skyline::from_offset(0x01E8B320)]
pub fn battle_info_side_get_terrain(this: &BattleInfoSide, method: OptionalMethod) -> &TerrainData;
