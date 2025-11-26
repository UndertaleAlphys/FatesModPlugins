use crate::calculator::command;
use crate::calculator::util::{CalculatorManagerTrait, ListFloats};
use crate::combat::battle_info::BattleInfoSideTrait;
use crate::unit::UnitTrait;
use crate::util::class::UnityClassTrait;
use engage::battle::BattleInfoSide;
use engage::calculator::{CalculatorManager, GameCalculatorCommand};
use engage::gamedata::unit::Unit;
use unity::prelude::{Il2CppString, OptionalMethod};

pub fn add(manager: &mut CalculatorManager) {
    if let Some(command) = manager.clone_from_name(command::HP) {
        command.assign_virtual_method("get_Name", is_in_play_area_command_name as _);
        command.assign_virtual_method("GetImpl", get_is_in_play_area_unit as _);
        command.assign_vtable(31, get_is_in_play_area_battle_info as _);
        manager.add_with_reverse(command);
    }
}

extern "C" fn is_in_play_area_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::IS_IN_PLAY_AREA.into()
}

extern "C" fn get_is_in_play_area_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    let result = if let Some(unit) = unit {
        unit.is_on_map() && unit.is_in_play_area()
    } else {
        false
    };
    result as i32 as f32
}

extern "C" fn get_is_in_play_area_battle_info(
    this: &GameCalculatorCommand,
    battle_info: &BattleInfoSide,
    method: OptionalMethod,
) -> f32 {
    get_is_in_play_area_unit(this, battle_info.get_unit(), method)
}
