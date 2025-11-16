use crate::class::move_type;
use crate::history::History;
use crate::map::{Map, MapMindTrait};
use crate::unit::skill::UnitSkillTrait;
use crate::unit::UnitTrait;
use engage::gamedata::skill::SkillData;
use engage::map::image::MapImage;
use engage::mapmind::MapMind;
use engage::script::EventResultScriptCommand;
use engage::util::get_instance;
use engage::{
    gamedata::{item::ItemData, unit::Unit, Gamedata},
    script::{DynValue, EventScript, EventScriptCommand, ScriptUtils},
};
use skyline::hooks::InlineCtx;
use unity::prelude::*;

#[repr(C)]
pub struct Vector3 {
    pub fields: Vector3Fields,
}

#[repr(C)]
pub struct Vector3Fields {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct Quaternion {
    pub fields: QuaternionFields,
}

#[repr(C)]
pub struct QuaternionFields {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[skyline::from_offset(0x021A3970)]
pub fn dyn_value_array_get_by_idx<'a>(
    array: &Il2CppArray<&'a DynValue>,
    idx: i32,
    method: OptionalMethod,
) -> Option<&'a DynValue>;

#[skyline::from_offset(0x02E3CD60)]
pub fn dyn_value_get_type(dv: &DynValue, method: OptionalMethod) -> i32;

#[skyline::from_offset(0x022A2260)]
pub fn transporter_delete(idx: i32, method: OptionalMethod);

#[skyline::from_offset(0x022A2570)]
pub fn transporter_delete_item(data: &ItemData, method: OptionalMethod);

#[skyline::from_offset(0x01DBD3E0)]
pub fn app_map_effect_create(
    name: Option<&'static Il2CppString>,
    position: Vector3,
    rotation: Quaternion,
    method: OptionalMethod,
);

#[skyline::from_offset(0x01A57C40)]
pub fn get_engage_turn_limit(unit: &Unit, method: OptionalMethod) -> &'static DynValue;

#[skyline::from_offset(0x02AF9650)]
pub fn set_chapter_field(
    chapter_data_ptr: i64,
    name: Option<&'static Il2CppString>,
    method: OptionalMethod,
);

/// This is called a proc(edural) macro. You use this to indicate that a function will be used as a hook.
///
/// Pay attention to the argument, offset.
/// This is the address of the start of the function you would like to hook.
/// This address has to be relative to the .text section of the game.
/// If you do not know what any of this means, take the address in Ghidra and remove the starting ``71`` and the zeroes that follow it.
/// Do not forget the 0x indicator, as it denotates that you are providing a hexadecimal value.

#[skyline::hook(offset = 0x024E279C, inline)]
pub fn scripts_regist(ctx: &InlineCtx) {
    let event = unsafe { &*((*ctx.registers[20].x.as_ref()) as *const EventScript) };
    ScriptIF::register(event);
}

pub struct ScriptIF;

impl ScriptIF {
    pub fn register(event: &EventScript) {
        event.register_action("RemoveTransporterItem", remove_transporter_item);
        event.register_action("EffectCreateRotateXYZ", create_map_effect_rotate_xyz);
        event.register_function("UnitGetEngageTurn", get_engage_turn);
        event.register_action("UnitSetEngageTurn", set_engage_turn);
        event.register_function("UnitGetStatus", get_unit_status);
        event.register_action("UnitSetStatusImpl", set_unit_status);
        event.register_function("UnitGetBondLevel", get_god_unit_bond_level);
        event.register_action("UnitSetBondLevel", set_god_unit_bond_level);
        event.register_function("UnitGetDebuff", unit_get_debuff);
        event.register_action("UnitSetDebuff", unit_set_debuff);
        // event.register_action("UnitAddPrivateSkill", unit_add_private_skill);
        // event.register_action("UnitRemovePrivateSkill", unit_remove_private_skill);
        event.register_action("M023SkySeal", m023_sky_seal);
        event.register_action("M023SkyBless", m023_sky_bless);
        event.register_function("UnitGetEngageMeter", unit_get_engage_meter);
        event.register_action("UnitSetEngageMeter", unit_set_engage_meter);
        event.register_function("IsInUnitAttackImage", unit_image::is_in_attack);
        event.register_function("IsInUnitInterferenceImage", unit_image::is_in_interference);
        event.register_function("IsInUnitHealImage", unit_image::is_in_heal);
        event.register_function("IsInUnitMoveImage", unit_image::is_in_move);
        event.register_action("UpdateUnit", unit_update);
        event.register_action("UnitSetEngageMeterM017", unit_set_engage_meter_m017);
    }
}

fn quaternion_multiply(q1: Quaternion, q2: Quaternion) -> Quaternion {
    Quaternion {
        fields: QuaternionFields {
            x: q1.fields.w * q2.fields.x + q1.fields.x * q2.fields.w + q1.fields.y * q2.fields.z
                - q1.fields.z * q2.fields.y,
            y: q1.fields.w * q2.fields.y - q1.fields.x * q2.fields.z
                + q1.fields.y * q2.fields.w
                + q1.fields.z * q2.fields.x,
            z: q1.fields.w * q2.fields.z + q1.fields.x * q2.fields.y - q1.fields.y * q2.fields.x
                + q1.fields.z * q2.fields.w,
            w: q1.fields.w * q2.fields.w
                - q1.fields.x * q2.fields.x
                - q1.fields.y * q2.fields.y
                - q1.fields.z * q2.fields.z,
        },
    }
}

extern "C" fn remove_transporter_item(args: &Il2CppArray<&DynValue>, _method: OptionalMethod) {
    for arg_idx in 0..args.len() {
        const NONE: i32 = i32::MAX;
        const NUMBER: i32 = 3;
        const STRING: i32 = 4;
        match {
            if let Some(dv) = unsafe { dyn_value_array_get_by_idx(args, arg_idx as i32, None) } {
                unsafe { dyn_value_get_type(dv, None) }
            } else {
                NONE
            }
        } {
            NUMBER => {
                let iidx = args.try_get_i32(arg_idx as i32);
                unsafe { transporter_delete(iidx, None) };
            }
            STRING => {
                if let Some(iid) = args.try_get_string(arg_idx as i32) {
                    let idata = ItemData::get(iid);
                    if let Some(idata) = idata {
                        unsafe { transporter_delete_item(idata, None) };
                    }
                }
            }
            _ => {}
        }
    }
}

extern "C" fn create_map_effect_rotate_xyz(args: &Il2CppArray<&DynValue>, _method: OptionalMethod) {
    let effect_name = args.try_get_string(0);
    let position_x = (args.try_get_i32(1) as f32 * 5.0) + 2.5;
    let position_y = args.try_get_i32(3);
    let position_z = (args.try_get_i32(2) as f32 * 5.0) + 2.5;
    let rotation_x_angle = args.try_get_i32(4);
    let rotation_y_angle = args.try_get_i32(5);
    let rotation_z_angle = args.try_get_i32(6);
    let order = args.try_get_i32(7);

    const XYZ: i32 = 0;
    const XZY: i32 = 1;
    const YXZ: i32 = 2;
    const YZX: i32 = 3;
    const ZXY: i32 = 4;
    const ZYX: i32 = 5;

    let position = Vector3 {
        fields: Vector3Fields {
            x: position_x as f32,
            y: position_y as f32,
            z: position_z as f32,
        },
    };

    let x_angle_rad = (rotation_x_angle as f32).to_radians();
    let x_half_angle = x_angle_rad / 2.0;
    let x_sin_half = x_half_angle.sin();
    let x_cos_half = x_half_angle.cos();

    let rotation_x = Quaternion {
        fields: QuaternionFields {
            x: x_sin_half,
            y: 0.0,
            z: 0.0,
            w: x_cos_half,
        },
    };

    let y_angle_rad = (rotation_y_angle as f32).to_radians();
    let y_half_angle = -y_angle_rad / 2.0;
    let y_sin_half = y_half_angle.sin();
    let y_cos_half = y_half_angle.cos();

    let rotation_y = Quaternion {
        fields: QuaternionFields {
            x: 0.0,
            y: y_sin_half,
            z: 0.0,
            w: y_cos_half,
        },
    };

    let z_angle_rad = (rotation_z_angle as f32).to_radians();
    let z_half_angle = z_angle_rad / 2.0;
    let z_sin_half = z_half_angle.sin();
    let z_cos_half = z_half_angle.cos();

    let rotation_z = Quaternion {
        fields: QuaternionFields {
            x: 0.0,
            y: 0.0,
            z: z_sin_half,
            w: z_cos_half,
        },
    };

    let rotation = match order {
        XYZ => {
            let rot_xy = quaternion_multiply(rotation_y, rotation_x);
            quaternion_multiply(rotation_z, rot_xy)
        }
        XZY => {
            let rot_xz = quaternion_multiply(rotation_z, rotation_x);
            quaternion_multiply(rotation_y, rot_xz)
        }
        YXZ => {
            let rot_yx = quaternion_multiply(rotation_x, rotation_y);
            quaternion_multiply(rotation_z, rot_yx)
        }
        YZX => {
            let rot_yz = quaternion_multiply(rotation_z, rotation_y);
            quaternion_multiply(rotation_x, rot_yz)
        }
        ZXY => {
            let rot_zx = quaternion_multiply(rotation_x, rotation_z);
            quaternion_multiply(rotation_y, rot_zx)
        }
        ZYX => {
            let rot_zy = quaternion_multiply(rotation_y, rotation_z);
            quaternion_multiply(rotation_x, rot_zy)
        }
        _ => {
            let rot_xy = quaternion_multiply(rotation_y, rotation_x);
            quaternion_multiply(rotation_z, rot_xy)
        }
    };

    match effect_name {
        Some(name) => unsafe {
            match std::panic::catch_unwind(|| {
                app_map_effect_create(Some(name), position, rotation, _method);
            }) {
                Ok(_) => println!("Successfully created effect: {}", name.to_string()),
                Err(e) => {
                    println!("Error occurred while creating effect: {}", name.to_string());
                    println!("Panic info: {:?}", e);
                }
            }
        },
        None => {}
    }
}

extern "C" fn get_engage_turn(
    args: &Il2CppArray<&DynValue>,
    _method: OptionalMethod,
) -> &'static DynValue {
    let unit: &Unit = args.try_get_unit(0).unwrap();
    DynValue::new_number(unit.fields.engage_turn as f64)
}

