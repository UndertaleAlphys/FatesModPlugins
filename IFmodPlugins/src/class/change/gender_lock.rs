use crate::class::{ClassTrait, GetClassGenderLock};
use engage::gamedata::{skill::SkillArrayEntityList, unit::Unit, JobData};
use skyline::hooks::InlineCtx;

fn class_change_check_get_unit(ctx: &InlineCtx) -> &Unit {
    unsafe { &*(*ctx.registers[20].x.as_ref() as *const Unit) }
}

fn class_change_check_get_job_data(ctx: &InlineCtx) -> &JobData {
    unsafe {
        let x19 = *ctx.registers[19].x.as_ref() as *const *const JobData;
        let job_data_ptr = *x19.byte_add(0x10);
        &*job_data_ptr
    }
}

#[skyline::hook(offset = 0x19C6E40, inline)]
fn check_male_gender(ctx: &mut InlineCtx) {
    let unit = class_change_check_get_unit(ctx);
    let job_data = class_change_check_get_job_data(ctx);
    if let Some(gender_lock) = job_data.get_gender_lock() {
        if gender_lock != unit.get_gender() {
            unsafe {
                *ctx.registers[24].w.as_mut() = 0;
            }
        }
    }
}

pub fn install() {
    skyline::install_hook!(check_male_gender);
}
