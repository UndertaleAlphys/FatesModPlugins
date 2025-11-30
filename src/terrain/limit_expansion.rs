use skyline::hooks::InlineCtx;
#[skyline::hook(offset = 0x01DFE560, inline)]
fn terrain_limit_cmp(ctx: &mut InlineCtx) {
    let terrain_count = unsafe { *ctx.registers[0].w.as_mut() };
    if terrain_count < 150 {
        unsafe { *ctx.registers[0].w.as_mut() = 0x79 };
    }
}
pub fn install() {
    skyline::install_hooks!(terrain_limit_cmp);
}
