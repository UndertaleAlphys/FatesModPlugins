use crate::util::bitmask::BitMask;
use engage::gamedata::unit::Unit;
use engage::gamedata::GodData;
use engage::util::get_instance;
use skyline::patching::Patch;
use unity::prelude::{Il2CppArray, OptionalMethod};
use unity::system::Il2CppString;

#[repr(C)]
#[unity::class("App", "UnitInfoParamManager")]
struct UnitInfoParamManager {
    junk0: [u8; 0x10],
    pub calc_unit: &'static Unit,
}

impl UnitInfoParamManager {
    pub fn get_param(&self, t: i32) -> &UnitInfoParamDetail {
        unsafe { unit_info_param_manager_get_param(self, t, None) }
    }
}

#[skyline::from_offset(0x01F93200)]
fn unit_info_param_manager_get_param(
    this: &UnitInfoParamManager,
    t: i32,
    method: OptionalMethod,
) -> &UnitInfoParamDetail;

#[repr(C)]
#[unity::class("App", "UnitInfoParamDetail")]
struct UnitInfoParamDetail {}

impl UnitInfoParamDetail {
    pub fn get_enhance_dir(&self, is_emblem_change: bool) -> i32 {
        unsafe { unit_info_param_detail_get_enhance_dir(self, is_emblem_change, None) }
    }
}

#[skyline::from_offset(0x01C53450)]
fn unit_info_param_detail_get_enhance_dir(
    this: &UnitInfoParamDetail,
    is_emblem_change: bool,
    method: OptionalMethod,
) -> i32;
#[repr(C)]
#[unity::class("App", "UnitInfoParamSetter")]
struct UnitInfoParamSetter {
    junk0: &'static i32,
    game_object: &'static GameObject,
    pub is_simple_ui: bool,
    frame_image: &'static UIImage,
    wdw_image: &'static UIImage,
    force_sprites: &'static Il2CppArray<&'static Sprite>,
    wdw_sprites: &'static Il2CppArray<&'static Sprite>,
    pub emblem_buff_sprites: &'static Il2CppArray<&'static Sprite>,
    junk1: [u8; 0xC0],
    pub emblem_buff_atk: &'static UIImage,
    pub emblem_battle_hit: &'static TextMeshProUGUI,
    pub emblem_buff_hit: &'static UIImage,
    //junk...
}

#[repr(C)]
#[unity::class("UnityEngine", "UI_Image")]
struct UIImage {}

#[repr(C)]
#[unity::class("UnityEngine", "GameObject")]
struct GameObject {}

#[repr(C)]
#[unity::class("UnityEngine", "Sprite")]
struct Sprite {}

#[skyline::hook(offset = 0x01F934B0)]
fn unit_info_param_setter_set_emblem_buff(
    this: &UnitInfoParamSetter,
    unit: &Unit,
    image: &UIImage,
    enhance: i32,
    _method: OptionalMethod,
) {
    let game_object = unsafe { unity_engine_component_get_game_object(image, None) };
    let should_show = enhance != 0;
    unsafe { app_info_util_try_set_active(game_object, should_show, None) };
    if should_show {
        if let Some(emblem) = resolve_emblem_unit(&unit) {
            const EMBLEM_DARK_FLAG: i32 = 0x4;
            let is_dark_emblem = emblem.flag.value.contains(EMBLEM_DARK_FLAG);
            let sprite = if !is_dark_emblem == (enhance > 0) {
                this.emblem_buff_sprites[0]
            } else {
                this.emblem_buff_sprites[1]
            };
            unsafe { unity_engine_ui_image_set_sprite(image, sprite, None) }
        }
    }
}

#[skyline::hook(offset = 0x01F93690)]
fn app_unit_info_param_setter_set_param_text(
    this: &UnitInfoParamSetter,
    text: &TextMeshProUGUI,
    image: &UIImage,
    t: i32,
    ex_str: &Il2CppString,
    method: OptionalMethod,
) {
    call_original!(this, text, image, t, ex_str, method);
    let manager = get_instance::<UnitInfoParamManager>();
    let dir = manager.get_param(t).get_enhance_dir(true);
    unit_info_param_setter_set_emblem_buff(this, manager.calc_unit, image, dir, None);
}

fn resolve_emblem_unit(unit: &Unit) -> Option<&GodData> {
    let emblem = unit.god_unit?;
    const MAIN_DATA_MODE: u64 = 0x800000;
    if unit.status.value.contains(MAIN_DATA_MODE) {
        Some(emblem.data.main_data)
    } else {
        Some(emblem.data)
    }
}
#[skyline::from_offset(0x02C46490)]
fn unity_engine_component_get_game_object(image: &UIImage, method: OptionalMethod) -> &GameObject;

#[skyline::from_offset(0x0290F7D0)]
fn app_info_util_try_set_active(game_object: &GameObject, is_active: bool, method: OptionalMethod);

#[skyline::from_offset(0x03BECFF0)]
fn unity_engine_ui_image_set_sprite(this: &UIImage, sprite: &Sprite, method: OptionalMethod);

#[repr(C)]
#[unity::class("App", "UnitStatusSetter_ValueParam")]
struct UnitStatusSetterValueParam {
    junk0: [u8; 0x38],
    pub god_up_value: Option<&'static TextMeshProUGUI>,
}

#[repr(C)]
#[unity::class("TMPro", "TextMeshProUGUI")]
struct TextMeshProUGUI {
    pub base: TmpTextFields,
}

#[repr(C)]
#[unity::class("TMPro", "TMP_Text")]
struct TmpText {
    junk0: [u8; 0xC0],
    pub text: Option<&'static Il2CppString>,
}

impl TextMeshProUGUI {
    pub fn set_text(&self, text: impl AsRef<str>, sync_text_input_box: bool) {
        unsafe { tm_pro_tmp_text_set_text(self, text.as_ref().into(), sync_text_input_box, None) }
    }
}

#[skyline::from_offset(0x02837690)]
fn tm_pro_tmp_text_set_text(
    this: &TextMeshProUGUI,
    source: &'static Il2CppString,
    sync_text_input_box: bool,
    method: OptionalMethod,
);

//
// Change Balloon Colour
//
#[skyline::hook(offset = 0x01B59DB0)]
fn app_unit_status_setter_value_param_set_balloon(
    this: &UnitStatusSetterValueParam,
    up_value: i32,
    ring_type: i32,
    method: OptionalMethod,
) {
    let ring_type = if up_value < 0 {
        reverse_type(ring_type)
    } else {
        ring_type
    };
    call_original!(this, up_value, ring_type, method);
    // if let Some(ref god_up_value) = this.god_up_value {
    //     if let Some(ref text) = god_up_value.base.text {
    //         if up_value < 0 {
    //             let text = format!("({})", text.to_string());
    //             god_up_value.set_text(text.as_str(), false);
    //         }
    //     }
    // }
}

#[inline]
fn reverse_type(t: i32) -> i32 {
    match t {
        2 => 1,
        _ => 2,
    }
}

pub fn install() {
    skyline::install_hooks!(
        app_unit_status_setter_value_param_set_balloon,
        unit_info_param_setter_set_emblem_buff,
        app_unit_info_param_setter_set_param_text,
    );
    // buff <= 0 -> buff == 0
    // b.le to b.eq
    Patch::in_text(0x01B59E14)
        .bytes([0xC0, 0x07, 0x00, 0x54])
        .unwrap()
}
