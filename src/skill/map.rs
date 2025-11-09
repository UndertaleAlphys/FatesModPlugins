use crate::{
    map::{Map, MapSkill, MapSkillResult, MapSkillResults},
    skill::{flag, SkillTrait},
    unit::UnitTrait,
    util::bitmask::BitMask,
};
use engage::{
    gamedata::{skill::SkillData, unit::Unit},
    map::image::MapImage,
    util::get_instance,
};
use skyline::{hooks::InlineCtx, patching::Patch};

#[skyline::hook(offset = 0x01F4E160)]
fn map_skill_prediction_impl(
    current: Option<&'static Unit>,
    reverse: Option<&'static Unit>,
    skill: Option<&'static SkillData>,
    results: &mut MapSkillResults,
) -> bool {
    //Init
    results.current.assign_unit(current);
    results.reverse.assign_unit(reverse);
    results.skill = skill;
    //Check... If any object DNE
    if current.is_none() || skill.is_none() {
        return false;
    }
    let skill = skill.unwrap();
    let move_self = skill.get_move_self();
    let move_target = skill.get_move_target();
    let movement_involved = move_self != 0 || move_target != 0;
    if !movement_involved {
        return true;
    }
    if reverse.is_none() {
        return move_target == 0;
    }
    let current = current.unwrap();
    let reverse = reverse.unwrap();
    let current_center = current.get_center_cell();
    let reverse_center = reverse.get_center_cell();
    if current_center.is_none() || reverse_center.is_none() {
        return false;
    }
    let current_center = current_center.unwrap();
    let reverse_center = reverse_center.unwrap();
    let x_diff = reverse_center.0 - current_center.0;
    let z_diff = reverse_center.1 - current_center.1;
    let is_before_move = skill.get_flag().contains(flag::BEFORE_MOVE);
    predict_single_unit(
        current,
        reverse,
        move_self,
        x_diff,
        z_diff,
        is_before_move,
        true,
        &mut results.current,
    );
    predict_single_unit(
        reverse,
        current,
        move_target,
        x_diff,
        z_diff,
        is_before_move,
        false,
        &mut results.reverse,
    );
    results.current.moved =
        current.get_x() != results.current.x || current.get_z() != results.current.z;
    results.reverse.moved =
        reverse.get_x() != results.reverse.x || reverse.get_z() != results.reverse.z;
    // println!(
    //     "self: ({},{})->({},{})",
    //     current.get_x(),
    //     current.get_z(),
    //     results.current.x,
    //     results.current.z
    // );
    // println!(
    //     "target: ({},{})->({},{})",
    //     reverse.get_x(),
    //     reverse.get_z(),
    //     results.reverse.x,
    //     results.reverse.z
    // );
    let final_result =
        results.current.moved == (move_self != 0) && results.reverse.moved == (move_target != 0);
    // if final_result {
    //     println!("Moved");
    // } else {
    //     println!("Not Moved");
    // }
    final_result
}

fn predict_single_unit(
    current: &Unit,
    reverse: &Unit,
    mov: i32,
    x_diff: i32,
    z_diff: i32,
    is_before_move: bool,
    is_offense: bool,
    result: &mut MapSkillResult,
) {
    let pos = predict_pos(current, reverse, mov, x_diff, z_diff, is_before_move);
    let x = pos.0;
    let z = pos.1;
    if Map::can_enter_terrain(current, x, z)
        && (!is_offense || !MapSkill::is_sight_out(current, x, z))
    {
        let map_image: &MapImage = get_instance::<MapImage>();
        let unit = map_image.get_target_unit(x, z);
        let no_other_unit_here = if let Some(unit) = unit {
            unit.index == current.index || unit.index == reverse.index
        } else {
            true
        };
        if no_other_unit_here {
            result.x = x;
            result.z = z;
        }
    }
}

