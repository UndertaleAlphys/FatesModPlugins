use engage::calculator::*;
use unity::prelude::OptionalMethod;
mod buff_eirike;
mod class;
mod command;
mod engage_count;
mod engage_turn;
mod half_debuff_set;
mod item;
mod none;
mod personal_skill;
mod rally_flag;
mod terrain;
mod util;
mod variable;

#[unity::hook("App", "UnitCalculator", "AddCommand")]
pub fn add_command_hook(manager: &mut CalculatorManager, method_info: OptionalMethod) {
    call_original!(manager, method_info);
    none::add(manager);
    terrain::add(manager);
    item::add(manager);
    class::add(manager);
    variable::add(manager);
    engage_turn::add(manager);
    rally_flag::add(manager);
    buff_eirike::add(manager);
    half_debuff_set::add(manager);
    engage_count::add(manager);
    personal_skill::add(manager);
}

pub fn install() {
    skyline::install_hook!(add_command_hook);
}
