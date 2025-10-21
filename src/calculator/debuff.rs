use crate::calculator::command;
use crate::calculator::util::CalculatorManagerTrait;
use crate::unit::UnitTrait;
use crate::util::class::UnityClassTrait;
use engage::calculator::{CalculatorManager, GameCalculatorCommand};
use engage::gamedata::unit::Unit;
use unity::prelude::{Il2CppString, OptionalMethod};

pub fn add(manager: &mut CalculatorManager) {
    let name = vec![
        get_debuff_str_command_name as *mut u8,
        get_debuff_mag_command_name as _,
        get_debuff_dex_command_name as _,
        get_debuff_spd_command_name as _,
        get_debuff_lck_command_name as _,
        get_debuff_def_command_name as _,
        get_debuff_res_command_name as _,
    ];
    let get_impl = vec![
        get_debuff_str_unit as *mut u8,
        get_debuff_mag_unit as _,
        get_debuff_dex_unit as _,
        get_debuff_spd_unit as _,
        get_debuff_lck_unit as _,
        get_debuff_def_unit as _,
        get_debuff_res_unit as _,
    ];
    let set_impl = vec![
        set_debuff_str_unit as *mut u8,
        set_debuff_mag_unit as _,
        set_debuff_dex_unit as _,
        set_debuff_spd_unit as _,
        set_debuff_lck_unit as _,
        set_debuff_def_unit as _,
        set_debuff_res_unit as _,
    ];
    for index in 0..7 {
        if let Some(debuff_c) = manager.clone_from_name(command::HP) {
            debuff_c.assign_virtual_method("get_Name", name[index]);
            debuff_c.assign_virtual_method("GetImpl", get_impl[index]);
            debuff_c.assign_virtual_method("SetImpl", set_impl[index]);
            manager.add_command(debuff_c);
            if let Some(reverse) = debuff_c.clone() {
                manager.add_command(reverse.reverse());
            }
        }
    }
}

extern "C" fn get_debuff_str_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::DEBUFF_STR.into()
}

extern "C" fn get_debuff_mag_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::DEBUFF_MAG.into()
}

extern "C" fn get_debuff_dex_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::DEBUFF_DEX.into()
}

extern "C" fn get_debuff_spd_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::DEBUFF_SPD.into()
}

extern "C" fn get_debuff_lck_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::DEBUFF_LCK.into()
}

extern "C" fn get_debuff_def_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::DEBUFF_DEF.into()
}

extern "C" fn get_debuff_res_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::DEBUFF_RES.into()
}

extern "C" fn get_debuff_str_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_debuff("Str") as f32)
}

extern "C" fn get_debuff_mag_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_debuff("Mag") as f32)
}

extern "C" fn get_debuff_dex_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_debuff("Dex") as f32)
}

extern "C" fn get_debuff_spd_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_debuff("Spd") as f32)
}

extern "C" fn get_debuff_lck_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_debuff("Lck") as f32)
}

extern "C" fn get_debuff_def_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_debuff("Def") as f32)
}

extern "C" fn get_debuff_res_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_debuff("Res") as f32)
}

extern "C" fn set_debuff_str_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_debuff("Str", value as i32));
}

extern "C" fn set_debuff_mag_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_debuff("Mag", value as i32));
}

extern "C" fn set_debuff_dex_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_debuff("Dex", value as i32));
}

extern "C" fn set_debuff_spd_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_debuff("Spd", value as i32));
}

extern "C" fn set_debuff_lck_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_debuff("Lck", value as i32));
}

extern "C" fn set_debuff_def_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_debuff("Def", value as i32));
}

extern "C" fn set_debuff_res_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_debuff("Res", value as i32));
}