fn predict_pos(
    current: &Unit,
    reverse: &Unit,
    mov: i32,
    x_diff: i32,
    z_diff: i32,
    is_before_move: bool,
) -> (i32, i32) {
    if current.can_be_moved() && mov != 0 {
        if is_before_move {
            predict_before_move(current, reverse, mov)
        } else {
            predict_normal(current, mov, x_diff, z_diff)
        }
    } else {
        (current.get_x(), current.get_z())
    }
}

fn predict_normal(current: &Unit, mov: i32, x_diff: i32, z_diff: i32) -> (i32, i32) {
    (
        current.get_x() + mov * x_diff,
        current.get_z() + mov * z_diff,
    )
}

fn predict_before_move(current: &Unit, reverse: &Unit, mov: i32) -> (i32, i32) {
    let map_size = reverse.get_person().get_bmap_size() as i32;
    if reverse.get_x() <= current.get_x() && current.get_x() < reverse.get_x() + map_size {
        let z = if current.get_z() > reverse.get_z() {
            reverse.get_z() + map_size
        } else {
            reverse.get_z() - 1
        };
        let diff = current.get_z() - z;
        if diff.abs() <= mov.abs() {
            (current.get_x(), z)
        } else {
            (current.get_x(), current.get_z())
        }
    } else if reverse.get_z() <= current.get_z() && current.get_z() < reverse.get_z() + map_size {
        let x = if current.get_x() > reverse.get_x() {
            reverse.get_x() + map_size
        } else {
            reverse.get_x() - 1
        };
        let diff = current.get_x() - x;
        if diff.abs() <= mov.abs() {
            (x, current.get_z())
        } else {
            (current.get_x(), current.get_z())
        }
    } else {
        (current.get_x(), current.get_z())
    }
}

#[skyline::hook(offset = 0x023BC480, inline)]
fn enable_resurrection_unit_move(ctx: &mut InlineCtx) {
    let result = unsafe {
        &mut *(*ctx.registers[0].x.as_ref() as *const MapSkillResult as *mut MapSkillResult)
    };
    let unit = result.unit;
    let reset_unit = if let Some(unit) = unit {
        if unit.can_revive() {
            false
        } else {
            true
        }
    } else {
        true
    };
    if reset_unit {
        result.init();
    }
}

#[skyline::hook(offset = 0x02C2A584, inline)]
fn before_move_range_i(ctx: &mut InlineCtx) {
    let range = unsafe { *ctx.registers[8].w.as_ref() };
    let move_self = unsafe { *ctx.registers[8].w.as_ref() };
    let range_i = range - move_self + 1;
    unsafe { *ctx.registers[8].w.as_mut() = range_i }
}

#[skyline::hook(offset = 0x02C2A590, inline)]
fn before_move_range_o(ctx: &mut InlineCtx) {
    let range = unsafe { *ctx.registers[8].w.as_ref() };
    let range_o = range + 0;
    unsafe { *ctx.registers[8].w.as_mut() = range_o }
}

#[skyline::hook(offset = 0x01DC53BC, inline)]
fn before_move_z_1(ctx: &mut InlineCtx) {
    let w8 = unsafe { *ctx.registers[8].w.as_ref() } as i32;
    let w9 = unsafe { *ctx.registers[9].w.as_ref() } as i32;
    let w24 = unsafe { *ctx.registers[24].w.as_ref() } as i32;
    let offset = if w9 < 0 { w9 + 1 } else { 0 };
    let result = w24 + w8 + offset;
    unsafe { *ctx.registers[24].w.as_mut() = result as u32 };
}

#[skyline::hook(offset = 0x01DC53A8, inline)]
fn before_move_z_2(ctx: &mut InlineCtx) {
    let w8 = unsafe { *ctx.registers[8].w.as_ref() } as i32;
    let w9 = unsafe { *ctx.registers[9].w.as_ref() } as i32;
    let w24 = unsafe { *ctx.registers[24].w.as_ref() } as i32;
    let offset = if w9 > 0 { w9 - 1 } else { 0 };
    let result = w24 - w8 + offset;
    unsafe { *ctx.registers[24].w.as_mut() = result as u32 };
}

