pub mod gender_lock;
pub mod level;
mod skill;

pub fn install() {
    level::install();
    skill::install();
    gender_lock::install();
}
