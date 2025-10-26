use crate::{
    map::MapMindTrait,
    menu::{self, OverlapSkillMenuItem},
    skill::SkillTrait,
};
use engage::{mapmind::MapMind, util::get_instance};
use unity::prelude::*;
#[skyline::hook(offset = 0x01E50340)]
fn overlap_menu_item_get_attr(this: &OverlapSkillMenuItem, method: OptionalMethod) -> i32 {
    let o_result = call_original!(this, method);
    let skill = this.skill;
    if let Some(skill) = skill {
        let mind: &MapMind = get_instance::<MapMind>();
        let unit = mind.get_unit();
        if let Some(unit) = unit {
            if skill.is_condition_true(unit, None) {
                o_result
            } else {
                menu::HIDEN
            }
        } else {
            o_result
        }
    } else {
        o_result
    }
}
pub fn install() {
    skyline::install_hooks!(overlap_menu_item_get_attr);
}
