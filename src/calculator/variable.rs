use crate::calculator::command;
use crate::calculator::util::CalculatorManagerTrait;
use crate::unit::UnitTrait;
use crate::util::class::UnityClassTrait;
use engage::battle::BattleInfoSide;
use engage::calculator::{CalculatorManager, GameCalculatorCommand};
use engage::gamedata::unit::Unit;
use unity::prelude::{Il2CppString, OptionalMethod};

#[macro_export]
macro_rules! gen_vecs {
    ([$($base:ident),+ $(,)?]) => {{
        ::paste::paste! {
            let name: Vec<*mut u8> = vec![
                $(
                    [<get_ $base _command_name>] as *mut u8
                ),+
            ];
            let get_impl: Vec<*mut u8> = vec![
                $(
                    [<get_ $base _unit>] as *mut u8
                ),+
            ];
            let get_impl_battle: Vec<*mut u8> = vec![
                $(
                    [<get_ $base _battle_info>] as *mut u8
                ),+
            ];
            let set_impl: Vec<*mut u8> = vec![
                $(
                    [<set_ $base _unit>] as *mut u8
                ),+
            ];
            let set_impl_battle: Vec<*mut u8> = vec![
                $(
                    [<set_ $base _battle_info>] as *mut u8
                ),+
            ];
            (name, get_impl, get_impl_battle, set_impl, set_impl_battle)
        }
    }};
}

pub fn add(manager: &mut CalculatorManager) {
    let (name, get_impl, get_impl_battle, set_impl, set_impl_battle) = gen_vecs!([
        debuff_str,
        debuff_mag,
        debuff_dex,
        debuff_spd,
        debuff_lck,
        debuff_def,
        debuff_res,
        stackable_new_debuff_str,
        stackable_new_debuff_mag,
        stackable_new_debuff_dex,
        stackable_new_debuff_spd,
        stackable_new_debuff_lck,
        stackable_new_debuff_def,
        stackable_new_debuff_res,
        non_stackable_new_debuff_str,
        non_stackable_new_debuff_mag,
        non_stackable_new_debuff_dex,
        non_stackable_new_debuff_spd,
        non_stackable_new_debuff_lck,
        non_stackable_new_debuff_def,
        non_stackable_new_debuff_res,
        debuff_animation_flag
    ]);
    for index in 0..name.len() {
        if let Some(debuff_c) = manager.clone_from_name(command::HP) {
            debuff_c.assign_virtual_method("get_Name", name[index]);
            debuff_c.assign_virtual_method("GetImpl", get_impl[index]);
            debuff_c.assign_vtable(31, get_impl_battle[index]);
            debuff_c.assign_virtual_method("SetImpl", set_impl[index]);
            debuff_c.assign_vtable(33, set_impl_battle[index]);
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
    Il2CppString::new("DebuffStr")
}

extern "C" fn get_debuff_str_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("DebuffStr") as f32)
}

extern "C" fn get_debuff_str_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("DebuffStr") as f32)
}

extern "C" fn set_debuff_str_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("DebuffStr", value as i32));
}

extern "C" fn set_debuff_str_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("DebuffStr", value as i32));
}

extern "C" fn get_debuff_mag_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("DebuffMag")
}

extern "C" fn get_debuff_mag_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("DebuffMag") as f32)
}

extern "C" fn get_debuff_mag_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("DebuffMag") as f32)
}

extern "C" fn set_debuff_mag_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("DebuffMag", value as i32));
}

extern "C" fn set_debuff_mag_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("DebuffMag", value as i32));
}

extern "C" fn get_debuff_dex_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("DebuffDex")
}

extern "C" fn get_debuff_dex_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("DebuffDex") as f32)
}

extern "C" fn get_debuff_dex_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("DebuffDex") as f32)
}

extern "C" fn set_debuff_dex_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("DebuffDex", value as i32));
}

extern "C" fn set_debuff_dex_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("DebuffDex", value as i32));
}

extern "C" fn get_debuff_spd_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("DebuffSpd")
}

extern "C" fn get_debuff_spd_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("DebuffSpd") as f32)
}

extern "C" fn get_debuff_spd_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("DebuffSpd") as f32)
}

extern "C" fn set_debuff_spd_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("DebuffSpd", value as i32));
}

extern "C" fn set_debuff_spd_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("DebuffSpd", value as i32));
}

extern "C" fn get_debuff_lck_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("DebuffLck")
}

extern "C" fn get_debuff_lck_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("DebuffLck") as f32)
}

extern "C" fn get_debuff_lck_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("DebuffLck") as f32)
}

