use skyline::hooks::InlineCtx;

#[skyline::hook(offset = 0x1A1D7D8, inline)]
fn bound_max_hp_emblem(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1D808, inline)]
fn bound_str_emblem(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1D83C, inline)]
fn bound_dex_emblem(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1D870, inline)]
fn bound_spd_emblem(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1D8A4, inline)]
fn bound_lck_emblem(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1D8DC, inline)]
fn bound_def_emblem(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1D914, inline)]
fn bound_mag_emblem(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1D948, inline)]
fn bound_res_emblem(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1D97C, inline)]
fn bound_bld_emblem(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1D9B4, inline)]
fn bound_sight_emblem(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1D9EC, inline)]
fn bound_mov_emblem(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1C934, inline)]
fn bound_max_hp(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1C964, inline)]
fn bound_str(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1C998, inline)]
fn bound_dex(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1C9CC, inline)]
fn bound_spd(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1CA00, inline)]
fn bound_lck(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1CA38, inline)]
fn bound_def(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1CA70, inline)]
fn bound_mag(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1CAA4, inline)]
fn bound_res(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1CAD8, inline)]
fn bound_bld(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1CB10, inline)]
fn bound_sight(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

#[skyline::hook(offset = 0x1A1CB48, inline)]
fn bound_mov(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].w.as_mut() = i8::MIN as u32;
        *ctx.registers[2].w.as_mut() = i8::MAX as u32;
    }
}

pub fn install() {
    skyline::install_hooks!(
        bound_max_hp_emblem,
        bound_str_emblem,
        bound_dex_emblem,
        bound_spd_emblem,
        bound_lck_emblem,
        bound_def_emblem,
        bound_mag_emblem,
        bound_res_emblem,
        bound_bld_emblem,
        bound_sight_emblem,
        bound_mov_emblem,
        bound_max_hp,
        bound_str,
        bound_dex,
        bound_spd,
        bound_lck,
        bound_def,
        bound_mag,
        bound_res,
        bound_bld,
        bound_sight,
        bound_mov,
    );
}
