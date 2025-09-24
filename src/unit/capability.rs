mod limit;
mod multiplier;

pub const MAXHP: i32 = 0;
pub const STR: i32 = 1;
pub const DEX: i32 = 2;
pub const SPD: i32 = 3;
pub const LCK: i32 = 4;
pub const DEF: i32 = 5;
pub const MAG: i32 = 6;
pub const RES: i32 = 7;
pub const BLD: i32 = 8;
pub const SIGHT: i32 = 9;
pub const MOV: i32 = 10;
pub fn is_index_valid(idx: i32) -> bool {
    idx <= MOV && idx >= MAXHP
}

pub fn install() {
    limit::install();
    multiplier::install();
}