extern "C" fn set_engage_turn(args: &Il2CppArray<&DynValue>, _method: OptionalMethod) {
    let unit: &Unit = args.try_get_unit(0).unwrap();
    let turn = args.try_get_i32(1);
    unit.set_engage_turn(turn);
}

extern "C" fn get_unit_status(
    args: &Il2CppArray<&DynValue>,
    _method: OptionalMethod,
) -> &'static DynValue {
    let unit: &Unit = args.try_get_unit(0).unwrap();
    DynValue::new_number(unit.fields.status.fields.value as f64)
}

extern "C" fn set_unit_status(args: &Il2CppArray<&DynValue>, _method: OptionalMethod) {
    let unit: &Unit = args.try_get_unit(0).unwrap();
    let status = args.try_get_i32(1) as u64;

    unsafe {
        let unit_ptr = unit as *const Unit as *mut Unit;
        (*unit_ptr).fields.status.fields.value = status;
    }
}

extern "C" fn set_god_unit_bond_level(args: &Il2CppArray<&DynValue>, _method: OptionalMethod) {
    let unit: &Unit = args.try_get_unit(0).unwrap();
    unit.get_god_unit()
        .unwrap()
        .set_god_level(unit, args.try_get_i32(1) as i32)
}

extern "C" fn get_god_unit_bond_level(
    args: &Il2CppArray<&DynValue>,
    _method: OptionalMethod,
) -> &'static DynValue {
    let unit: &Unit = args.try_get_unit(0).unwrap();
    DynValue::new_number(
        unit.get_god_unit()
            .unwrap()
            .get_bond(unit)
            .unwrap()
            .fields
            .level as f64,
    )
}

