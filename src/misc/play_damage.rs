use crate::unit::capability;
use engage::calculator::GameCalculatorCommand;
use engage::gamedata::unit::Unit;
use skyline::patching::Patch;
use unity::prelude::OptionalMethod;

#[skyline::hook(offset = 0x01B481E0)]
fn calculator_set_hp_unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    if let Some(unit) = unit {
        let max_hp = unit.get_capability(capability::MAXHP, true);
        let mut value = value as i32;
        let mut play_damage = unit.get_hp() - value;
        if play_damage != 0 {
            if play_damage > 0 {
                if unit.has_sid("SID_毒無効".into()) {
                    play_damage = 0;
                } else if unit.has_sid("SID_毒半減".into()) {
                    play_damage = play_damage / 2;
                };
            }
            value = unit.get_hp() - play_damage;
            unsafe { unit_play_set_damage(unit, play_damage, false, false, None) };
            value = value.min(max_hp).clamp(1, u8::MAX as i32);
            unit.set_hp(value);
        }
    }
}

#[skyline::from_offset(0x01A2B940)]
fn unit_play_set_damage(
    unit: &Unit,
    damage: i32,
    can_die: bool,
    is_multi: bool,
    method: OptionalMethod,
);

pub fn install() {
    // NOP
    Patch::in_text(0x01A2B9D8)
        .bytes([0x1F, 0x20, 0x03, 0xD5])
        .unwrap();
    // CMP w21, #1->#0
    Patch::in_text(0x01A2BA0C)
        .bytes([0xBF, 0x02, 0x00, 0x71])
        .unwrap();
    // MOV w1, #1
    Patch::in_text(0x01A2BC60)
        .bytes([0x21, 0x00, 0x80, 0x52])
        .unwrap();
    // CMP w19, #0
    Patch::in_text(0x0235AECC)
        .bytes([0x7F, 0x02, 0x00, 0x71])
        .unwrap();
    // CMP w19, #0
    Patch::in_text(0x0235AEE8)
        .bytes([0x7F, 0x02, 0x00, 0x71])
        .unwrap();
    // CMP w19, #0
    Patch::in_text(0x0235A984)
        .bytes([0x7F, 0x02, 0x00, 0x71])
        .unwrap();
    // CMP w19, #0
    Patch::in_text(0x0235A9A8)
        .bytes([0x7F, 0x02, 0x00, 0x71])
        .unwrap();
    // NOP
    Patch::in_text(0x01E3C41C)
        .bytes([0x1F, 0x20, 0x03, 0xD5])
        .unwrap();
    // MOV w1, w20
    Patch::in_text(0x01E3C400)
        .bytes([0xE1, 0x03, 0x14, 0x2A])
        .unwrap();
    skyline::install_hook!(calculator_set_hp_unit);
}
