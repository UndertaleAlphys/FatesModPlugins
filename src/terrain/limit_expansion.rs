use skyline::hooks::InlineCtx;
#[skyline::hook(offset = 0x01DFE560, inline)]
fn terrain_limit_cmp(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[0].w.as_mut() = 0x79 };
}
pub fn install() {
    skyline::install_hooks!(terrain_limit_cmp);
}