extern "C" fn unit_get_debuff(
    args: &Il2CppArray<&DynValue>,
    _method: OptionalMethod,
) -> &'static DynValue {
    let unit = args.try_get_unit(0);
    let debuff_type = args.try_get_string(1);
    let value = if unit.is_some() && debuff_type.is_some() {
        unit.unwrap().get_debuff(debuff_type.unwrap().to_string())
    } else {
        0
    };
    DynValue::new_number(value as f64)
}

extern "C" fn unit_set_debuff(args: &Il2CppArray<&DynValue>, _method: OptionalMethod) {
    let unit = args.try_get_unit(0);
    let debuff_type = args.try_get_string(1);
    let value = args.try_get_i32(2);
    if unit.is_some() && debuff_type.is_some() {
        unit.unwrap()
            .set_debuff(debuff_type.unwrap().to_string(), value);
    }
}

extern "C" fn unit_add_private_skill(args: &Il2CppArray<&DynValue>, _method: OptionalMethod) {
    let unit = args.try_get_unit(0);
    let skill = args.try_get_string(1);
    if unit.is_some() && skill.is_some() {
        let unit = unit.unwrap();
        let skill = skill.unwrap();
        if let Some(skill) = SkillData::get(skill) {
            unit.add_skill(skill);
        } else {
            println!("Skill not found");
        }
    } else {
        println!("Index Error");
    }
}

extern "C" fn unit_remove_private_skill(args: &Il2CppArray<&DynValue>, _method: OptionalMethod) {
    let unit = args.try_get_unit(0);
    let skill = args.try_get_string(1);
    if unit.is_some() && skill.is_some() {
        let unit = unit.unwrap();
        let skill = skill.unwrap();
        if SkillData::get(skill).is_some() {
            unit.remove_private_sid(skill.to_string());
        } else {
            println!("Skill not found");
        }
    } else {
        println!("Index Error");
    }
}

extern "C" fn m023_sky_seal(_args: &Il2CppArray<&DynValue>, _method: OptionalMethod) {
    m023_sky_effect(m023_decrease_mov, m023_increase_mov);
}

