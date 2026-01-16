use crate::class::*;
use crate::skill::SkillArrayTrait;
use engage::gamedata::{item::ItemData, unit::Unit, JobData};
use skyline::hooks::InlineCtx;

// CTX Getters
fn class_change_check_get_unit(ctx: &InlineCtx) -> &Unit {
    unsafe { &*(*ctx.registers[20].x.as_ref() as *const Unit) }
}

fn class_change_check_get_job_data(ctx: &InlineCtx) -> &JobData {
    unsafe {
        let x19 = *ctx.registers[19].x.as_ref() as *const *const JobData;
        let job_data_ptr = *x19.byte_add(0x10);
        &*job_data_ptr
    }
}

fn level_reset_get_unit(ctx: &InlineCtx) -> &Unit {
    unsafe { &*(*ctx.registers[19].x.as_ref() as *const Unit) }
}

fn level_reset_get_job_data(ctx: &InlineCtx) -> &JobData {
    unsafe { &*(*ctx.registers[20].x.as_ref() as *const JobData) }
}

fn disallow_high_to_low_chck(ctx: &InlineCtx) -> bool {
    let job_data = class_change_check_get_job_data(ctx);
    let unit = class_change_check_get_unit(ctx);
    if job_data.get_class_rank() == ClassRank::Base {
        match unit.get_job().get_class_rank() {
            ClassRank::Advanced => false,
            ClassRank::Special => unit.level < 20 && unit.level > 0,
            ClassRank::Base => unit.level > 0,
        }
    } else {
        unit.level > 0
    }
}

#[skyline::hook(offset = 0x19C6C6C, inline)]
fn disallow_high_to_low_impl(ctx: &mut InlineCtx) {
    let result = disallow_high_to_low_chck(ctx);
    unsafe { *ctx.registers[8].w.as_mut() = result as u32 };
}

#[skyline::hook(offset = 0x19C6C34, inline)]
fn disallow_high_to_low_disp(ctx: &mut InlineCtx) {
    let result = disallow_high_to_low_chck(ctx);
    let disp_lv = if result { 1 } else { 99 };
    unsafe { *ctx.registers[0].w.as_mut() = disp_lv };
}

#[skyline::hook(offset = 0x19C6AD8, inline)]
fn prevent_same_class_change(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[8].w.as_mut() = 0 }
}

#[skyline::hook(offset = 0x19C69C0, inline)]
fn prevent_same_class_change_normal_disp(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[0].w.as_mut() = 99 }
}

#[skyline::hook(offset = 0x19C6A68, inline)]
fn prevent_same_class_change_special_disp(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[0].w.as_mut() = 99 }
}

#[skyline::hook(offset = 0x1A088E4, inline)]
fn disable_level_addition_on_high_class(ctx: &mut InlineCtx) {
    let w19 = unsafe { *ctx.registers[19].w.as_ref() };
    unsafe { *ctx.registers[8].w.as_mut() = w19 };
}

#[skyline::hook(offset = 0x1A3C848, inline)]
fn level_reset(ctx: &mut InlineCtx) {
    let unit = level_reset_get_unit(ctx);
    let unit_class = unit.get_job();
    let job_data = level_reset_get_job_data(ctx);
    let unit_class_rank = unit_class.get_class_rank();
    let reset_level = match job_data.get_class_rank() {
        ClassRank::Advanced => match unit_class_rank {
            ClassRank::Advanced => unit.level,
            ClassRank::Special => unit.level.max(20),
            ClassRank::Base => 20,
        },
        ClassRank::Base => match unit_class_rank {
            ClassRank::Advanced => 1,
            ClassRank::Special => {
                if unit.level < 20 {
                    unit.level
                } else {
                    1
                }
            }
            ClassRank::Base => unit.level,
        },
        ClassRank::Special => unit.level,
    };
    unsafe { *ctx.registers[25].w.as_mut() = reset_level as u32 };
}

#[skyline::hook(offset = 0x1A3C7B0)]
pub fn class_change(
    this: &Unit,
    job_data: &JobData,
    item_data: &ItemData,
    method_info: OptionalMethod,
) {
    call_original!(this, job_data, item_data, method_info);
    let current_class = this.get_job();
    let should_learn = if let Some(learn_skill) = current_class.learn_skill {
        if SkillData::get(learn_skill).is_some() {
            if this.level >= current_class.max_level {
                true
            } else if this
                .equip_skill_pool
                .contains_sid(learn_skill.to_string().as_str())
            {
                true
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    };
    if should_learn {
        this.learn_job_skill(current_class);
    }
}

pub fn install() {
    skyline::install_hooks!(
        disallow_high_to_low_impl,
        disallow_high_to_low_disp,
        disable_level_addition_on_high_class,
        level_reset,
        prevent_same_class_change,
        prevent_same_class_change_normal_disp,
        prevent_same_class_change_special_disp,
        class_change,
    );
}