extern "C" fn set_debuff_lck_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("DebuffLck", value as i32));
}

extern "C" fn set_debuff_lck_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("DebuffLck", value as i32));
}

extern "C" fn get_debuff_def_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("DebuffDef")
}

extern "C" fn get_debuff_def_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("DebuffDef") as f32)
}

extern "C" fn get_debuff_def_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("DebuffDef") as f32)
}

extern "C" fn set_debuff_def_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("DebuffDef", value as i32));
}

extern "C" fn set_debuff_def_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("DebuffDef", value as i32));
}

extern "C" fn get_debuff_res_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("DebuffRes")
}

extern "C" fn get_debuff_res_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("DebuffRes") as f32)
}

extern "C" fn get_debuff_res_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("DebuffRes") as f32)
}

extern "C" fn set_debuff_res_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("DebuffRes", value as i32));
}

extern "C" fn set_debuff_res_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("DebuffRes", value as i32));
}

extern "C" fn get_stackable_new_debuff_str_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("StackableNewDebuffStr")
}

extern "C" fn get_stackable_new_debuff_str_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("StackableNewDebuffStr") as f32)
}

extern "C" fn get_stackable_new_debuff_str_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("StackableNewDebuffStr") as f32)
}

extern "C" fn set_stackable_new_debuff_str_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("StackableNewDebuffStr", value as i32));
}

extern "C" fn set_stackable_new_debuff_str_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("StackableNewDebuffStr", value as i32));
}

extern "C" fn get_stackable_new_debuff_mag_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("StackableNewDebuffMag")
}

extern "C" fn get_stackable_new_debuff_mag_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("StackableNewDebuffMag") as f32)
}

extern "C" fn get_stackable_new_debuff_mag_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("StackableNewDebuffMag") as f32)
}

extern "C" fn set_stackable_new_debuff_mag_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("StackableNewDebuffMag", value as i32));
}

extern "C" fn set_stackable_new_debuff_mag_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("StackableNewDebuffMag", value as i32));
}

extern "C" fn get_stackable_new_debuff_dex_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("StackableNewDebuffDex")
}

extern "C" fn get_stackable_new_debuff_dex_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("StackableNewDebuffDex") as f32)
}

extern "C" fn get_stackable_new_debuff_dex_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("StackableNewDebuffDex") as f32)
}

extern "C" fn set_stackable_new_debuff_dex_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("StackableNewDebuffDex", value as i32));
}

extern "C" fn set_stackable_new_debuff_dex_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("StackableNewDebuffDex", value as i32));
}

extern "C" fn get_stackable_new_debuff_spd_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("StackableNewDebuffSpd")
}

extern "C" fn get_stackable_new_debuff_spd_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("StackableNewDebuffSpd") as f32)
}

extern "C" fn get_stackable_new_debuff_spd_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("StackableNewDebuffSpd") as f32)
}

extern "C" fn set_stackable_new_debuff_spd_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("StackableNewDebuffSpd", value as i32));
}

extern "C" fn set_stackable_new_debuff_spd_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("StackableNewDebuffSpd", value as i32));
}

extern "C" fn get_stackable_new_debuff_lck_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("StackableNewDebuffLck")
}

extern "C" fn get_stackable_new_debuff_lck_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("StackableNewDebuffLck") as f32)
}

extern "C" fn get_stackable_new_debuff_lck_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("StackableNewDebuffLck") as f32)
}

extern "C" fn set_stackable_new_debuff_lck_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("StackableNewDebuffLck", value as i32));
}

extern "C" fn set_stackable_new_debuff_lck_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("StackableNewDebuffLck", value as i32));
}

extern "C" fn get_stackable_new_debuff_def_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("StackableNewDebuffDef")
}

extern "C" fn get_stackable_new_debuff_def_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("StackableNewDebuffDef") as f32)
}

extern "C" fn get_stackable_new_debuff_def_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("StackableNewDebuffDef") as f32)
}

extern "C" fn set_stackable_new_debuff_def_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("StackableNewDebuffDef", value as i32));
}

extern "C" fn set_stackable_new_debuff_def_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("StackableNewDebuffDef", value as i32));
}

extern "C" fn get_stackable_new_debuff_res_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("StackableNewDebuffRes")
}

extern "C" fn get_stackable_new_debuff_res_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("StackableNewDebuffRes") as f32)
}

extern "C" fn get_stackable_new_debuff_res_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("StackableNewDebuffRes") as f32)
}

extern "C" fn set_stackable_new_debuff_res_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("StackableNewDebuffRes", value as i32));
}

extern "C" fn set_stackable_new_debuff_res_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("StackableNewDebuffRes", value as i32));
}

