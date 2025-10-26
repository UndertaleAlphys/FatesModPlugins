use engage::calculator::*;
use unity::prelude::OptionalMethod;
mod buff_eirike;
mod class;
mod command;
mod engage_turn;
mod item;
mod rally_flag;
mod terrain;
mod util;
mod variable;

#[unity::hook("App", "UnitCalculator", "AddCommand")]
pub fn add_command_hook(manager: &mut CalculatorManager, method_info: OptionalMethod) {
    call_original!(manager, method_info);
    terrain::add(manager);
    item::add(manager);
    class::add(manager);
    variable::add(manager);
    engage_turn::add(manager);
    rally_flag::add(manager);
    buff_eirike::add(manager);
}

pub fn install() {
    skyline::install_hook!(add_command_hook);
}
