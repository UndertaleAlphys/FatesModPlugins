use crate::calculator::command;
use crate::calculator::util::CalculatorManagerTrait;
use crate::history::History;
use crate::unit::UnitTrait;
use crate::util::class::UnityClassTrait;
use engage::calculator::{CalculatorManager, GameCalculatorCommand};
use engage::gamedata::unit::Unit;
use unity::prelude::{Il2CppString, OptionalMethod};
use crate::unit::skill::UnitSkillTrait;

pub fn add(manager: &mut CalculatorManager) {
    if let Some(half_debuff_set_command_c) = manager.clone_from_name(command::HP) {
        half_debuff_set_command_c
            .assign_virtual_method("get_Name", half_debuff_set_command_name as _);
        half_debuff_set_command_c.assign_virtual_method("SetImpl", half_debuff_set_set_impl as _);
        manager.add_command(half_debuff_set_command_c);
        if let Some(reverse_c) = half_debuff_set_command_c.clone() {
            manager.add_command(reverse_c.reverse());
        }
    }
}
extern "C" fn half_debuff_set_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::HALF_DEBUFF_SET.into()
}

extern "C" fn half_debuff_set_set_impl(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _value: f32,
    _method: OptionalMethod,
) {
    if let Some(unit) = unit {
        History::private_skill(unit);
        set_half_debuff(unit, "Str");
        set_half_debuff(unit, "Mag");
        unit.update();
    }
}

fn set_half_debuff(who: &Unit, which_debuff: impl AsRef<str>) {
    let debuff_giver_sid = format!("SID_{}Halved", which_debuff.as_ref());
    let debuff_effect_sid = format!("{}Effect", debuff_giver_sid.as_str());
    if who.has_sid(debuff_giver_sid.as_str().into()) {
        if who.has_sid(debuff_effect_sid.as_str().into()) {
            who.remove_private_sid(debuff_effect_sid.as_str());
        } else {
            who.add_sid(debuff_effect_sid.as_str());
            who.set_variable("DebuffAnimationFlag", 1);
        }
    }
}
