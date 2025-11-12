use engage::sequence::levelupsequence::LevelUpSequence;
use unity::prelude::OptionalMethod;

#[skyline::hook(offset = 0x01be94b0)]
fn level_up_sequence_learn_class_skill(this: &LevelUpSequence, method: OptionalMethod) {
    call_original!(this, method);
}
pub fn install() {
    skyline::install_hook!(level_up_sequence_learn_class_skill);
}
