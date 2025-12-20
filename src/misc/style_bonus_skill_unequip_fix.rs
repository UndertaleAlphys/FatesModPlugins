use engage::gamedata::skill::SkillData;
use engage::gamedata::unit::Unit;
use engage::gamedata::Gamedata;
use std::collections::HashSet;
use std::sync::OnceLock;
use unity::prelude::OptionalMethod;

#[skyline::hook(offset = 0x01a36f10)]
fn unit_remove_equip_skill(unit: &Unit, skill: &SkillData, method: OptionalMethod) {
    let remove_sid = get_type_bonus_parent(skill.sid.to_string());
    if let Some(skill) = SkillData::get(remove_sid) {
        call_original!(unit, skill, method);
    }
}

fn get_type_bonus_parent(sid: impl AsRef<str>) -> String {
    static BATTLE_TYPES: OnceLock<HashSet<&str>> = OnceLock::new();
    let battle_types = BATTLE_TYPES.get_or_init(|| {
        const TYPE_STR: &[&str] = &[
            "連携", "騎馬", "隠密", "重装", "飛行", "魔法", "気功", "竜族",
        ];
        TYPE_STR.into_iter().map(|&t| t.into()).collect()
    });
    let sid = sid.as_ref().to_string();
    if let Some(pos) = sid.rfind('_') {
        if battle_types.contains(&sid[pos + 1..]) {
            let new_sid = sid[..pos].to_string();
            let inheritance_mark = new_sid.clone() + "_InheritanceMark";
            if SkillData::get(inheritance_mark).is_some() {
                return new_sid;
            }
        }
    }
    sid
}

pub fn install() {
    skyline::install_hook!(unit_remove_equip_skill);
}
