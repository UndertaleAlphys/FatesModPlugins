use crate::combat::battle_info::{BattleInfoSideTrait, BattleInfoTrait};
use crate::history::History;
use crate::item::ItemListTrait;
use engage::sequence::mapsequence::battle::MapSequenceBattle;
use unity::prelude::OptionalMethod;

pub fn install() {
    skyline::install_hooks!(map_sequence_battle_commit_battle);
}

#[skyline::hook(offset = 0x023B7A20)]
fn map_sequence_battle_commit_battle(this: &MapSequenceBattle, method: OptionalMethod) {
    let side = this.info.get_defense_side();
    if let Some(unit) = side.unit {
        let weapon = side.get_unit_item();
        let index = unit.item_list.unit_items.iter().position(|x| {
            if let Some(x) = x {
                x.index == weapon.index
            } else {
                false
            }
        });
        if let Some(index) = index {
            History::item_list(unit);
            unit.item_list.equip(index as i32);
        }
    }
    call_original!(this, method)
}
