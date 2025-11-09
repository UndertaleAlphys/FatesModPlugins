mod inspector;

use crate::combat::cannon::inspector::CannonInspectorTrait;
use crate::map::MapImageTerrainTrait;
use crate::terrain::TerrainTrait;
use crate::unit::UnitTrait;
use engage::gamedata::skill::SkillData;
use engage::gamedata::terrain::TerrainData;
use engage::gamedata::unit::Unit;
use engage::map::image::MapImage;
use engage::map::inspectors::{CannonInspector, Inspector, MapInspector};
use engage::mapmind::MapMind;
use engage::sequence::mapsequencetargetselect::{MapTarget, MapTargetData};
use engage::tmpro::TextMeshProUGUI;
use engage::unitpool::UnitPool;
use engage::util::get_instance;
use skyline::hooks::InlineCtx;
use skyline::patching::Patch;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::OnceLock;
use unity::prelude::{Il2CppClassData, Il2CppString, MethodInfo, OptionalMethod};

pub fn install() {
    skyline::install_hooks!(
        cannon_inspector_decrease_shell,
        cannon_display,
        cannon_map_info_display,
        cannon_multi_target,
        cannons_other_than_fire_cannon_has_dont_create_terrain,
        map_target_enumerate,
        map_cannon_range_should_show,
        // map_panel_active_set_mod
    );
    // NOP
    Patch::in_text(0x01f32f04)
        .bytes([0x1F, 0x20, 0x03, 0xD5])
        .unwrap();
    Patch::in_text(0x01f32f08)
        .bytes([0x1F, 0x20, 0x03, 0xD5])
        .unwrap();
    Patch::in_text(0x01f32f10)
        .bytes([0x1F, 0x20, 0x03, 0xD5])
        .unwrap();
    Patch::in_text(0x02c2f818)
        .bytes([0x1F, 0x20, 0x03, 0xD5])
        .unwrap();
}

// Do not decrease shell
#[skyline::hook(offset = 0x025bb2a0)]
fn cannon_inspector_decrease_shell(_this: &CannonInspector, _method: OptionalMethod) {}

// Display as: INF
#[skyline::hook(offset = 0x01e446b0, inline)]
fn cannon_display(ctx: &mut InlineCtx) {
    let infinity: &'static Il2CppString = "∞".into();
    unsafe {
        *ctx.registers[0].x.as_mut() = infinity as *const _ as _;
    }
}

#[repr(C)]
#[unity::class("App", "MapInfoIconLocatorRoot")]
struct MapInfoIconLocatorRoot {
    _junk0: [u8; 0x18],
    value_text: &'static mut TextMeshProUGUI,
}

#[skyline::hook(offset = 0x02085570)]
fn cannon_map_info_display(
    this: &mut MapInfoIconLocatorRoot,
    inspector: &MapInspector,
    method: OptionalMethod,
) {
    call_original!(this, inspector, method);
    this.value_text.set_text("∞".into(), false);
}

#[skyline::hook(offset = 0x0267e358, inline)]
fn cannon_multi_target(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[0].x.as_mut() = true as _;
    }
}

#[skyline::hook(offset = 0x0267f044, inline)]
fn cannons_other_than_fire_cannon_has_dont_create_terrain(ctx: &mut InlineCtx) {
    let current_unit: &Unit = MapMind::get_unit();
    let (x, z) = (current_unit.get_x(), current_unit.get_z());
    let image_terrain = get_instance::<MapImage>().terrain;
    let terrain = image_terrain.get_terrain(x, z);
    let is_fire_cannon = terrain.map_or(false, |t| t.is_fire_cannon());
    let tid_nothing = Il2CppString::new("TID_無し");
    if !is_fire_cannon {
        unsafe {
            *ctx.registers[3].x.as_mut() = tid_nothing as *const _ as _;
        }
    }
}

#[skyline::from_offset(0x01e41c80)]
fn map_target_data_set(
    data: *mut MapTargetData,
    x: i32,
    z: i32,
    item_mask: u32,
    select_item_index: i32,
    method: OptionalMethod,
);

#[skyline::hook(offset = 0x01f32f14, inline)]
fn map_target_enumerate(ctx: &mut InlineCtx) {
    let this =
        unsafe { &mut *((*ctx.registers[19].x.as_ref()) as *const MapTarget as *mut MapTarget) };
    let inspector = unsafe { &*(*ctx.registers[20].x.as_ref() as *const CannonInspector) };
    let is_fire_cannon = inspector.get_terrain_data().is_fire_cannon();
    map_target_enumerate_cannon(this, is_fire_cannon);
}

