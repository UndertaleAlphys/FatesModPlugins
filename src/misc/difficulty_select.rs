use crate::menu;
use crate::menu::{MainMenuHard, MainMenuNormal};
use crate::util::class::UnityClassTrait;
use engage::menu::BasicMenuItem;
use unity::prelude::OptionalMethod;

extern "C" fn build_attribute_hidden(_this: &BasicMenuItem, _method: OptionalMethod) -> i32 {
    menu::HIDEN
}

#[skyline::hook(offset = 0x02300EB0)]
fn main_menu_normal_ctor(this: &mut MainMenuNormal, method: OptionalMethod) {
    call_original!(this, method);
    this.assign_vtable(8, build_attribute_hidden as _);
}

#[skyline::hook(offset = 0x02300600)]
fn main_menu_hard_ctor(this: &mut MainMenuHard, method: OptionalMethod) {
    call_original!(this, method);
    this.assign_vtable(8, build_attribute_hidden as _);
}

#[skyline::hook(offset = 0x0209AB00)]
fn my_room_difficulty_change(this: &BasicMenuItem, method: OptionalMethod) -> i32 {
    build_attribute_hidden(this, method)
}

pub fn install() {
    skyline::install_hooks!(
        // main_menu_normal_ctor,
        // main_menu_hard_ctor,
        // my_room_difficulty_change
    );
}
