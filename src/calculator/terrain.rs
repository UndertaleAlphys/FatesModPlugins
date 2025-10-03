use crate::calculator::command;
use crate::calculator::util::{CalculatorCommandTrait, CalculatorManagerTrait};
use crate::combat::battle_info::BattleInfoSideTrait;
use crate::unit::terrain::UnitTerrainTrait;
use engage::{
    battle::BattleInfoSide,
    calculator::{CalculatorManager, GameCalculatorCommand},
    gamedata::unit::Unit,
};
use unity::{prelude::OptionalMethod, system::Il2CppString};

pub fn add(manager: &mut CalculatorManager) {
    if let Some(terrain_avo) = manager.find_checked(command::TERRAIN_AVO) {
        terrain_avo.assign_virtual_method("GetImpl", get_terrain_avo_command_unit as _);
    }
    if let Some(terrain_def) = manager.find_checked(command::TERRAIN_DEF) {
        terrain_def.assign_virtual_method("GetImpl", get_terrain_def_command_unit as _);
    }
    if let Some(terrain_heal) = manager.clone_from_name(command::TERRAIN_AVO) {
        terrain_heal.assign_virtual_method("get_Name", get_terrain_heal_command_name as _);
        terrain_heal.assign_virtual_method("GetImpl", get_terrain_heal_command_unit as _);
        terrain_heal.assign_vtable(31, get_terrain_heal_command_battle_info as _);
        manager.add_command(terrain_heal);
        if let Some(foe_terrain_heal) = terrain_heal.clone() {
            manager.add_command(foe_terrain_heal.reverse());
        }
    }
    if let Some(terrain_mov) = manager.clone_from_name(command::TERRAIN_AVO) {
        terrain_mov.assign_virtual_method("get_Name", get_terrain_mov_command_name as _);
        terrain_mov.assign_virtual_method("GetImpl", get_terrain_mov_command_unit as _);
        terrain_mov.assign_vtable(31, get_terrain_mov_command_battle_info as _);
        manager.add_command(terrain_mov);
        if let Some(foe_terrain_mov) = terrain_mov.clone() {
            manager.add_command(foe_terrain_mov.reverse());
        }
    }
    if let Some(terrain_immune_break) = manager.clone_from_name(command::TERRAIN_AVO) {
        terrain_immune_break
            .assign_virtual_method("get_Name", get_terrain_immune_break_command_name as _);
        terrain_immune_break
            .assign_virtual_method("GetImpl", get_terrain_immune_break_command_unit as _);
        terrain_immune_break.assign_vtable(31, get_terrain_immune_break_command_battle_info as _);
        manager.add_command(terrain_immune_break);
        if let Some(foe_terrain_immune_break) = terrain_immune_break.clone() {
            manager.add_command(foe_terrain_immune_break.reverse());
        }
    }
}

fn get_terrain_heal_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::TERRAIN_HEAL.into()
}
fn get_terrain_mov_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::TERRAIN_MOV.into()
}

fn get_terrain_immune_break_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::TERRAIN_IMMUNE_BREAK.into()
}

fn get_terrain_avo_command_unit(
    _this: &GameCalculatorCommand,
    unit: &Unit,
    _method: OptionalMethod,
) -> f32 {
    let avo = unit.get_terrain_avo();
    avo as f32
}

fn get_terrain_def_command_unit(
    _this: &GameCalculatorCommand,
    unit: &Unit,
    _method: OptionalMethod,
) -> f32 {
    let def = unit.get_terrain_def();
    def as f32
}

fn get_terrain_heal_command_unit(
    _this: &GameCalculatorCommand,
    unit: &Unit,
    _method: OptionalMethod,
) -> f32 {
    let heal = unit.get_terrain_heal();
    heal as f32
}

fn get_terrain_mov_command_unit(
    _this: &GameCalculatorCommand,
    unit: &Unit,
    _method: OptionalMethod,
) -> f32 {
    let mov = unit.get_terrain_mov();
    mov as f32
}

fn get_terrain_immune_break_command_unit(
    _this: &GameCalculatorCommand,
    unit: &Unit,
    _method: OptionalMethod,
) -> f32 {
    let immune_break = unit.is_terrain_immune_to_break();
    if immune_break {
        1.0
    } else {
        0.0
    }
}

fn get_terrain_heal_command_battle_info(
    _this: &GameCalculatorCommand,
    side: &BattleInfoSide,
    _method: OptionalMethod,
) -> f32 {
    let unit = side.get_unit();
    if let Some(unit) = unit {
        get_terrain_heal_command_unit(_this, unit, None)
    } else {
        0f32
    }
}

fn get_terrain_mov_command_battle_info(
    _this: &GameCalculatorCommand,
    side: &BattleInfoSide,
    _method: OptionalMethod,
) -> f32 {
    let unit = side.get_unit();
    if let Some(unit) = unit {
        get_terrain_mov_command_unit(_this, unit, None)
    } else {
        0f32
    }
}

fn get_terrain_immune_break_command_battle_info(
    _this: &GameCalculatorCommand,
    side: &BattleInfoSide,
    _method: OptionalMethod,
) -> f32 {
    let unit = side.get_unit();
    if let Some(unit) = unit {
        get_terrain_immune_break_command_unit(_this, unit, None)
    } else {
        0f32
    }
}
