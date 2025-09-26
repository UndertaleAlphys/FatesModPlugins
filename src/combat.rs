pub mod ai;
pub mod battle_info;
mod engage_count;
mod no_dynlv;
mod silence;
mod skill_point;

pub fn install() {
    engage_count::install();
    skill_point::install();
    silence::install();
    no_dynlv::install();
}