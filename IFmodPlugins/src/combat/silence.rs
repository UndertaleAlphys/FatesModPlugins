use crate::{
    combat::{
        ai::{AIInterferenceSimulator, AISimulatorBase},
        battle_info::{BattleInfoSideTrait, BattleInfoTrait},
    },
    item::{use_type, ItemListTrait, ItemTrait},
    util::bitmask::BitMask,
};
use engage::{
    battle::{BattleInfo, BattleInfoSide},
    gamedata::item::{ItemData, UnitItem},
};
use skyline::hooks::InlineCtx;
use unity::prelude::*;
#[skyline::hook(offset = 0x01A46970, inline)]
pub fn silence_magic_weapon(ctx: &mut InlineCtx) {
    // Guarantee Existence
    let item = unsafe { &*(*ctx.registers[23].x.as_ref() as *const ItemData) };
    if item.is_silence_target() {
        unsafe { *ctx.registers[9].w.as_mut() = 6 };
    }
}

#[skyline::hook(offset = 0x01A46984, inline)]
pub fn silence_engage_attack(ctx: &mut InlineCtx) {
    let skill_flag = unsafe { *ctx.registers[9].w.as_ref() as u32 };
    let skill_flag = skill_flag.remove(2);
    unsafe { *ctx.registers[9].w.as_mut() = skill_flag };
}

#[skyline::hook(offset = 0x01930740)]
pub fn interference_cal_score(ai: &mut AIInterferenceSimulator, method: OptionalMethod) {
    call_original!(ai, method);
    let rod_equipped = ai.base_class.battle_info.get_side().get_unit_item();
    if rod_equipped.item.usetype != use_type::SILENCE {
        return;
    }
    let defense = &ai.base_class.defense;
    let item_list = &defense.item_list;
    for item_idx in 0..item_list.unit_items.len() as i32 {
        let unit_item = item_list.get_item(item_idx);
        if let Some(unit_item) = unit_item {
            if defense.can_equip(item_idx, true, true) {
                if unit_item.item.is_silence_target() {
                    ai.base_class.score_p4 = 0;
                    break;
                }
            }
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        // silence_magic_weapon,
        // silence_engage_attack,
        interference_cal_score
    );
}
