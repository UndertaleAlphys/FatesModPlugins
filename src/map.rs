use engage::{gamedata::unit::Unit, mapmind::MapMind};
use unity::{il2cpp::object::Il2CppArray, prelude::*};

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
    fn get_unit(&self) -> &Unit;
}

impl MapMindTrait for MapMind {
    fn get_unit(&self) -> &Unit {
        unsafe { map_mind_get_unit(self, None) }
    }
}

#[skyline::from_offset(0x01DEE2B0)]
fn map_mind_get_unit(this: &MapMind, method: OptionalMethod) -> &Unit;
