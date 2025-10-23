use crate::unit::capability;
use engage::gamedata::{terrain::TerrainData, unit::Unit};
use unity::prelude::OptionalMethod;
#[skyline::hook(offset = 0x01E3BB40)]
pub fn terrain_heal(
    this: Option<&Unit>,
    terrain: &TerrainData,
    method_info: OptionalMethod,
) -> i32 {
    let heal = call_original!(this, terrain, method_info);
    if let Some(unit) = this {
        unit.get_capability(capability::MAXHP, true) * heal / 100
    } else {
        heal
    }
}

#[skyline::hook(offset = 0x01E3B970)]
pub fn terrain_damage(
    this: Option<&Unit>,
    terrain: &TerrainData,
    method_info: OptionalMethod,
) -> i32 {
    let dmg = call_original!(this, terrain, method_info);
    if let Some(unit) = this {
        unit.get_capability(capability::MAXHP, true) * dmg / 100
    } else {
        dmg
    }
}

pub fn install() {
    skyline::install_hooks!(terrain_heal, terrain_damage);
}
