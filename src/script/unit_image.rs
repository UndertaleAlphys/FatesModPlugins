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
