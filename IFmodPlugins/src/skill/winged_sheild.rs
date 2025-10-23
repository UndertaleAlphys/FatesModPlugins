use crate::unit::terrain::UnitTerrainTrait;
use engage::gamedata::{
    terrain::{self, TerrainData},
    unit::Unit,
};
use skyline::hooks::InlineCtx;
use unity::prelude::OptionalMethod;
#[skyline::hook(offset = 0x01A34C90)]
fn is_terrain_invalid(unit: &Unit, terrain: &TerrainData, _method: OptionalMethod) -> bool {
    unit.is_terrain_valid(terrain)
}
#[skyline::hook(offset = 0x01E3B9E8, inline)]
fn phase_start_damage(ctx: &mut InlineCtx) {
    let terrain = unsafe { &*(*ctx.registers[19].x.as_ref() as *const TerrainData) };
    let unit = unsafe { &*(*ctx.registers[20].x.as_ref() as *const Unit) };
    unsafe { *ctx.registers[0].w.as_mut() = !unit.is_terrain_valid(terrain) as u32 };
}

#[skyline::hook(offset = 0x01E3BBA0, inline)]
fn phase_start_heal(ctx: &mut InlineCtx) {
    let terrain = unsafe { &*(*ctx.registers[19].x.as_ref() as *const TerrainData) };
    let unit = unsafe { &*(*ctx.registers[20].x.as_ref() as *const Unit) };
    unsafe { *ctx.registers[0].w.as_mut() = unit.is_terrain_valid(terrain) as u32 };
}

#[skyline::hook(offset = 0x01E44DDC, inline)]
fn map_terrain_single(ctx: &mut InlineCtx) {
    let terrain = unsafe { &*(*ctx.registers[28].x.as_ref() as *const TerrainData) };
    let unit = unsafe { &*(*ctx.registers[20].x.as_ref() as *const Unit) };
    unsafe { *ctx.registers[0].w.as_mut() = unit.is_terrain_valid(terrain) as u32 };
}

#[skyline::hook(offset = 0x01E7A938, inline)]
fn battle_set_terrain_1(ctx: &mut InlineCtx) {
    let terrain = unsafe { &*(*ctx.registers[21].x.as_ref() as *const TerrainData) };
    let unit = unsafe { &*(*ctx.registers[23].x.as_ref() as *const Unit) };
    unsafe { *ctx.registers[0].w.as_mut() = unit.is_terrain_valid(terrain) as u32 };
}

#[skyline::hook(offset = 0x01E7A99C, inline)]
fn battle_set_terrain_2(ctx: &mut InlineCtx) {
    let terrain = unsafe { &*(*ctx.registers[21].x.as_ref() as *const TerrainData) };
    let unit = unsafe { &*(*ctx.registers[19].x.as_ref() as *const Unit) };
    unsafe { *ctx.registers[0].w.as_mut() = unit.is_terrain_valid(terrain) as u32 };
}

#[skyline::hook(offset = 0x02470958, inline)]
fn battle_cal_interrupt(ctx: &mut InlineCtx) {
    let terrain = unsafe { &*(*ctx.registers[20].x.as_ref() as *const TerrainData) };
    let unit = unsafe { &*(*ctx.registers[21].x.as_ref() as *const Unit) };
    unsafe { *ctx.registers[0].w.as_mut() = unit.is_terrain_valid(terrain) as u32 };
}

pub fn install() {
    skyline::install_hooks!(
        is_terrain_invalid,
        phase_start_damage,
        phase_start_heal,
        map_terrain_single,
        battle_set_terrain_1,
        battle_set_terrain_2,
        battle_cal_interrupt,
    );
}
