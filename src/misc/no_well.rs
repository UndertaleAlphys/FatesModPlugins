use crate::menu;
use unity::prelude::OptionalMethod;

#[skyline::hook(offset = 0x01B2DC20)]
fn item_exchange_menu_build_attribute(_this: i64, _method_info: OptionalMethod) -> i32 {
    menu::DISABLED
}

pub fn install() {
    skyline::install_hook!(item_exchange_menu_build_attribute);
}
