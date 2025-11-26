use crate::calculator::command;
use crate::calculator::util::CalculatorManagerTrait;
use crate::history::History;
use crate::unit::skill::UnitSkillTrait;
use crate::unit::UnitTrait;
use crate::util::class::UnityClassTrait;
use engage::battle::BattleInfoSide;
use engage::calculator::{CalculatorManager, GameCalculatorCommand};
use engage::gamedata::skill::SkillData;
use engage::gamedata::unit::Unit;
use engage::gamedata::Gamedata;
use engage::map::image::MapImage;
use engage::util::get_instance;
use unity::prelude::{Il2CppString, OptionalMethod};

pub fn add(manager: &mut CalculatorManager) {
    // Vtable indexes need more tests...
    if let Some(add) = manager.clone_from_name(command::MALE_FEMALE_COUNTS) {
        add.assign_virtual_method("get_Name", add_personal_skill_command_name as _);
        add.assign_vtable(36, add_personal_skill_unit as _);
        add.assign_vtable(37, add_personal_skill_battle_info as _);
        manager.add_with_reverse(add);
    }
    if let Some(rm) = manager.clone_from_name(command::MALE_FEMALE_COUNTS) {
        rm.assign_virtual_method("get_Name", remove_personal_skill_command_name as _);
        rm.assign_vtable(36, remove_personal_skill_unit as _);
        rm.assign_vtable(37, remove_personal_skill_battle_info as _);
        manager.add_with_reverse(rm);
    }
}
extern "C" fn add_personal_skill_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::ADD_PERSONAL_SKILL.into()
}

extern "C" fn remove_personal_skill_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::REMOVE_PERSONAL_SKILL.into()
}

fn do_personal_skill_unit<F>(unit: Option<&Unit>, sid: &Il2CppString, action: F) -> bool
where
    F: Fn(&Unit, &SkillData),
{
    let mut result = false;
    if let Some(unit) = unit {
        if unit.is_on_map() && unit.is_in_play_area() {
            if let Some(unit) =
                get_instance::<MapImage>().get_target_unit(unit.get_x(), unit.get_z())
            {
                if let Some(skill) = SkillData::get(sid) {
                    History::private_skill(unit);
                    action(unit, skill);
                    result = true;
                }
            }
        }
    }
    result
}
extern "C" fn add_personal_skill_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    sid: &Il2CppString,
    _method_info: OptionalMethod,
) -> f32 {
    do_personal_skill_unit(unit, sid, |u, s| u.add_skill(s)) as i32 as f32
}

extern "C" fn add_personal_skill_battle_info(
    this: &GameCalculatorCommand,
    battle_info: &BattleInfoSide,
    sid: &Il2CppString,
    method: OptionalMethod,
) -> f32 {
    add_personal_skill_unit(this, battle_info.unit, sid, method)
}

extern "C" fn remove_personal_skill_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    sid: &Il2CppString,
    _method_info: OptionalMethod,
) -> f32 {
    do_personal_skill_unit(unit, sid, |u, s| u.remove_private_sid(s.sid.to_string())) as i32 as f32
}

extern "C" fn remove_personal_skill_battle_info(
    this: &GameCalculatorCommand,
    battle_info: &BattleInfoSide,
    sid: &Il2CppString,
    method: OptionalMethod,
) -> f32 {
    remove_personal_skill_unit(this, battle_info.unit, sid, method)
}
