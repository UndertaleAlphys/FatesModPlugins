use engage::gamedata::terrain::TerrainData;
use unity::prelude::OptionalMethod;
mod limit_expansion;
pub mod percentage;
pub trait TerrainTrait {
    fn is_fly_enabled(&self) -> bool;
    fn is_immune_break(&self) -> bool;
    fn is_fire_cannon(&self) -> bool;
    fn is_magic_cannon(&self) -> bool;
    fn is_bow_cannon(&self) -> bool;
}

impl TerrainTrait for TerrainData {
    fn is_fly_enabled(&self) -> bool {
        unsafe { terrain_is_fly_enabled(self, None) }
    }
    fn is_immune_break(&self) -> bool {
        unsafe { terrain_data_is_immune_break(self, None) }
    }
    fn is_fire_cannon(&self) -> bool {
        unsafe { terrain_data_is_fire_cannon(self, None) }
    }
    fn is_magic_cannon(&self) -> bool {
        unsafe { terrain_data_is_magic_cannon(self, None) }
    }
    fn is_bow_cannon(&self) -> bool {
        unsafe { terrain_data_is_bow_cannon(self, None) }
    }
}

#[skyline::from_offset(0x021E33A0)]
fn terrain_is_fly_enabled(terrain: &TerrainData, method: OptionalMethod) -> bool;
#[skyline::from_offset(0x021E3380)]
fn terrain_data_is_immune_break(this: &TerrainData, method: OptionalMethod) -> bool;

#[skyline::from_offset(0x021e32f0)]
fn terrain_data_is_fire_cannon(this: &TerrainData, method: OptionalMethod) -> bool;

#[skyline::from_offset(0x021e32e0)]
fn terrain_data_is_magic_cannon(this: &TerrainData, method: OptionalMethod) -> bool;

#[skyline::from_offset(0x021e32d0)]
fn terrain_data_is_bow_cannon(this: &TerrainData, method: OptionalMethod) -> bool;

pub fn install() {
    percentage::install();
    limit_expansion::install();
}
