use crate::calculator::command;
use crate::calculator::util::CalculatorManagerTrait;
use crate::unit::UnitTrait;
use crate::util::class::UnityClassTrait;
use engage::battle::BattleInfoSide;
use engage::calculator::{CalculatorManager, GameCalculatorCommand};
use engage::gamedata::unit::Unit;
use paste::paste;
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
        debuff_str, debuff_mag, debuff_dex, debuff_spd, debuff_lck, debuff_def, debuff_res,
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

#[function::gen(
    debuff_str, debuff_mag, debuff_dex, debuff_spd, debuff_lck, debuff_def, debuff_res
)]
extern "C" fn get___SNAKENAME___command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("__CAMELNAME__")
}

#[function::gen(
    debuff_str, debuff_mag, debuff_dex, debuff_spd, debuff_lck, debuff_def, debuff_res
)]
extern "C" fn get___SNAKENAME___unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("__CAMELNAME__") as f32)
}

#[function::gen(
    debuff_str, debuff_mag, debuff_dex, debuff_spd, debuff_lck, debuff_def, debuff_res
)]
extern "C" fn get___SNAKENAME___battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("__CAMELNAME__") as f32)
}

#[function::gen(
    debuff_str, debuff_mag, debuff_dex, debuff_spd, debuff_lck, debuff_def, debuff_res
)]
extern "C" fn set___SNAKENAME___unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("__CAMELNAME__", value as i32));
}

#[function::gen(
    debuff_str, debuff_mag, debuff_dex, debuff_spd, debuff_lck, debuff_def, debuff_res
)]
extern "C" fn set___SNAKENAME___battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("__CAMELNAME__", value as i32));
}