#[skyline::hook(offset = 0x01DC53C4, inline)]
fn before_move_x_1(ctx: &mut InlineCtx) {
    let w8 = unsafe { *ctx.registers[8].w.as_ref() } as i32;
    let w10 = unsafe { *ctx.registers[10].w.as_ref() } as i32;
    let w25 = unsafe { *ctx.registers[25].w.as_ref() } as i32;
    let offset = if w10 < 0 { w10 + 1 } else { 0 };
    let result = w25 + w8 + offset;
    unsafe { *ctx.registers[25].w.as_mut() = result as u32 };
}

#[skyline::hook(offset = 0x01DC5240, inline)]
fn before_move_x_2(ctx: &mut InlineCtx) {
    let w8 = unsafe { *ctx.registers[8].w.as_ref() } as i32;
    let w10 = unsafe { *ctx.registers[10].w.as_ref() } as i32;
    let w25 = unsafe { *ctx.registers[25].w.as_ref() } as i32;
    let offset = if w10 > 0 { w10 - 1 } else { 0 };
    let result = w25 - w8 + offset;
    unsafe { *ctx.registers[25].w.as_mut() = result as u32 };
}

//3，4

#[skyline::hook(offset = 0x01E7941C, inline)]
fn before_move_z_3(ctx: &mut InlineCtx) {
    let w8 = unsafe { *ctx.registers[8].w.as_ref() } as i32;
    let w9 = unsafe { *ctx.registers[9].w.as_ref() } as i32;
    let w21 = unsafe { *ctx.registers[21].w.as_ref() } as i32;
    let offset = if w9 < 0 { w9 + 1 } else { 0 };
    let result = w21 + w8 + offset;
    unsafe { *ctx.registers[21].w.as_mut() = result as u32 };
}

#[skyline::hook(offset = 0x01E7940C, inline)]
fn before_move_z_4(ctx: &mut InlineCtx) {
    let w8 = unsafe { *ctx.registers[8].w.as_ref() } as i32;
    let w9 = unsafe { *ctx.registers[9].w.as_ref() } as i32;
    let w21 = unsafe { *ctx.registers[21].w.as_ref() } as i32;
    let offset = if w9 > 0 { w9 - 1 } else { 0 };
    let result = w21 - w8 + offset;
    unsafe { *ctx.registers[21].w.as_mut() = result as u32 };
}

#[skyline::hook(offset = 0x01E79420, inline)]
fn before_move_x_3(ctx: &mut InlineCtx) {
    let w8 = unsafe { *ctx.registers[8].w.as_ref() } as i32;
    let w10 = unsafe { *ctx.registers[10].w.as_ref() } as i32;
    let w23 = unsafe { *ctx.registers[23].w.as_ref() } as i32;
    let offset = if w10 < 0 { w10 + 1 } else { 0 };
    let result = w23 + w8 + offset;
    unsafe { *ctx.registers[23].w.as_mut() = result as u32 };
}

#[skyline::hook(offset = 0x01E793FC, inline)]
fn before_move_x_4(ctx: &mut InlineCtx) {
    let w8 = unsafe { *ctx.registers[8].w.as_ref() } as i32;
    let w10 = unsafe { *ctx.registers[10].w.as_ref() } as i32;
    let w23 = unsafe { *ctx.registers[23].w.as_ref() } as i32;
    let offset = if w10 > 0 { w10 - 1 } else { 0 };
    let result = w23 - w8 + offset;
    unsafe { *ctx.registers[23].w.as_mut() = result as u32 };
}
pub fn install() {
    skyline::install_hooks!(
        map_skill_prediction_impl,
        enable_resurrection_unit_move,
        before_move_range_i,
        before_move_range_o,
        before_move_x_1,
        before_move_x_2,
        before_move_z_1,
        before_move_z_2,
        before_move_x_3,
        before_move_x_4,
        before_move_z_3,
        before_move_z_4,
    );
    // nops
    Patch::in_text(0x023BC484)
        .bytes([0x1F, 0x20, 0x03, 0xD5])
        .unwrap();
}
