use crate::calculator::command;
use crate::calculator::util::{CalculatorManagerTrait, ListFloats};
use crate::class::ClassTrait;
use crate::unit::UnitTrait;
use crate::util::class::UnityClassTrait;
use engage::calculator::{CalculatorManager, GameCalculatorCommand};
use engage::gamedata::unit::Unit;
use engage::map::image::MapImage;
use engage::util::get_instance;
use unity::prelude::{Il2CppString, OptionalMethod};

pub fn add(manager: &mut CalculatorManager) {
    let around_class_move_type_count_c = manager.clone_from_name(command::MALE_FEMALE_COUNTS);
    if let Some(around_class_move_type_count_c) = around_class_move_type_count_c {
        around_class_move_type_count_c
            .assign_virtual_method("get_Name", around_class_move_type_command_name as _);
        around_class_move_type_count_c.assign_vtable(34, around_class_move_type as _);
        manager.add_command(around_class_move_type_count_c);
    }
}

fn around_class_move_type_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    // Usage: AroundClassMoveType(range, moveType_1, moveType_2, ...)
    command::AROUND_CLASS_MOVE_TYPE_COUNT.into()
}

fn around_class_move_type(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    args: &ListFloats,
    _method_info: OptionalMethod,
) -> f32 {
    let type_list_length = args.size as usize;
    if unit.is_none() || type_list_length <= 1 {
        return 0.0;
    }
    let unit = unit.unwrap();
    if !unit.is_on_map() || !unit.is_in_play_area() {
        return 0.0;
    }
    let mut cnt = 0;
    let center = (unit.get_x(), unit.get_z());
    let range = args.items[0] as i32;
    let battle_style = args.items[1..]
        .iter()
        .map(|f| *f as i32)
        .collect::<Vec<i32>>();
    let image: &MapImage = get_instance::<MapImage>();
    for (x, z) in iter_range(range, center) {
        if let Some(around_unit) = image.get_target_unit(x, z) {
            if around_unit.is_enemy() == unit.is_enemy() {
                if battle_style.contains(&(around_unit.get_job().get_battle_style())) {
                    cnt += 1;
                }
            }
        }
    }
    cnt as f32
}

fn iter_range(range: i32, center: (i32, i32)) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    if range > 0 {
        let (center_x, center_z) = center;
        let start_x = center_x;
        let start_z = center_z - range;
        for i in 0..range + 1 {
            for j in 0..range + 1 {
                let point = (start_x - i + j, start_z + i + j);
                if point != center {
                    result.push(point);
                }
            }
        }

        let start_x = center_x;
        let start_z = center_z - range + 1;
        for i in 0..range {
            for j in 0..range {
                let point = (start_x - i + j, start_z + i + j);
                if point != center {
                    result.push(point);
                }
            }
        }
    }
    result
}
