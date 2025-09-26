use engage::gamedata::terrain::{self, TerrainData};
use unity::{il2cpp::method, prelude::OptionalMethod};
mod limit_expansion;
pub mod percentage;
pub trait TerrainTrait {
    fn is_fly_enabled(&self) -> bool;
}

impl TerrainTrait for TerrainData {
    fn is_fly_enabled(&self) -> bool {
        unsafe { terrain_is_fly_enabled(self, None) }
    }
}
#[skyline::from_offset(0x021E33A0)]
fn terrain_is_fly_enabled(terrain: &TerrainData, method: OptionalMethod) -> bool;
pub fn install() {
    percentage::install();
    limit_expansion::install();
}
