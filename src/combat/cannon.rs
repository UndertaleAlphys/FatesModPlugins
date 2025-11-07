use engage::map::inspectors::{CannonInspector, MapInspector};
use engage::tmpro::TextMeshProUGUI;
use skyline::hooks::InlineCtx;
use unity::prelude::{Il2CppString, OptionalMethod};

pub fn install() {
    skyline::install_hooks!(
        cannon_inspector_decrease_shell,
        cannon_display,
        cannon_map_info_display,
        // cannon_range
    );
}

// Do not decrease shell
#[skyline::hook(offset = 0x025bb2a0)]
fn cannon_inspector_decrease_shell(_this: &CannonInspector, _method: OptionalMethod) {}

// Display as: INF
#[skyline::hook(offset = 0x01e446b0, inline)]
fn cannon_display(ctx: &mut InlineCtx) {
    let infinity: &'static Il2CppString = "∞".into();
    unsafe {
        *ctx.registers[0].x.as_mut() = infinity as *const _ as _;
    }
}

#[repr(C)]
#[unity::class("App", "MapInfoIconLocatorRoot")]
struct MapInfoIconLocatorRoot {
    _junk0: [u8; 0x18],
    value_text: &'static mut TextMeshProUGUI,
}

#[skyline::hook(offset = 0x02085570)]
fn cannon_map_info_display(
    this: &mut MapInfoIconLocatorRoot,
    inspector: &MapInspector,
    method: OptionalMethod,
) {
    call_original!(this, inspector, method);
    this.value_text.set_text("∞".into(), false);
}

#[skyline::hook(offset = 0x01f32f08, inline)]
fn cannon_range(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[0].x.as_mut() = true as _;
    }
}
