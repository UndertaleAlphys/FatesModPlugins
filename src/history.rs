use engage::gamedata::unit::Unit;
use unity::prelude::OptionalMethod;

pub struct History {}

impl History {
    pub fn engage_turn(unit: &Unit) {
        unsafe { map_history_engage_turn(unit, None) };
    }
}

#[skyline::from_offset(0x01DDEDA0)]
fn map_history_engage_turn(unit: &Unit, method: OptionalMethod);
