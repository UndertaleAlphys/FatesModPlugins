use crate::map::MapImageTerrainTrait;
use crate::terrain::TerrainTrait;
use crate::unit::UnitTrait;
use engage::gamedata::terrain::TerrainData;
use engage::gamedata::unit::Unit;
use engage::map::image::MapImage;
use engage::map::overlap::MapOverlap;
use engage::util::get_instance;

pub trait UnitTerrainTrait {
    fn is_terrain_valid(&self, terrain: &TerrainData) -> bool;
    fn get_terrain_avo(&self) -> i32;
    fn get_terrain_def(&self) -> i32;
    fn get_terrain_heal(&self) -> i32;
    fn get_terrain_mov(&self) -> i32;
    fn is_terrain_immune_to_break(&self) -> bool;
}

impl UnitTerrainTrait for Unit {
    fn is_terrain_valid(&self, terrain: &TerrainData) -> bool {
        if terrain.is_fly_enabled() {
            true
        } else if self.is_fly() {
            self.has_winged_shield()
        } else {
            true
        }
    }
    fn get_terrain_avo(&self) -> i32 {
        get_terrain_and_overlap_data(self, |t| {
            t.avoid
                + if self.is_enemy() {
                    t.enemy_avoid
                } else {
                    t.player_avoid
                }
        }) as i32
    }

    fn get_terrain_def(&self) -> i32 {
        get_terrain_and_overlap_data(self, |t| {
            t.defense
                + if self.is_enemy() {
                    t.enemy_defense
                } else {
                    t.player_defense
                }
        }) as i32
    }

    fn get_terrain_heal(&self) -> i32 {
        get_terrain_and_overlap_data(self, |t| t.heal) as i32
    }

    fn get_terrain_mov(&self) -> i32 {
        get_terrain_and_overlap_data(self, |t| t.move_first) as i32
    }

    fn is_terrain_immune_to_break(&self) -> bool {
        get_terrain_and_overlap_data(self, |t| t.is_immune_break() as i8) > 0
    }
}

pub fn get_terrain_and_overlap_data<F>(unit: &Unit, getter: F) -> i8
where
    F: Fn(&TerrainData) -> i8,
{
    let x = unit.get_x();
    let z = unit.get_z();
    let terrain = get_instance::<MapImage>().terrain.get_terrain(x, z);
    let overlap = MapOverlap::get_terrain(x, z);
    get_single_terrain_data(unit, terrain, &getter)
        + get_single_terrain_data(unit, overlap, &getter)
}

fn get_single_terrain_data<F>(unit: &Unit, t: Option<&TerrainData>, getter: F) -> i8
where
    F: Fn(&TerrainData) -> i8,
{
    t.map_or(0, |t| {
        if unit.is_terrain_valid(t) {
            getter(t)
        } else {
            0
        }
    })
}
