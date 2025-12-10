use crate::unit::pool::UnitPoolTrait;
use engage::gamedata::unit::Unit;
use engage::unitpool::UnitPool;

pub const PLAYER: i32 = 0;
pub const ENEMY: i32 = 1;
pub const ALLY: i32 = 2;

pub struct ForceIterator {
    pub force_type: i32,
    current_index: i32,
    max_index: i32,
}

impl ForceIterator {
    pub fn new(force_type: i32) -> ForceIterator {
        let first_unit = UnitPool::get_first(force_type).unwrap();
        let first_index = first_unit.index as i32;
        ForceIterator {
            force_type,
            current_index: first_index,
            max_index: UnitPool::get_count(1 << force_type) + first_index - 1,
        }
    }
}

impl Iterator for ForceIterator {
    type Item = &'static Unit;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index <= self.max_index {
            let unit = UnitPool::get_by_index(self.current_index).map(|u| &*u);
            self.current_index += 1;
            unit
        } else {
            None
        }
    }
}
