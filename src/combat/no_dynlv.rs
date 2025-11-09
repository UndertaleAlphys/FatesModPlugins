use skyline::hooks::InlineCtx;
use unity::prelude::*;
#[skyline::hook(offset = 0x02B4AFA0)]
fn gmap_get_avg_lv(_difficulty: i32, _sortie_count: i32, _method: OptionalMethod) -> i32 {
    1
}
// For Tri's mod
#[skyline::hook(offset = 0x01A0CDCC, inline)]
fn set_internal_level_1(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[0].w.as_mut() = 0 }
}

#[skyline::hook(offset = 0x01A0CE34, inline)]
fn set_internal_level_2(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[0].w.as_mut() = 0 }
}
pub fn install() {
    skyline::install_hooks!(gmap_get_avg_lv, set_internal_level_1, set_internal_level_2,);
}
