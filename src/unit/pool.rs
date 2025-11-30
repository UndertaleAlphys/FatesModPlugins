use engage::gamedata::unit::Unit;
use engage::unitpool::UnitPool;
use unity::prelude::OptionalMethod;

pub trait UnitPoolTrait {
    fn get_first(force_index: i32) -> Option<&'static Unit>;
}

impl UnitPoolTrait for UnitPool {
    fn get_first(force_index: i32) -> Option<&'static Unit> {
        unsafe { unit_pool_get_first(1 << force_index, force_index, None) }
    }
}

#[skyline::from_offset(0x01c54090)]
fn unit_pool_get_first(
    force_mask: u32,
    start_force_index: i32,
    method: OptionalMethod,
) -> Option<&'static Unit>;
