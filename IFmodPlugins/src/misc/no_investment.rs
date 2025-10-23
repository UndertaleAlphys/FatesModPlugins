use crate::menu;
use unity::prelude::OptionalMethod;

#[skyline::hook(offset = 0x01FCC160)]
fn investment_menu_build_attribute(_this: i64, _method_info: OptionalMethod) -> i32 {
    menu::DISABLED
}
pub fn install() {
    skyline::install_hook!(investment_menu_build_attribute);
}
