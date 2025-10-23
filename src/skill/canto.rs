use std::sync::atomic::{self, AtomicI32};

use crate::{
    map::{MapDeployMoveImage, MapDeployTemplate, MapMindTrait, MapSequenceMind},
    unit::{capability, status},
    util::bitmask::BitMask,
};
use engage::gamedata::unit::Unit;
use skyline::hooks::InlineCtx;
use unity::prelude::*;

static LAST_MOVE: AtomicI32 = AtomicI32::new(0);
#[skyline::hook(offset = 0x02C33BF0)]
pub fn get_distance(template: &MapDeployTemplate, x: i32, z: i32, method: OptionalMethod) -> i32 {
    let mov_spent = template.move_image.get_value(x, z);
    if let Some(mov_spent) = mov_spent {
        let mov_spent = mov_spent as i32;
        if !template.unit.status.value.contains(status::CANTO) {
            LAST_MOVE.store(mov_spent, atomic::Ordering::Relaxed);
        }
        mov_spent
    } else {
        0
    }
}

#[skyline::hook(offset = 0x02C1E7F0)]
pub fn unit_move_xz(
    template: &MapDeployTemplate,
    unit: &Unit,
    x: i32,
    z: i32,
    move_power: i32,
    flag: i64,
    weapon_flag: i64,
    method_info: OptionalMethod,
) {
    let real_move_power = if unit.status.value.contains(status::CANTO) {
        const CANTO_SID: &'static str = "SID_再移動＋＋";
        if unit.has_sid(Il2CppString::new(CANTO_SID)) {
            let unit_mov = unit.get_capability(capability::MOV, true);
            (unit_mov - LAST_MOVE.load(atomic::Ordering::Relaxed)).max(0)
        } else {
            move_power
        }
    } else {
        move_power
    };
    call_original!(
        template,
        unit,
        x,
        z,
        real_move_power,
        flag,
        weapon_flag,
        method_info
    )
}

pub fn install() {
    skyline::install_hooks!(get_distance, unit_move_xz,);
}
