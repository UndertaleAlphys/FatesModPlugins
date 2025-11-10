use crate::history::History;
use crate::skill::SkillArrayTrait;
use crate::{
    class::ClassTrait,
    item::{kind, use_type, ItemTrait},
    skill::{
        bad_states::{self},
        flag, SkillTrait,
    },
    terrain::TerrainTrait,
    util::bitmask::BitMask,
};
use engage::gamedata::{item::ItemData, skill::SkillData, unit::Unit, Gamedata, WeaponMask};
use engage::map::image::MapImage;
use engage::unitpool::UnitPool;
use engage::util::get_instance;
use unity::prelude::*;

pub mod capability;
pub mod status;
pub mod terrain;

const MALE_GENDER: i32 = 1;
const FEMALE_GENDER: i32 = 2;
pub trait UnitTrait {
    fn add_skill(&self, skill: &SkillData);
    fn add_sid(&self, sid: impl AsRef<str>);
    fn remove_private_sid(&self, sid: impl AsRef<str>);
    fn has_whole_sid(&self, sid: impl AsRef<str>) -> bool;
    fn get_variable(&self, variable_name: impl AsRef<str>) -> i32;
    fn set_variable(&self, variable_name: impl AsRef<str>, value: i32);
    fn get_debuff(&self, debuff_type: impl AsRef<str>) -> i32;
    fn set_debuff(&self, debuff_type: impl AsRef<str>, debuff: i32);
    fn get_engage_meter(&self) -> i32;
    fn set_engage_meter(&self, meter: i32);
    fn get_engage_meter_limit(&self) -> i32;
    fn is_engage_meter_full(&self) -> bool;
    fn get_hp(&self) -> i32;
    fn get_maxhp(&self) -> i32;
    fn is_fly(&self) -> bool;
    fn has_winged_shield(&self) -> bool;
    fn get_center_cell(&self) -> Option<(i32, i32)>;
    fn can_be_moved(&self) -> bool;
    fn can_revive(&self) -> bool;
    fn auto_equip_item(&self);
    fn calc_item_range(&self, item: &ItemData, command: Option<&SkillData>) -> (i32, i32);
    fn is_enemy(&self) -> bool;
    fn get_weapon_level(&self, kind: i32, enhanced: bool) -> i32;
    fn set_engage_turn(&self, engage_turn: i32);
    fn get_engage_turn_limit(&self) -> i32;
    fn get_engage_link_unit(&self) -> Option<&Unit>;
    fn is_on_map(&self) -> bool;
    fn is_in_play_area(&self) -> bool;
    fn iter_range(&self, range: i32) -> Vec<(i32, i32)>;
}

impl UnitTrait for Unit {
    fn add_skill(&self, skill: &SkillData) {
        unsafe { unit_add_skill(self, skill, None) }
    }
    fn add_sid(&self, sid: impl AsRef<str>) {
        let skill = SkillData::get(sid.as_ref());
        if let Some(skill) = skill {
            self.add_skill(skill);
        }
    }
    fn remove_private_sid(&self, sid: impl AsRef<str>) {
        unsafe { unit_remove_private_sid(self, sid.into(), None) }
    }

    fn has_whole_sid(&self, sid: impl AsRef<str>) -> bool {
        self.has_sid(sid.as_ref().into())
            || self.private_skill.contains_sid(sid.as_ref())
            || self.receive_skill.contains_sid(sid.as_ref())
            || self.supported_skill.contains_sid(sid.as_ref())
    }

    fn get_variable(&self, variable_name: impl AsRef<str>) -> i32 {
        let prefix = format!("SID_{}_", variable_name.as_ref());
        let mut result = 0;
        for skill in self.private_skill.iter() {
            let value = skill
                .get_skill()
                .map(|s| s.sid.to_string())
                .map(|s| s.strip_prefix(prefix.as_str()).map(|s| s.to_string()))
                .unwrap_or(None)
                .map(|s| s.parse::<i32>().ok())
                .unwrap_or(None)
                .map(|v| match v >= 0 && v < 7 {
                    true => Some(v),
                    false => None,
                })
                .unwrap_or(None);
            if let Some(value) = value {
                result = result.bit_add(1 << value);
            }
        }
        result
    }

    fn set_variable(&self, variable_name: impl AsRef<str>, value: i32) {
        let prefix = format!("SID_{}_", variable_name.as_ref());
        let old_value = self.get_variable(variable_name.as_ref());
        let value = value.clamp(0, 127);
        if old_value != value {
            History::private_skill(self);
            for bit in 0..7 {
                let mask = 1 << bit;
                if old_value.contains(mask) && !value.contains(mask) {
                    let variable_sid = format!("{}{}", prefix, bit);
                    self.remove_private_sid(variable_sid);
                } else if !old_value.contains(mask) && value.contains(mask) {
                    let variable_sid = format!("{}{}", prefix, bit);
                    self.add_sid(variable_sid);
                }
            }
        }
    }

