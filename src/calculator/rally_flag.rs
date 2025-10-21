use crate::calculator::command;
use crate::calculator::util::CalculatorManagerTrait;
use crate::history::History;
use crate::unit::UnitTrait;
use crate::util::class::UnityClassTrait;
use engage::calculator::{CalculatorManager, GameCalculatorCommand};
use engage::gamedata::skill::SkillData;
use engage::gamedata::unit::Unit;
use engage::gamedata::Gamedata;
use unity::prelude::{Il2CppString, OptionalMethod};

pub fn add(manager: &mut CalculatorManager) {
    if let Some(rally_flag_c) = manager.clone_from_name(command::HP) {
        rally_flag_c.assign_virtual_method("get_Name", get_rally_flag_command_name as _);
        rally_flag_c.assign_virtual_method("GetImpl", get_rally_flag_unit as _);
        rally_flag_c.assign_virtual_method("SetImpl", set_rally_flag_unit as _);
        manager.add_command(rally_flag_c);
    }
}

const RALLY_FLAG_SID: &str = "SID_Rally_Flag";
extern "C" fn get_rally_flag_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::RALLY_FLAG.into()
}

extern "C" fn get_rally_flag_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    let result = unit.map_or(false, |u| u.has_sid(RALLY_FLAG_SID.into()));
    result as i32 as f32
}

extern "C" fn set_rally_flag_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    if let Some(unit) = unit {
        let value = value != 0f32;
        History::private_skill(unit);
        if value {
            SkillData::get(RALLY_FLAG_SID).map(|s| unit.add_skill(s));
        } else {
            unit.remove_private_sid(RALLY_FLAG_SID);
        }
    }
}
