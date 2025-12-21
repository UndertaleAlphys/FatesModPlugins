use crate::calculator::command;
use crate::calculator::util::CalculatorManagerTrait;
use crate::combat::battle_info::BattleInfoSideTrait;
use crate::unit::UnitTrait;
use crate::util::class::UnityClassTrait;
use engage::battle::{BattleInfo, BattleInfoSide};
use engage::calculator::{CalculatorManager, GameCalculatorCommand};
use engage::gamedata::unit::Unit;
use unity::prelude::{Il2CppString, OptionalMethod};

pub fn add(manager: &mut CalculatorManager) {
    if let Some(none) = manager.clone_from_name(command::HP) {
        none.assign_virtual_method("get_Name", none_command_name as _);
        none.assign_virtual_method("GetImpl", get_none_unit as _);
        none.assign_virtual_method("SetImpl", set_none_unit as _);
        none.assign_vtable(31, get_none_battle_info as _);
        none.assign_vtable(33, set_none_battle_info as _);
        manager.add_command(none);
    }
}
extern "C" fn none_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::NONE.into()
}

extern "C" fn get_none_unit(
    _this: &GameCalculatorCommand,
    _unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    0f32
}

extern "C" fn get_none_battle_info(
    _this: &GameCalculatorCommand,
    _battle_info: &BattleInfo,
    _method: OptionalMethod,
) -> f32 {
    0f32
}

extern "C" fn set_none_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    println!("Unit:{} Debug:{}", unit.unwrap().get_name_string(), value);
}

extern "C" fn set_none_battle_info(
    this: &GameCalculatorCommand,
    battle_info: &BattleInfoSide,
    value: f32,
    method: OptionalMethod,
) {
    set_none_unit(this, battle_info.get_unit(), value, method);
}
