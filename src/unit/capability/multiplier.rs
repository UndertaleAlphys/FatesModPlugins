use crate::skill::SkillArrayTrait;
use crate::unit::capability;
use engage::gamedata::unit::Unit;
use skyline::hooks::InlineCtx;
use unity::prelude::*;

enum Multiple {
    M0_5,
    M1_0,
    M1_5,
}
trait CapabilityMultiplier {
    fn get_capability_no_api(&self, idx: i32) -> Option<i32>;
    fn get_final_enhance(&self, idx: i32, enhance: i32, multiple: Multiple) -> Option<i32>;
}
impl CapabilityMultiplier for Unit {
    fn get_capability_no_api(&self, idx: i32) -> Option<i32> {
        if capability::is_index_valid(idx) {
            let idx = idx as usize;
            let class = self.get_job();
            let unit_base = self.base_capability.capability[idx] as i32;
            let unit_limit = self.person.get_limit().data[idx] as i32;
            let class_base = class.get_base().data[idx] as i32;
            let class_limit = class.get_limit().data[idx] as i32;
            let base = unit_base + class_base;
            let limit = unit_limit + class_limit;
            Some(base.clamp(0, limit))
        } else {
            None
        }
    }
    fn get_final_enhance(&self, idx: i32, enhance: i32, multiple: Multiple) -> Option<i32> {
        if capability::is_index_valid(idx) {
            let capability_without_enhance = self.get_capability_no_api(idx).unwrap();
            let capability_before_multiply = capability_without_enhance + enhance;
            let capability_after_multiply = match multiple {
                Multiple::M0_5 => (capability_before_multiply + 1) / 2,
                Multiple::M1_0 => capability_before_multiply,
                Multiple::M1_5 => capability_before_multiply * 3 / 2,
            };
            Some(capability_after_multiply - capability_without_enhance)
        } else {
            None
        }
    }
}

#[skyline::hook(offset = 0x1A1C944, inline)]
fn commit_max_hp(ctx: &mut InlineCtx) {
    let this: &Unit = unsafe { &*(*ctx.registers[19].x.as_ref() as *const Unit) };
    let old_enhance_hp = unsafe { *ctx.registers[0].w.as_ref() } as i32;
    let multiple = if this.has_sid(Il2CppString::new("SID_禍事罪穢効果")) {
        Multiple::M0_5
    } else {
        Multiple::M1_0
    };
    let new_enhance_hp = this
        .get_final_enhance(capability::MAXHP, old_enhance_hp, multiple)
        .unwrap();
    unsafe {
        *ctx.registers[0].w.as_mut() = new_enhance_hp as u32;
    }
}

#[skyline::hook(offset = 0x1A1C978, inline)]
fn commit_str(ctx: &mut InlineCtx) {
    let this: &Unit = unsafe { &*(*ctx.registers[19].x.as_ref() as *const Unit) };
    let old_enhance_str = unsafe { *ctx.registers[0].w.as_ref() } as i32;
    let multiple = if this.has_sid(Il2CppString::new("SID_力半減_効果")) {
        Multiple::M0_5
    } else {
        Multiple::M1_0
    };
    let new_enhance_str = this
        .get_final_enhance(capability::STR, old_enhance_str, multiple)
        .unwrap();
    unsafe {
        *ctx.registers[0].w.as_mut() = new_enhance_str as u32;
    }
}
#[skyline::hook(offset = 0x1A1CA84, inline)]
fn commit_mag(ctx: &mut InlineCtx) {
    let this: &Unit = unsafe { &*(*ctx.registers[19].x.as_ref() as *const Unit) };
    let old_enhance_mag = unsafe { *ctx.registers[0].w.as_ref() } as i32;
    let multiple = if this.has_sid(Il2CppString::new("SID_魔半減_効果")) {
        Multiple::M0_5
    } else {
        Multiple::M1_0
    };
    let new_enhance_mag = this
        .get_final_enhance(capability::MAG, old_enhance_mag, multiple)
        .unwrap();
    unsafe {
        *ctx.registers[0].w.as_mut() = new_enhance_mag as u32;
    }
}
#[skyline::hook(offset = 0x1A1CB5C, inline)]
fn commit_mov(ctx: &mut InlineCtx) {
    let this: &Unit = unsafe { &*(*ctx.registers[19].x.as_ref() as *const Unit) };
    let old_enhance_mov = unsafe { *ctx.registers[0].w.as_ref() } as i32;
    let less_mov = this.has_sid(Il2CppString::new("SID_移動半減_効果"));
    let more_mov = this.has_sid(Il2CppString::new("SID_移動半分に増加_効果"));
    let multiple = if less_mov ^ more_mov {
        if less_mov {
            Multiple::M0_5
        } else {
            Multiple::M1_5
        }
    } else {
        Multiple::M1_0
    };
    let new_enhance_mov = this
        .get_final_enhance(capability::MOV, old_enhance_mov, multiple)
        .unwrap();
    unsafe {
        *ctx.registers[0].w.as_mut() = new_enhance_mov as u32;
    }
}
pub fn install() {
    skyline::install_hooks!(commit_max_hp, commit_str, commit_mag, commit_mov);
}
