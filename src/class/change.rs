pub mod gender_lock;
pub mod level;
pub fn install() {
    level::install();
    gender_lock::install();
}