use engage::gamedata::unit::Unit;
use unity::prelude::OptionalMethod;

pub struct History;

impl History {
    pub fn engage_turn(unit: &Unit) {
        unsafe { map_history_engage_turn(unit, None) };
    }
    pub fn private_skill(unit: &Unit) {
        unsafe { map_history_private_skill(unit, None) };
    }
    pub fn engage_meter(unit: &Unit) {
        unsafe { map_history_engage_count(unit, None) };
    }
    pub fn item_list(unit: &Unit) {
        unsafe { map_history_unit_item_list(unit, None) };
    }
    pub fn revive(unit: &Unit) {
        unsafe { map_history_revive(unit, None) };
    }
    pub fn clear_extra_hp_stock_count(unit: &Unit) {
        unsafe { map_history_clear_extra_hp_stock(unit, None) };
    }
}

#[skyline::from_offset(0x01DDEDA0)]
fn map_history_engage_turn(unit: &Unit, method: OptionalMethod);

#[skyline::from_offset(0x01DD9420)]
fn map_history_private_skill(unit: &Unit, method: OptionalMethod);

// #[unity::from_offset("App", "MapHistory", "EngageCount")]
#[skyline::from_offset(0x01dd8ca0)]
fn map_history_engage_count(unit: &Unit, method: OptionalMethod);

// #[unity::from_offset("App", "MapHistory", "UnitItemList")]
#[skyline::from_offset(0x01DDB4B0)]
fn map_history_unit_item_list(unit: &Unit, method: OptionalMethod);

#[skyline::from_offset(0x02718290)]
fn map_history_revive(unit: &Unit, method: OptionalMethod);

#[skyline::from_offset(0x01ddecd0)]
fn map_history_clear_extra_hp_stock(unit: &Unit, method: OptionalMethod);
