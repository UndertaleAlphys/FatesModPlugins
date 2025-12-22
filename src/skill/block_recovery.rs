use crate::unit::skill::UnitSkillTrait;
use engage::gamedata::unit::Unit;
use skyline::hooks::InlineCtx;

pub fn install() {
    skyline::install_hook!(unit_revive);
}

#[skyline::hook(offset = 0x01f339c0, inline)]
fn unit_revive(ctx: &mut InlineCtx) {
    let unit = unsafe { &*((*ctx.registers[0].x.as_ref()) as *const Unit) };
    let mut percentage: Option<i32> = None;
    if let Some(mask) = unit.mask_skill {
        for skill in mask.iter() {
            let value = skill.get_skill().and_then(|s| {
                s.sid
                    .to_string()
                    .strip_prefix("SID_RecoveryBlocked_")?
                    .parse::<i32>()
                    .ok()
            });
            if value.is_some() {
                unit.remove_private_sid(skill.get_skill().unwrap().sid.to_string());
                percentage = value;
                break;
            }
        }
    }
    if let Some(percentage) = percentage {
        let mut heal_hp = unsafe { *ctx.registers[1].w.as_ref() } as i32;
        heal_hp = (heal_hp * percentage / 100).clamp(1, heal_hp.max(1));
        unsafe { *ctx.registers[1].w.as_mut() = heal_hp as u32 };
    }
}
