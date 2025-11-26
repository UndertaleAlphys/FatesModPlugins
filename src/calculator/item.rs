use crate::calculator::command;
use crate::calculator::util::{CalculatorManagerTrait, ListFloats};
use crate::item::ItemTrait;
use crate::util::class::UnityClassTrait;
use engage::calculator::CalculatorManager;
use engage::{calculator::GameCalculatorCommand, gamedata::unit::Unit};
use unity::prelude::{Il2CppString, OptionalMethod};

pub fn add(manager: &mut CalculatorManager) {
    let inventory_item_count_c = manager.clone_from_name(command::MALE_FEMALE_COUNTS);
    if let Some(inventory_item_count_c) = inventory_item_count_c {
        inventory_item_count_c
            .assign_virtual_method("get_Name", get_inventory_item_count_command_name as _);
        inventory_item_count_c.assign_vtable(34, inventory_item_count as _);
        manager.add_command(inventory_item_count_c);
    }
}

extern "C" fn get_inventory_item_count_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    // Usage: InventoryItemCount(type)
    command::INVENTORY_ITEM_COUNT.into()
}

extern "C" fn inventory_item_count(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    args: &ListFloats,
    _method_info: OptionalMethod,
) -> f32 {
    let type_list_length = args.size as usize;
    if unit.is_none() || type_list_length < 1 {
        0.0
    } else {
        let mut cnt = 0;
        let unit = unit.unwrap();
        for idx in 0..8 {
            let item = unit.item_list.get_item(idx);
            if item.is_none() {
                continue;
            }
            let item = item.unwrap();
            if !item.item.is_engage_weapon() || unit.is_engaging() {
                for idx in 0..type_list_length {
                    let t = args.items[idx];
                    if t == item.item.kind as f32 {
                        cnt += 1;
                        break;
                    }
                }
            }
        }
        cnt as f32
    }
}
