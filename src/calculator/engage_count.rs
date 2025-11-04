use crate::calculator::command;
use crate::calculator::util::CalculatorManagerTrait;
use crate::history::History;
use crate::unit::UnitTrait;
use crate::util::class::UnityClassTrait;
use engage::calculator::{CalculatorManager, GameCalculatorCommand};
use engage::gamedata::unit::Unit;
use unity::prelude::OptionalMethod;

pub fn add(manager: &mut CalculatorManager) {
    if let Some(engage_meter_c) = manager.find_checked(command::ENGAGE_METER) {
        engage_meter_c.assign_virtual_method("GetImpl", get_engage_meter_unit as _);
        engage_meter_c.assign_virtual_method("SetImpl", set_engage_meter_unit as _);
    }
}
extern "C" fn get_engage_meter_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    let result = unit.map_or(0, |u| u.get_engage_meter());
    result as f32
}
extern "C" fn set_engage_meter_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    if let Some(unit) = unit {
        History::engage_meter(unit);
        unit.set_engage_meter(value as i32);
    }
}