    fn get_debuff(&self, debuff_type: impl AsRef<str>) -> i32 {
        let name = format!("Debuff{}", debuff_type.as_ref());
        self.get_variable(name.as_str())
    }
    fn set_debuff(&self, debuff_type: impl AsRef<str>, debuff: i32) {
        let name = format!("Debuff{}", debuff_type.as_ref());
        self.set_variable(name.as_str(), debuff);
    }
    fn get_engage_meter(&self) -> i32 {
        unsafe { unit_get_engage_count(self, None) }
    }
    fn set_engage_meter(&self, meter: i32) {
        let meter = meter.clamp(0, self.get_engage_meter_limit());
        unsafe {
            unit_set_engage_count(self, meter, None);
        }
    }
    fn get_engage_meter_limit(&self) -> i32 {
        unsafe { unit_get_engage_count_limit(self, None) }
    }
    fn is_engage_meter_full(&self) -> bool {
        self.get_engage_meter() >= self.get_engage_meter_limit()
    }
    fn get_hp(&self) -> i32 {
        unsafe { unit_get_hp(self, None) }
    }
    fn get_maxhp(&self) -> i32 {
        unsafe { unit_get_max_hp(self, None) }
    }
    fn is_fly(&self) -> bool {
        if self.job.is_fly() {
            true
        } else {
            if let Some(mask) = self.mask_skill {
                mask.flags.contains(flag::MOVE_FLY)
            } else {
                false
            }
        }
    }
    fn has_winged_shield(&self) -> bool {
        self.has_sid(Il2CppString::new("SID_翼盾"))
    }
    fn get_center_cell(&self) -> Option<(i32, i32)> {
        let map_size = self.get_person().get_bmap_size();
        if map_size % 2 == 0 {
            None
        } else {
            let addition = (map_size - 1) / 2;
            let x = self.x + addition;
            let z = self.z + addition;
            Some((x as i32, z as i32))
        }
    }
    fn can_be_moved(&self) -> bool {
        unsafe { unit_can_external_move(self, None) }
    }
    fn can_revive(&self) -> bool {
        unsafe { unit_can_revive(self, None) }
    }
    fn auto_equip_item(&self) {
        unsafe { unit_item_equip(self, None) };
    }
    fn calc_item_range(&self, item: &ItemData, command: Option<&SkillData>) -> (i32, i32) {
        let mut i = item.range_i as i32;
        let mut o = item.range_o as i32;
        if item.kind == kind::TOOL {
            i = 0;
        }
        if let Some(mask) = self.mask_skill {
            if item.kind == kind::ROD
                && item.usetype == use_type::HEAL
                && mask.flags.contains(flag::SELF_HEAL)
            {
                i = 0
            }
            let should_calc_cannon = mask.contains_sid("SID_CannonSkill");
            let mut extra = 0;
            for skl in mask.iter() {
                if let Some(skl) = skl.get_skill() {
                    let range_target = skl.get_range_target();
                    let is_type_matched = range_target == 9 || item.kind == range_target as u32;
                    if should_calc_cannon && skl.is_cannon_skill() {
                        i = if is_type_matched { skl.range_i } else { 0 };
                        o = if is_type_matched { skl.range_o } else { 0 };
                        extra = 0;
                        break;
                    } else if is_type_matched {
                        let si = skl.get_range_i();
                        let so = skl.get_range_o();
                        if si != 0 {
                            i = i.min(skl.get_range_i());
                        }
                        if so != 0 {
                            o = o.max(skl.get_range_o());
                        }
                        extra += skl.get_range_add();
                    }
                }
            }
            o += extra;
            if let Some(command) = command {
                let mut ci = command.get_range_i();
                let mut co = command.get_range_o();
                if ci == 0 {
                    ci = i;
                }
                if co == 0 {
                    co = o;
                }
                i = i.max(ci);
                o = o.min(co);
            }
            if (mask.bad_states.contains(bad_states::SILENCE) && item.is_silence_target())
                || mask.bad_states.contains(bad_states::SLEEP)
            {
                i = 0;
                o = 0;
            }
        }
        i = i.clamp(0, 255);
        o = o.clamp(0, 255);
        if i <= o {
            (i, o)
        } else {
            (0, 0)
        }
    }
    fn is_enemy(&self) -> bool {
        self.force.map_or(0, |f| f.force_type) == 1
    }
    fn get_weapon_level(&self, kind: i32, enhanced: bool) -> i32 {
        let required_weapon_mask = 1 << kind;
        let class_contains_weapon = WeaponMask::instantiate()
            .map_or(0, |m| {
                self.job
                    .get_weapon_mask_with_selected(&m, self.selected_weapon_mask)
                    .value
            })
            .contains(required_weapon_mask);
        let class_weapon_level = if class_contains_weapon {
            self.job
                .get_max_weapon_level_with_aptitude(kind, self.aptitude)
        } else {
            0
        };
        if enhanced {
            let skill_weapon_level = self.mask_skill.map_or(0, |m| m.get_weapon_level(kind));
            class_weapon_level.max(skill_weapon_level)
        } else {
            class_weapon_level
        }
    }

