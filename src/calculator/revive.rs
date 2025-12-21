use crate::calculator::command;
use crate::calculator::util::CalculatorManagerTrait;
use crate::util::class::UnityClassTrait;
use engage::calculator::{CalculatorManager, GameCalculatorCommand};
use engage::gamedata::unit::Unit;
use engage::script::DynValue;
use unity::prelude::{ArrayInstantiator, Il2CppArray, OptionalMethod};

pub fn add(manager: &mut CalculatorManager) {
    if let Some(revive_c) = manager.find_checked(command::REVIVE) {
        revive_c.assign_virtual_method("SetImpl", set_revive_stock_count_unit as _);
        manager.add_command(revive_c);
        if let Some(reverse) = revive_c.clone() {
            manager.add_command(reverse.reverse());
        }
    }
}

extern "C" fn set_revive_stock_count_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&mut Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    if let Some(unit) = unit {
        if value == 0.0 {
            let unit_to_be_killed = DynValue::new_number(unit.index as f64);
            let args = &mut [unit_to_be_killed];
            let args = Il2CppArray::<&mut DynValue>::from_slice(args).unwrap();
            unsafe { script_unit_die(args, None) };
        }
    }
}

#[skyline::from_offset(0x02199300)]
pub fn script_unit_die(args: &Il2CppArray<&mut DynValue>, method: OptionalMethod);
