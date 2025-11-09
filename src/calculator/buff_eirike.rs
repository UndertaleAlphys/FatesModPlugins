use crate::calculator::command;
use crate::calculator::util::CalculatorManagerTrait;
use crate::map::MapMindTrait;
use crate::unit::UnitTrait;
use crate::util::class::UnityClassTrait;
use engage::calculator::{CalculatorManager, GameCalculatorCommand};
use engage::gamedata::unit::Unit;
use engage::map::image::MapImage;
use engage::mapmind::MapMind;
use engage::util::get_instance;
use unity::prelude::{Il2CppString, OptionalMethod};

pub fn add(manager: &mut CalculatorManager) {
    let names = [
        around_buff_gentle_wind_command_name as *mut u8,
        around_buff_brave_sky_command_name as *mut u8,
    ];
    let set_impls = [
        around_buff_gentle_wind_set_impl as *mut u8,
        around_buff_brave_sky_set_impl as *mut u8,
    ];
    for index in 0..names.len() {
        if let Some(around_buff_c) = manager.clone_from_name(command::HP) {
            around_buff_c.assign_virtual_method("get_Name", names[index]);
            around_buff_c.assign_virtual_method("SetImpl", set_impls[index]);
            manager.add_command(around_buff_c);
        }
    }
}

extern "C" fn around_buff_gentle_wind_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::AROUND_BUFF_GENTLE_WIND.into()
}

extern "C" fn around_buff_brave_sky_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::AROUND_BUFF_BRAVE_SKY.into()
}

extern "C" fn around_buff_gentle_wind_set_impl(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    around_buff_giver(unit, "BuffGentleWind", (value as i32).clamp(0, 7), 3);
}

extern "C" fn around_buff_brave_sky_set_impl(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    around_buff_giver(unit, "BuffBraveSky", (value as i32).clamp(0, 7), 3);
}

fn around_buff_giver(
    unit: Option<&Unit>,
    variable_name: impl AsRef<str>,
    buff_amount: i32,
    range: i32,
) {
    if let Some(unit) = unit {
        let image: &MapImage = get_instance();
        let mut units = unit
            .iter_range(range)
            .iter()
            .map(|(x, z)| image.get_target_unit(*x, *z))
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect::<Vec<_>>();
        if let Some(unit) = get_instance::<MapMind>().get_unit() {
            units.push(unit)
        }
        for target_unit in units.iter() {
            if target_unit.is_enemy() == unit.is_enemy()
                && target_unit.get_variable(variable_name.as_ref()) < buff_amount
            {
                target_unit.set_variable(variable_name.as_ref(), buff_amount);
            }
        }
    }
}