    fn set_engage_turn(&self, engage_turn: i32) {
        let engage_turn = engage_turn.clamp(0, self.get_engage_turn_limit()) as u8;
        if self.is_engaging() {
            if let Some(mut_self) = UnitPool::get_by_index(self.index as i32) {
                History::engage_turn(self);
                mut_self.engage_turn = engage_turn;
            }
            if let Some(link) = self.get_engage_link_unit() {
                if let Some(mut_link) = UnitPool::get_by_index(link.index as i32) {
                    History::engage_turn(link);
                    mut_link.engage_turn = engage_turn;
                }
            }
        }
    }

    fn get_engage_turn_limit(&self) -> i32 {
        unsafe { unit_get_engage_turn_limit(self, None) }
    }

    fn get_engage_link_unit(&self) -> Option<&Unit> {
        unsafe { unit_get_engage_link_unit(self, None) }
    }

    fn is_on_map(&self) -> bool {
        unsafe { unit_is_on_map(self, None) }
    }

    fn is_in_play_area(&self) -> bool {
        unsafe { unit_is_in_play_area(self, None) }
    }
    fn iter_range(&self, range: i32) -> Vec<(i32, i32)> {
        let mut result = Vec::new();
        let image: &MapImage = get_instance::<MapImage>();
        let min_x = image.playarea_x1.min(image.playarea_x2);
        let max_x = image.playarea_x1.max(image.playarea_x2);
        let min_z = image.playarea_z1.min(image.playarea_z2);
        let max_z = image.playarea_z1.max(image.playarea_z2);
        let in_box = |(x, z)| min_x <= x && x <= max_x && min_z <= z && z <= max_z;
        if range > 0 && self.is_on_map() && self.is_in_play_area() {
            let center = (self.get_x(), self.get_z());
            let (center_x, center_z) = center;
            let start_x = center_x;
            let start_z = center_z - range;
            for i in 0..range + 1 {
                for j in 0..range + 1 {
                    let point = (start_x - i + j, start_z + i + j);
                    if point != center && in_box(point) {
                        result.push(point);
                    }
                }
            }

            let start_x = center_x;
            let start_z = center_z - range + 1;
            for i in 0..range {
                for j in 0..range {
                    let point = (start_x - i + j, start_z + i + j);
                    if point != center && in_box(point) {
                        result.push(point);
                    }
                }
            }
        }
        result
    }
}

#[skyline::from_offset(0x01A5D430)]
fn unit_add_skill(unit: &Unit, skill_data: &SkillData, method: OptionalMethod);

#[skyline::from_offset(0x01A38090)]
fn unit_remove_private_sid(unit: &Unit, sid: &Il2CppString, method: OptionalMethod);

// #[unity::from_offset("App", "Unit", "set_EngageCount")]
#[skyline::from_offset(0x01A264A0)]
fn unit_set_engage_count(this: &Unit, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "get_EngageCount")]
fn unit_get_engage_count(this: &Unit, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Unit", "GetEngageCountLimit")]
fn unit_get_engage_count_limit(this: &Unit, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Unit", "GetMaxHp")]
fn unit_get_max_hp(this: &Unit, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Unit", "get_Hp")]
fn unit_get_hp(this: &Unit, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x01A29F10)]
fn unit_can_external_move(unit: &Unit, method: OptionalMethod) -> bool;

#[skyline::from_offset(0x01A4F860)]
fn unit_can_revive(unit: &Unit, method: OptionalMethod) -> bool;

#[skyline::from_offset(0x01A21530)]
fn unit_item_equip(this: &Unit, method: OptionalMethod);

#[skyline::from_offset(0x01A39D00)]
fn unit_is_on_map(unit: &Unit, method: OptionalMethod) -> bool;

#[skyline::from_offset(0x01A23CF0)]
fn unit_is_in_play_area(unit: &Unit, method: OptionalMethod) -> bool;

#[skyline::hook(offset = 0x01A4F850)]
fn unit_set_engage_turn(unit: &Unit, value: i32, _method: OptionalMethod) {
    unit.set_engage_turn(value);
}

#[skyline::from_offset(0x01A57D60)]
fn unit_get_engage_turn_limit(unit: &Unit, method: OptionalMethod) -> i32;

#[skyline::from_offset(0x01A274D0)]
fn unit_get_engage_link_unit(unit: &Unit, method: OptionalMethod) -> Option<&Unit>;

#[skyline::hook(offset = 0x01A46500)]
fn unit_calc_range(
    unit: &Unit,
    item: &ItemData,
    range_i: &mut i32,
    range_o: &mut i32,
    command_skill: Option<&SkillData>,
    _method: OptionalMethod,
) -> bool {
    let (i, o) = unit.calc_item_range(item, command_skill);
    *range_i = i;
    *range_o = o;
    i <= o
}

pub fn install() {
    capability::install();
    skyline::install_hooks!(unit_calc_range, unit_set_engage_turn);
}
