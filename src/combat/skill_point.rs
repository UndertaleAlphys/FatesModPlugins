use engage::gamedata::unit::Unit;
use unity::prelude::OptionalMethod;
#[skyline::hook(offset = 0x01A39FE4)]
fn exp_to_sp(_this: &Unit, sp: i32, _method_info: OptionalMethod) -> i32 {
    sp
}
pub fn install() {
    skyline::install_hook!(exp_to_sp);
}
