use engage::gamedata::unit::Unit;
use unity::prelude::OptionalMethod;

pub struct History {}

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
}

#[skyline::from_offset(0x01DDEDA0)]
fn map_history_engage_turn(unit: &Unit, method: OptionalMethod);

#[skyline::from_offset(0x01DD9420)]
fn map_history_private_skill(unit: &Unit, method: OptionalMethod);

#[unity::from_offset("App", "MapHistory", "EngageCount")]
fn map_history_engage_count(unit: &Unit, method: OptionalMethod);
