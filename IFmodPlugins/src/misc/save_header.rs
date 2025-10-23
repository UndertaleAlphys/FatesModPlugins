use skyline::hooks::InlineCtx;
#[skyline::hook(offset = 0x02514AC4, inline)]
fn save_header_fix(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[8].w.as_mut() = 1 << 0x1F }
}

pub fn install() {
    skyline::install_hook!(save_header_fix);
}
