use crate::calculator::command;
use crate::calculator::util::CalculatorManagerTrait;
use crate::history::History;
use crate::unit::UnitTrait;
use crate::util::class::UnityClassTrait;
use engage::calculator::{CalculatorManager, GameCalculatorCommand};
use engage::gamedata::unit::Unit;
use unity::prelude::{Il2CppString, OptionalMethod};

pub fn add(manager: &mut CalculatorManager) {
    if let Some(engage_turn_c) = manager.clone_from_name(command::HP) {
        engage_turn_c.assign_virtual_method("get_Name", get_engage_turn_command_name as _);
        engage_turn_c.assign_virtual_method("GetImpl", get_engage_turn_unit as _);
        engage_turn_c.assign_virtual_method("SetImpl", set_engage_turn_unit as _);
        manager.add_command(engage_turn_c);
        if let Some(reverse_c) = engage_turn_c.clone() {
            manager.add_command(reverse_c.reverse());
        }
    }
    if let Some(engage_turn_limit_c) = manager.clone_from_name(command::HP) {
        engage_turn_limit_c
            .assign_virtual_method("get_Name", get_engage_turn_limit_command_name as _);
        engage_turn_limit_c.assign_virtual_method("GetImpl", get_engage_turn_limit_unit as _);
        manager.add_command(engage_turn_limit_c);
        if let Some(reverse_c) = engage_turn_limit_c.clone() {
            manager.add_command(reverse_c.reverse());
        }
    }
}

extern "C" fn get_engage_turn_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::ENGAGE_TURN.into()
}

extern "C" fn get_engage_turn_limit_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::ENGAGE_TURN_LIMIT.into()
}
extern "C" fn get_engage_turn_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    let result = unit.map_or(0, |u| u.engage_turn);
    result as f32
}

extern "C" fn get_engage_turn_limit_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    let result = unit.map_or(0, |u| u.get_engage_turn_limit());
    result as f32
}

extern "C" fn set_engage_turn_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    if let Some(unit) = unit {
        History::engage_turn(unit);
        unit.set_engage_turn((value as i32).clamp(0, unit.get_engage_turn_limit()));
    }
}
