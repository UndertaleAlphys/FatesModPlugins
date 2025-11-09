use engage::gamedata::terrain::TerrainData;
use engage::map::inspectors::CannonInspector;
use unity::prelude::OptionalMethod;

pub trait CannonInspectorTrait {
    fn get_terrain_data(&self) -> &TerrainData;
}

impl CannonInspectorTrait for CannonInspector {
    fn get_terrain_data(&self) -> &TerrainData {
        unsafe { cannon_inspector_get_terrain(self, None) }
    }
}

#[skyline::from_offset(0x025bae90)]
fn cannon_inspector_get_terrain(
    cannon_inspector: &CannonInspector,
    method: OptionalMethod,
) -> &TerrainData;
