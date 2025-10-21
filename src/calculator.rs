use engage::calculator::*;
use unity::prelude::OptionalMethod;
mod class;
mod command;
mod debuff;
mod engage_turn;
mod item;
mod rally_flag;
mod terrain;
mod util;

#[unity::hook("App", "UnitCalculator", "AddCommand")]
pub fn add_command_hook(manager: &mut CalculatorManager, method_info: OptionalMethod) {
    call_original!(manager, method_info);
    terrain::add(manager);
    item::add(manager);
    class::add(manager);
    debuff::add(manager);
    engage_turn::add(manager);
    rally_flag::add(manager);
}

pub fn install() {
    skyline::install_hook!(add_command_hook);
}
