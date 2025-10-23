use crate::item::{heal_override, ItemTrait};
use engage::gamedata::item::ItemData;
use skyline::hooks::InlineCtx;
#[skyline::hook(offset = 0x01DEDAB0, inline)]
fn heal_override(ctx: &mut InlineCtx) {
    // Get Item Data
    let item = unsafe { &*(*ctx.registers[1].x.as_ref() as *const ItemData) };
    let heal_overrided = item.get_heal_overrided();
    // Assign Overrided Value
    if let Some(heal_overrided) = heal_overrided {
        unsafe { *ctx.registers[8].w.as_mut() = heal_overrided as u32 }
    }
}
pub fn install() {
    skyline::install_hook!(heal_override);
}
