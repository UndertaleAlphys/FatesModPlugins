use crate::calculator::command;
use crate::calculator::util::CalculatorManagerTrait;
use crate::combat::battle_info::BattleInfoSideTrait;
use crate::skill::SkillArrayTrait;
use crate::unit::capability;
use crate::util::class::UnityClassTrait;
use engage::battle::BattleInfoSide;
use engage::calculator::{CalculatorManager, GameCalculatorCommand};
use unity::prelude::{Il2CppString, OptionalMethod};

pub fn add(manager: &mut CalculatorManager) {
    if let Some(follow_up_ability_c) = manager.clone_from_name(command::HP) {
        follow_up_ability_c.assign_virtual_method("get_Name", follow_up_ability_command_name as _);
        follow_up_ability_c.assign_vtable(31, follow_up_ability_get_battle_info as _);
        manager.add_with_reverse(follow_up_ability_c);
    }
}

extern "C" fn follow_up_ability_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::FOLLOW_UP_ABILITY.into()
}

extern "C" fn follow_up_ability_get_battle_info(
    _this: &GameCalculatorCommand,
    battle_info: &BattleInfoSide,
    _method: OptionalMethod,
) -> f32 {
    calc_follow_up_ability(battle_info) as f32
}

#[skyline::from_offset(0x01e89400)]
fn battle_info_side_get_spd(side: &BattleInfoSide, method: OptionalMethod) -> i32;

fn calc_value_by_prefix(side: &BattleInfoSide, prefix: impl AsRef<str>) -> i32 {
    let mut result = 0;
    for skill in side.mask_skill.iter() {
        if let Some(skill) = skill.get_skill() {
            let sid = skill.sid.to_string();
            let offset = sid
                .strip_prefix(prefix.as_ref())
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(0);
            result += offset;
        }
    }
    result
}

#[inline]
fn calc_follow_up_offset(side: &BattleInfoSide) -> i32 {
    calc_value_by_prefix(side, "SID_FollowUpOffset_")
        + calc_value_by_prefix(side.reverse, "SID_FoeFollowUpOffset_")
        + calc_value_by_prefix(side, "SID_BothFollowUpOffset_")
        + calc_value_by_prefix(side.reverse, "SID_BothFollowUpOffset_")
}

#[inline]
fn calc_darting_blow_offset(side: &BattleInfoSide) -> i32 {
    const OFFENSE: i32 = 0;
    if side.side_type == OFFENSE && side.mask_skill.contains_sid("SID_飛燕の一撃") {
        5
    } else {
        0
    }
}

fn cannot_follow_up(side: &BattleInfoSide) -> bool {
    let cannot_follow_up = "SID_追撃不可";
    let wary_fighter = "SID_守備隊形";
    side.mask_skill.contains_sid(cannot_follow_up)
        || side.mask_skill.contains_sid(wary_fighter)
        || side.reverse.mask_skill.contains_sid(wary_fighter)
}

fn calc_follow_up_ability(side: &BattleInfoSide) -> i32 {
    if side.get_unit().is_none() || cannot_follow_up(side) {
        0
    } else {
        let unit_spd = unsafe { battle_info_side_get_spd(side, None) };
        let foe_spd = unsafe { battle_info_side_get_spd(side.reverse, None) };
        let mut result = unit_spd - foe_spd;
        result += calc_follow_up_offset(side);
        result += calc_darting_blow_offset(side);
        let unit = side.get_unit().unwrap();
        let unit_hp = unit.get_hp();
        let unit_max_hp = unit.get_capability(capability::MAXHP, true);
        if result < 5 {
            const DEFENSE: i32 = 1;
            if side.side_type == DEFENSE {
                if unit_hp * 100 >= unit_max_hp * 75 && side.mask_skill.contains_sid("SID_切り返し")
                {
                    result = 5;
                } else if unit_hp * 100 >= unit_max_hp * 50
                    && side.mask_skill.contains_sid("SID_切り返し＋")
                {
                    result = 5;
                }
            }
        }
        result
    }
}