extern "C" fn m023_sky_bless(_args: &Il2CppArray<&DynValue>, _method: OptionalMethod) {
    m023_sky_effect(m023_increase_mov, m023_decrease_mov);
}

fn m023_decrease_mov(u: &Unit) {
    u.remove_private_sid("SID_移動半分に増加_効果_M023");
    u.add_sid("SID_移動半減_効果_M023");
}

fn m023_increase_mov(u: &Unit) {
    u.remove_private_sid("SID_移動半減_効果_M023");
    u.add_sid("SID_移動半分に増加_効果_M023");
}

fn m023_sky_effect<F, G>(fly_effect: F, ground_effect: G)
where
    F: Fn(&Unit),
    G: Fn(&Unit),
{
    let image: &MapImage = get_instance::<MapImage>();
    let ((x_min, z_min), (x_max, z_max)) = Map::get_play_area();
    for x in x_min..=x_max {
        for z in z_min..=z_max {
            if let Some(unit) = image.get_target_unit(x, z) {
                History::private_skill(unit);
                if unit.get_job().move_type == move_type::FLY {
                    fly_effect(unit);
                } else {
                    ground_effect(unit);
                }
            }
        }
    }
}

extern "C" fn unit_get_engage_meter(
    args: &Il2CppArray<&DynValue>,
    _method: OptionalMethod,
) -> &'static DynValue {
    let result = args.try_get_unit(0).map_or(0, |u| u.get_engage_meter());
    DynValue::new_number(result as f64)
}

extern "C" fn unit_set_engage_meter(args: &Il2CppArray<&DynValue>, _method: OptionalMethod) {
    if let Some(unit) = args.try_get_unit(0) {
        let meter = args.try_get_i32(1);
        History::engage_meter(unit);
        unit.set_engage_meter(meter);
    }
}

extern "C" fn unit_set_engage_meter_m017(args: &Il2CppArray<&DynValue>, _method: OptionalMethod) {
    if let Some(unit) = args.try_get_unit(0) {
        let meter = args.try_get_i32(1).clamp(0, unit.get_engage_meter_limit());
        let mind: &MapMind = get_instance();
        let mind_units = [mind.get_unit(), mind.get_target_unit()];
        for mind_unit in mind_units {
            if let Some(mind_unit) = mind_unit {
                if mind_unit.index == unit.index {
                    History::private_skill(mind_unit);
                    History::engage_meter(mind_unit);
                    mind_unit.set_engage_meter(0);
                    mind_unit.add_sid(format!("SID_M017_SetEngageMeter_{meter}"));
                    return;
                }
            }
        }
        History::engage_meter(unit);
        unit.set_engage_meter(meter);
    }
}

mod unit_image {
    use crate::unit::image::{MapImageCoreBit, UnitImageGet};
    use engage::gamedata::unit::Unit;
    use engage::script::{DynValue, ScriptUtils};
    use unity::prelude::{Il2CppArray, OptionalMethod};

    pub extern "C" fn is_in_attack(
        args: &Il2CppArray<&DynValue>,
        _method: OptionalMethod,
    ) -> &'static DynValue {
        let result = unit_is_in_image(args, UnitImageGet::attack_image);
        DynValue::new_boolean(result)
    }

    pub extern "C" fn is_in_interference(
        args: &Il2CppArray<&DynValue>,
        _method: OptionalMethod,
    ) -> &'static DynValue {
        let result = unit_is_in_image(args, UnitImageGet::interference_image);
        DynValue::new_boolean(result)
    }

    pub extern "C" fn is_in_heal(
        args: &Il2CppArray<&DynValue>,
        _method: OptionalMethod,
    ) -> &'static DynValue {
        let result = unit_is_in_image(args, UnitImageGet::heal_image);
        DynValue::new_boolean(result)
    }

    pub extern "C" fn is_in_move(
        args: &Il2CppArray<&DynValue>,
        _method: OptionalMethod,
    ) -> &'static DynValue {
        let result = unit_is_in_image(args, UnitImageGet::move_image);
        DynValue::new_boolean(result)
    }

    fn unit_is_in_image(
        args: &Il2CppArray<&DynValue>,
        image: fn(&Unit) -> Option<&'static MapImageCoreBit>,
    ) -> bool {
        if args.len() < 3 {
            return false;
        }
        let Some(unit) = args.try_get_unit(0) else {
            return false;
        };
        let x = args.try_get_i32(1);
        let z = args.try_get_i32(2);
        unit.is_in_image(x, z, image)
    }
}

extern "C" fn unit_update(args: &Il2CppArray<&DynValue>, _method: OptionalMethod) {
    if let Some(unit) = args.try_get_unit(0) {
        unit.update();
    }
}

pub fn install() {
    skyline::install_hook!(scripts_regist);
}
