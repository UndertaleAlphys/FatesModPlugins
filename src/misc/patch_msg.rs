use crate::util::language;
use skyline::hooks::InlineCtx;
use unity::prelude::Il2CppString;

fn extra_patch_msg() -> String {
    let lan = language::get();
    let mod_name = "Fates";
    let mod_version =
        std::fs::read_to_string(r"sd:/engage/mods/IF mod (Cobalt)/v").unwrap_or("None".into());
    let mod_info = if lan == language::SIMPLIFIED_CHINESE {
        "永久免费"
    } else {
        "Free Forever"
    };
    let result = format!("\n{} v{}\n{}", mod_name, mod_version, mod_info);
    result
}
#[skyline::hook(offset = 0x022975BC, inline)]
pub fn edit_patch_name(ctx: &mut InlineCtx) {
    let patch_name = unsafe {
        &mut *((*ctx.registers[0].x.as_ref()) as *const Il2CppString as *mut Il2CppString)
    };
    let modified_name = patch_name.to_string() + &extra_patch_msg();
    let modified_name = Il2CppString::new(modified_name);
    unsafe {
        let ptr = modified_name as *const Il2CppString as u64;
        *ctx.registers[0].x.as_mut() = ptr
    };
}

pub fn install() {
    skyline::install_hook!(edit_patch_name);
}
