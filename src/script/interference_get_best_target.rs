use crate::script::util::DynValueTrait;
use crate::unit::force::ForceIterator;
use crate::unit::image::UnitImageGet;
use crate::unit::{capability, force, UnitTrait};
use engage::gamedata::unit::Unit;
use engage::script::{DynValue, ScriptUtils};
use unity::prelude::{Il2CppArray, OptionalMethod};

pub extern "C" fn enfeeble(
    args: &Il2CppArray<&DynValue>,
    _method: OptionalMethod,
) -> &'static DynValue {
    let nil = DynValue::new_nil();
    let Some(unit) = args.try_get_unit(0) else {
        return nil;
    };
    let Some(force) = unit.force else {
        return nil;
    };
    let force_type = force.force_type;
    let foe_force_type = match force_type {
        force::ENEMY => force::PLAYER,
        _ => force::ENEMY,
    };
    let mut best_candidate: Option<&Unit> = None;
    let mut best_score = 0f64;
    for foe_unit in ForceIterator::new(foe_force_type) {
        let x = foe_unit.get_x();
        let z = foe_unit.get_z();
        if !unit.is_in_image(x, z, UnitImageGet::interference_image) {
            continue;
        }
        let mut in_range_count = 0;
        for ally_unit in ForceIterator::new(force_type) {
            if ally_unit.index != unit.index
                && ally_unit.is_in_image(x, z, UnitImageGet::attack_image)
            {
                in_range_count += 1;
            }
        }
        const DEBUFF_VALUES: &[(&str, i32)] = &[
            ("Str", 2),
            ("Mag", 2),
            ("Dex", 1),
            ("Spd", 2),
            ("Lck", 1),
            ("Def", 2),
            ("Res", 2),
        ];
        let mut debuff_add = 0;
        for (debuff_type, debuff_value) in DEBUFF_VALUES.iter() {
            debuff_add += (4 - foe_unit.get_debuff(debuff_type)).max(0) * debuff_value;
        }
        let debuff_score = debuff_add as f64;
        let in_range_score = (1 + in_range_count) as f64;
        let hp_score = foe_unit.get_capability(capability::MAXHP, true) as f64
            / foe_unit.get_hp().clamp(1, 3) as f64;
        let score = debuff_score * in_range_score * hp_score;
        if score > best_score {
            best_score = score;
            best_candidate = Some(foe_unit);
        }
    }
    if let Some(candidate) = best_candidate {
        DynValue::new_number(candidate.index as f64)
    } else {
        nil
    }
}

pub extern "C" fn freeze(
    args: &Il2CppArray<&DynValue>,
    _method: OptionalMethod,
) -> &'static DynValue {
    let nil = DynValue::new_nil();
    let Some(unit) = args.try_get_unit(0) else {
        return nil;
    };
    let Some(force) = unit.force else {
        return nil;
    };
    let force_type = force.force_type;
    let foe_force_type = match force_type {
        force::ENEMY => force::PLAYER,
        _ => force::ENEMY,
    };
    let mut best_candidate: Option<&Unit> = None;
    let mut best_score = 0f64;
    for foe_unit in ForceIterator::new(foe_force_type) {
        let x = foe_unit.get_x();
        let z = foe_unit.get_z();
        if !unit.is_in_image(x, z, UnitImageGet::interference_image) {
            continue;
        }
        let mut in_range_count = 0;
        for ally_unit in ForceIterator::new(force_type) {
            if ally_unit.index != unit.index
                && ally_unit.is_in_image(x, z, UnitImageGet::attack_image)
            {
                in_range_count += 1;
            }
        }
        let in_range_score = (1 + in_range_count) as f64;
        let hp_score = foe_unit.get_capability(capability::MAXHP, true) as f64
            / foe_unit.get_hp().max(1) as f64;
        let score = in_range_score * hp_score;
        if score > best_score {
            best_score = score;
            best_candidate = Some(foe_unit);
        }
    }
    if let Some(candidate) = best_candidate {
        DynValue::new_number(candidate.index as f64)
    } else {
        nil
    }
}
