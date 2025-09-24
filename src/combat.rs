pub mod ai;
pub mod battle_info;
mod engage_count;
mod silence;

pub fn install() {
    engage_count::install();
    silence::install();
}