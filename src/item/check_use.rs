use crate::{
    item::{use_type, ItemTrait},
    unit::UnitTrait,
};
use engage::gamedata::{item::ItemData, skill::SkillArray, unit::Unit};
use skyline::hooks::InlineCtx;
use unity::prelude::*;

#[skyline::hook(offset = 0x01DEA700)]
fn can_item_use(
    unit: Option<&Unit>,
    item: Option<&ItemData>,
    target: Option<&Unit>,
    use_type: i32,
    skills: Option<&SkillArray>,
    method_info: OptionalMethod,
) -> bool {
    let o_result = call_original!(unit, item, target, use_type, skills, method_info);
    if unit.is_none() || item.is_none() || target.is_none() || use_type == 0 {
        o_result
    } else {
        // let unit = unit.unwrap();
        let item = item.unwrap();
        let target = target.unwrap();
        match item.usetype {
            use_type::HEAL => {
                if item.is_tiki_blessing() && !target.is_engage_meter_full() {
                    true
                } else {
                    o_result
                }
            }
            use_type::REST_HEAL => {
                if item.is_tiki_blessing() && !target.is_engage_meter_full() {
                    true
                } else {
                    o_result
                }
            }
            _ => o_result,
        }
    }
}

#[skyline::hook(offset = 0x01FCB0F0, inline)]
fn enable_sortie_item_use(ctx: &mut InlineCtx) {
    // Give Skill Command
    let flag = 1u64 << 31;
    unsafe { *ctx.registers[9].x.as_mut() |= flag };
}

pub fn install() {
    skyline::install_hooks!(can_item_use, enable_sortie_item_use);
}
