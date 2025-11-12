use crate::unit::skill::UnitSkillTrait;
use engage::gamedata::skill::SkillData;
use engage::gamedata::Gamedata;
use engage::gamemessage::GameMessage;
use engage::mess::Mess;
use engage::sequence::unitgrowsequence::UnitGrowSequence;
use unity::prelude::OptionalMethod;
use unity::system::Il2CppString;

#[skyline::hook(offset = 0x01f7f580)]
fn unit_grow_sequence_class_change(this: &UnitGrowSequence, method: OptionalMethod) {
    let unit = this.unit;
    let previous_class_skill = unit.learned_job_skill;
    call_original!(this, method);
    if let Some(previous_class_skill) = previous_class_skill {
        unit.add_to_equip_skill_pool(previous_class_skill);
    }
    if let Some(current_class) = this.class_change_job {
        if let Some(current_class_sid) = current_class.learn_skill {
            if let Some(skill_data) = SkillData::get(current_class_sid) {
                unit.remove_equip_skill(skill_data);
                unit.remove_equip_pool_skill(skill_data);
                // if this.old_level >= current_class.max_level as i32 {
                //     let mess_id = skill_data.name.unwrap_or("".into());
                //     let skill_name: &Il2CppString = Mess::get(mess_id);
                //     Mess::set_argument(0, "");
                //     Mess::set_argument(1, skill_name);
                //     let learn_skill_mess: &Il2CppString = Mess::get("MID_MSG_LEARN_SKILL");
                //     println!("{}", learn_skill_mess);
                //     GameMessage::create_key_wait(this, learn_skill_mess);
                // }
            }
        }
    }
}

pub fn install() {
    skyline::install_hook!(unit_grow_sequence_class_change);
}