fn map_target_enumerate_cannon(this: &mut MapTarget, is_fire_cannon: bool) -> Option<()> {
    this.m_dataset.as_mut()?.clear();
    let unit = this.unit?;
    if !unit.is_on_map() || !unit.is_in_play_area() {
        return None;
    };
    let mut ranges = HashMap::new();
    let mut max_o: Option<i32> = None;
    for list_index in 0..unit.item_list.unit_items.len() {
        let item = unit.item_list.get_item(list_index as i32);
        if let Some(item) = item.as_ref() {
            let (i, o) = unit.calc_item_range(item.item, None);
            assert!(i <= o);
            assert!(0 <= i);
            if o > 0 {
                ranges.insert(list_index, (i, o));
                if let Some(temp_o) = max_o {
                    max_o = Some(temp_o.max(o))
                } else {
                    max_o = Some(o);
                }
            }
        }
    }
    let max_o = max_o?;
    let image = get_instance::<MapImage>();
    let player_area_min_x = image.playarea_x1.min(image.playarea_x2);
    let player_area_max_x = image.playarea_x1.max(image.playarea_x2);
    let player_area_min_z = image.playarea_z1.min(image.playarea_z2);
    let player_area_max_z = image.playarea_z1.max(image.playarea_z2);
    let rect_min_x = player_area_min_x.max(unit.get_x() - max_o);
    let rect_max_x = player_area_max_x.min(unit.get_x() + max_o);
    let rect_min_z = player_area_min_z.max(unit.get_z() - max_o);
    let rect_max_z = player_area_max_z.min(unit.get_z() + max_o);
    assert!(rect_min_x <= rect_max_x);
    assert!(rect_min_z <= rect_max_z);
    let data_set = this.m_dataset.as_mut()?;
    for z in rect_min_z..=rect_max_z {
        for x in rect_min_x..=rect_max_x {
            let mut target_unit_count = 0;
            static AROUND: OnceLock<Vec<(i32, i32)>> = OnceLock::new();
            let around = AROUND.get_or_init(|| vec![(-1, 0), (0, 0), (1, 0), (0, -1), (0, 1)]);
            for (dx, dz) in around {
                let tx = dx + x;
                let tz = dz + z;
                if image.get_target_unit(tx, tz).is_some() {
                    target_unit_count += 1;
                }
            }
            if target_unit_count == 0 {
                continue;
            }
            let distance = (unit.get_x() - x).abs() + (unit.get_z() - z).abs();
            let mut item_mask: u32 = is_fire_cannon as _;
            for (index, i, o) in ranges.iter().map(|(idx, (i, o))| (*idx, *i, *o)) {
                if i <= distance && distance <= o {
                    item_mask |= 1 << index;
                }
            }
            if item_mask != 0 {
                data_set._item_mask |= item_mask as i32;
                let data = data_set.m_stack.pop()?;
                let data_ptr = data as *mut MapTargetData;
                unsafe { map_target_data_set(data_ptr, x, z, item_mask, -1, None) };
                let display_unit = image.get_target_unit(x, z);
                if let Some(display_unit) = display_unit {
                    if let Some(unit) = UnitPool::get_by_index(display_unit.index as i32) {
                        data.m_unit = unit;
                    }
                }
                data_set.m_list.add(data);
            }
        }
    }
    Some(())
}

#[skyline::hook(offset = 0x02c2f81c, inline)]
fn map_cannon_range_should_show(ctx: &mut InlineCtx) {
    let terrain = unsafe { &*(*ctx.registers[0].x.as_ref() as *const TerrainData) };
    let is_cannon =
        terrain.is_bow_cannon() || terrain.is_magic_cannon() || terrain.is_fire_cannon();
    unsafe {
        *ctx.registers[0].x.as_mut() = is_cannon as _;
    }
}

#[skyline::hook(offset = 0x01dfff90)]
fn map_panel_active_set_mod(
    map_panel_active: u64,
    mode: i32,
    is_force_update: bool,
    command: &SkillData,
    method: OptionalMethod,
) {
    println!("Mode={mode}");
    call_original!(map_panel_active, mode, is_force_update, command, method);
}
