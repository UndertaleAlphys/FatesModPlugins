use engage::map::image::MapImage;
use engage::util::get_instance;
use engage::{
    gamedata::{skill::SkillData, terrain::TerrainData, unit::Unit},
    map::image::MapImageTerrain,
    mapmind::MapMind,
};
use unity::{il2cpp::object::Il2CppArray, prelude::*};

pub struct Map;

impl Map {
    pub fn can_enter_terrain(unit: &Unit, x: i32, z: i32) -> bool {
        unsafe { map_can_enter_terrain(unit, x, z, None) }
    }

    pub fn get_play_area() -> ((i32, i32), (i32, i32)) {
        let image: &MapImage = get_instance::<MapImage>();
        let min_x = image.playarea_x1.min(image.playarea_x2);
        let max_x = image.playarea_x1.max(image.playarea_x2);
        let min_z = image.playarea_z1.min(image.playarea_z2);
        let max_z = image.playarea_z1.max(image.playarea_z2);
        ((min_x, min_z), (max_x, max_z))
    }

    pub fn is_in_play_area(x: i32, z: i32) -> bool {
        let ((min_x, min_z), (max_x, max_z)) = Self::get_play_area();
        min_x <= x && x <= max_x && min_z <= z && z <= max_z
    }
}

pub struct MapSkill;

impl MapSkill {
    pub fn is_sight_out(unit: &Unit, x: i32, z: i32) -> bool {
        unsafe { map_skill_is_sight_out(unit, x, z, None) }
    }
}

#[repr(C)]
#[unity::class("App", "MapSequenceMind")]
pub struct MapSequenceMind {
    _junk: [u8; 0x60],
    pub unit: Option<&'static mut Unit>,
    pub target: Option<&'static mut Unit>,
    pub is_move_only: bool,
}

#[repr(C)]
#[unity::class("App", "MapDeployTemplate")]
pub struct MapDeployTemplate {
    _junk0: [u8; 0x38],
    pub move_image: &'static mut MapDeployMoveImage,
    _junk1: [u8; 0x158],
    pub unit: &'static mut Unit,
}

#[repr(C)]
#[unity::class("App", "MapDeployMoveImage")]
pub struct MapDeployMoveImage {
    pub array: &'static Il2CppArray<u8>,
    pub display: i32,
}

impl MapDeployMoveImage {
    pub fn get_value(&self, x: i32, z: i32) -> Option<u8> {
        if x >= 0 && x < 32 && z >= 0 && z < 32 {
            let offset = (z * 32 + x) as usize;
            if offset < self.array.max_length {
                Some(self.array[offset])
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub trait MapMindTrait {
    fn get_unit(&self) -> Option<&Unit>;
}

impl MapMindTrait for MapMind {
    fn get_unit(&self) -> Option<&Unit> {
        unsafe { map_mind_get_unit(self, None) }
    }
}

#[repr(C)]
pub struct MapSkillResult {
    pub moved: bool,
    pub unit: Option<&'static Unit>,
    pub x: i32,
    pub z: i32,
}

impl MapSkillResult {
    pub fn init(&mut self) {
        self.moved = false;
        self.x = 0;
        self.z = 0;
        self.unit = None;
    }
    pub fn assign_unit_unchecked(&mut self, unit: &'static Unit) {
        self.moved = false;
        self.unit = Some(unit);
        self.x = unit.x as i32;
        self.z = unit.z as i32;
    }
    pub fn assign_unit(&mut self, unit: Option<&'static Unit>) {
        if unit.is_some() {
            self.assign_unit_unchecked(unit.unwrap());
        } else {
            self.init();
        }
    }
}

#[repr(C)]
pub struct MapSkillResults {
    pub skill: Option<&'static SkillData>,
    pub current: MapSkillResult,
    pub reverse: MapSkillResult,
}

impl MapSkillResults {
    pub fn init(&mut self) {
        self.skill = None;
        self.current.init();
        self.reverse.init();
    }
}

pub trait MapImageTerrainTrait {
    fn get_terrain(&self, x: i32, z: i32) -> Option<&TerrainData>;
}

impl MapImageTerrainTrait for MapImageTerrain {
    fn get_terrain(&self, x: i32, z: i32) -> Option<&TerrainData> {
        unsafe { map_image_terrain_get_terrain_data(self, x, z, None) }
    }
}

// #[unity::class("App", "MapTarget")]
// pub struct MapTarget {
//     _junk0: [u8; 0x10],
//     pub unit: &'static Unit,
//     pub x: u8,
//     pub z: u8,
//     _junk1: [u8; 0x36],
//     pub command_skill: Option<&'static SkillData>,
// }

#[skyline::from_offset(0x01EECF90)]
fn map_can_enter_terrain(unit: &Unit, x: i32, z: i32, method: OptionalMethod) -> bool;

#[skyline::from_offset(0x01DEE2B0)]
fn map_mind_get_unit(this: &MapMind, method: OptionalMethod) -> Option<&Unit>;

#[skyline::from_offset(0x01F4EA60)]
fn map_skill_is_sight_out(unit: &Unit, x: i32, z: i32, method: OptionalMethod) -> bool;

#[skyline::from_offset(0x02064ED0)]
fn map_image_terrain_get_terrain_data(
    this: &MapImageTerrain,
    x: i32,
    z: i32,
    method: OptionalMethod,
) -> Option<&TerrainData>;
