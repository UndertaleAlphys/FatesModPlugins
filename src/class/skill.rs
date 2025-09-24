use engage::gamedata::JobData;
use unity::prelude::OptionalMethod;

use crate::class::*;

#[skyline::hook(offset = 0x02056CA0)]
pub fn get_class_learn_skill_level(this: &JobData, _method: OptionalMethod) -> i32 {
    this.get_class_learn_skill_level()
}

pub fn install() {
    skyline::install_hook!(get_class_learn_skill_level);
}