extern "C" fn get_non_stackable_new_debuff_str_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("NonStackableNewDebuffStr")
}

extern "C" fn get_non_stackable_new_debuff_str_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("NonStackableNewDebuffStr") as f32)
}

extern "C" fn get_non_stackable_new_debuff_str_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("NonStackableNewDebuffStr") as f32)
}

extern "C" fn set_non_stackable_new_debuff_str_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("NonStackableNewDebuffStr", value as i32));
}

extern "C" fn set_non_stackable_new_debuff_str_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("NonStackableNewDebuffStr", value as i32));
}

extern "C" fn get_non_stackable_new_debuff_mag_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("NonStackableNewDebuffMag")
}

extern "C" fn get_non_stackable_new_debuff_mag_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("NonStackableNewDebuffMag") as f32)
}

extern "C" fn get_non_stackable_new_debuff_mag_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("NonStackableNewDebuffMag") as f32)
}

extern "C" fn set_non_stackable_new_debuff_mag_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("NonStackableNewDebuffMag", value as i32));
}

extern "C" fn set_non_stackable_new_debuff_mag_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("NonStackableNewDebuffMag", value as i32));
}

extern "C" fn get_non_stackable_new_debuff_dex_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("NonStackableNewDebuffDex")
}

extern "C" fn get_non_stackable_new_debuff_dex_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("NonStackableNewDebuffDex") as f32)
}

extern "C" fn get_non_stackable_new_debuff_dex_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("NonStackableNewDebuffDex") as f32)
}

extern "C" fn set_non_stackable_new_debuff_dex_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("NonStackableNewDebuffDex", value as i32));
}

extern "C" fn set_non_stackable_new_debuff_dex_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("NonStackableNewDebuffDex", value as i32));
}

extern "C" fn get_non_stackable_new_debuff_spd_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("NonStackableNewDebuffSpd")
}

extern "C" fn get_non_stackable_new_debuff_spd_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("NonStackableNewDebuffSpd") as f32)
}

extern "C" fn get_non_stackable_new_debuff_spd_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("NonStackableNewDebuffSpd") as f32)
}

extern "C" fn set_non_stackable_new_debuff_spd_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("NonStackableNewDebuffSpd", value as i32));
}

extern "C" fn set_non_stackable_new_debuff_spd_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("NonStackableNewDebuffSpd", value as i32));
}

extern "C" fn get_non_stackable_new_debuff_lck_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("NonStackableNewDebuffLck")
}

extern "C" fn get_non_stackable_new_debuff_lck_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("NonStackableNewDebuffLck") as f32)
}

extern "C" fn get_non_stackable_new_debuff_lck_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("NonStackableNewDebuffLck") as f32)
}

extern "C" fn set_non_stackable_new_debuff_lck_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("NonStackableNewDebuffLck", value as i32));
}

extern "C" fn set_non_stackable_new_debuff_lck_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("NonStackableNewDebuffLck", value as i32));
}

extern "C" fn get_non_stackable_new_debuff_def_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("NonStackableNewDebuffDef")
}

extern "C" fn get_non_stackable_new_debuff_def_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("NonStackableNewDebuffDef") as f32)
}

extern "C" fn get_non_stackable_new_debuff_def_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("NonStackableNewDebuffDef") as f32)
}

extern "C" fn set_non_stackable_new_debuff_def_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("NonStackableNewDebuffDef", value as i32));
}

extern "C" fn set_non_stackable_new_debuff_def_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("NonStackableNewDebuffDef", value as i32));
}

extern "C" fn get_non_stackable_new_debuff_res_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("NonStackableNewDebuffRes")
}

extern "C" fn get_non_stackable_new_debuff_res_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("NonStackableNewDebuffRes") as f32)
}

extern "C" fn get_non_stackable_new_debuff_res_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("NonStackableNewDebuffRes") as f32)
}

extern "C" fn set_non_stackable_new_debuff_res_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("NonStackableNewDebuffRes", value as i32));
}

extern "C" fn set_non_stackable_new_debuff_res_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("NonStackableNewDebuffRes", value as i32));
}

extern "C" fn get_debuff_animation_flag_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("DebuffAnimationFlag")
}

extern "C" fn get_debuff_animation_flag_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("DebuffAnimationFlag") as f32)
}

extern "C" fn get_debuff_animation_flag_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("DebuffAnimationFlag") as f32)
}

extern "C" fn set_debuff_animation_flag_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("DebuffAnimationFlag", value as i32));
}

extern "C" fn set_debuff_animation_flag_battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("DebuffAnimationFlag", value as i32));
}
