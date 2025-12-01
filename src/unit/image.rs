use crate::map::Map;
use crate::util::bitmask::BitMask;
use engage::gamedata::unit::Unit;
use unity::prelude::Il2CppArray;

pub trait UnitImageGet {
    fn attack_image(&self) -> Option<&'static MapImageCoreBit>;
    fn move_image(&self) -> Option<&'static MapImageCoreBit>;
    fn interference_image(&self) -> Option<&'static MapImageCoreBit>;
    fn heal_image(&self) -> Option<&'static MapImageCoreBit>;
    fn is_in_image(
        &self,
        x: i32,
        z: i32,
        image: fn(&Unit) -> Option<&'static MapImageCoreBit>,
    ) -> bool;
}

impl UnitImageGet for Unit {
    fn attack_image(&self) -> Option<&'static MapImageCoreBit> {
        address_to_map_image(self.attack_image)
    }
    fn move_image(&self) -> Option<&'static MapImageCoreBit> {
        address_to_map_image(self.move_image)
    }
    fn interference_image(&self) -> Option<&'static MapImageCoreBit> {
        address_to_map_image(self.interference_image)
    }
    fn heal_image(&self) -> Option<&'static MapImageCoreBit> {
        address_to_map_image(self.heal_image)
    }
    fn is_in_image(
        &self,
        x: i32,
        z: i32,
        image: fn(&Unit) -> Option<&'static MapImageCoreBit>,
    ) -> bool {
        if !Map::is_in_play_area(x, z) {
            return false;
        }
        let image = image(self);
        let Some(image) = image else { return false };
        image.get(x, z)
    }
}

#[inline]
fn address_to_map_image(addr: u64) -> Option<&'static MapImageCoreBit> {
    if addr != 0 {
        let ptr = addr as *const MapImageCoreBit;
        unsafe { Some(&*ptr) }
    } else {
        None
    }
}

#[unity::class("App", "MapImageCoreBit")]
pub struct MapImageCoreBit {
    pub images: &'static Il2CppArray<u32>,
}

impl MapImageCoreBit {
    fn get(&self, x: i32, z: i32) -> bool {
        assert!(x >= 0 && x < 32 && z >= 0 && z < 32);
        // z = set
        // x = block
        let block_value = self.images[z as usize];
        (block_value >> x & 1) == 1
    }
}
