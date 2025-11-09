use engage::{battle::BattleInfo, gamedata::unit::Unit};
#[repr(C)]
pub struct AISimulatorBase {
    pub offense: &'static mut Unit,
    pub offense_index: i32,
    pub defense: &'static mut Unit,
    pub battle_info: &'static mut BattleInfo,
    pub score: u32,
    pub score_p4: u8,
}
#[repr(C)]
#[unity::class("App", "AIInterferenceSimulator")]
pub struct AIInterferenceSimulator {
    pub base_class: AISimulatorBase,
    pub is_not_suitable: bool,
    pub flag: i32,
}
