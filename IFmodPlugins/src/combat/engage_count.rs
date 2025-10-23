use engage::battle::BattleCalculator;
use unity::prelude::*;
#[skyline::hook(offset = 0x02470740)]
fn add_engage_count_1(calculator: &BattleCalculator, side: i32, method: OptionalMethod) {}

#[skyline::hook(offset = 0x024709D0)]
fn add_engage_count_2(calculator: &BattleCalculator, side: i32, method: OptionalMethod) {}

pub fn install() {
    skyline::install_hooks!(add_engage_count_1, add_engage_count_2,);
}