use crate::{item::ItemListTrait, unit::UnitTrait};
use engage::gamedata::{
    item::{ItemData, UnitItem, UnitItemList},
    unit::Unit,
};
use unity::prelude::OptionalMethod;
#[skyline::hook(offset = 0x01A4F4C0)]
fn clear_god_unit(this: &Unit, method: OptionalMethod) {
    call_original!(this, method);
    let equip_item = this.item_list.get_equipped_index();
    if let Some(equip_item) = equip_item {
        if !this.can_equip(equip_item, false, true) {
            this.item_list.take_off(equip_item);
            this.auto_equip();
        }
    }
}

#[skyline::hook(offset = 0x01A0C050)]
fn unit_get_weapon_level(
    this: &Unit,
    kind: i32,
    cal_enhance: bool,
    _method: OptionalMethod,
) -> i32 {
    this.get_weapon_level(kind, cal_enhance)
}

pub fn install() {
    skyline::install_hooks!(clear_god_unit, unit_get_weapon_level);
}
