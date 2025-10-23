use engage::gamedata::JobData;
use skyline::hooks::InlineCtx;
use unity::prelude::OptionalMethod;

use crate::class::*;

#[skyline::hook(offset = 0x02056CA0)]
fn get_class_learn_skill_level(this: &JobData, _method: OptionalMethod) -> i32 {
    this.get_class_learn_skill_level()
}

// Do not auto learn class skill when creating unit.
#[skyline::hook(offset = 0x01A08BA8, inline)]
fn create_unit_impl(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[0].w.as_mut() = 100 };
}

#[skyline::hook(offset = 0x01A0D138, inline)]
fn create_unit_impl_exclude_internal_level(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[0].w.as_mut() = 100 };
}

pub fn install() {
    skyline::install_hooks!(
        get_class_learn_skill_level,
        create_unit_impl,
        create_unit_impl_exclude_internal_level,
    );
}
