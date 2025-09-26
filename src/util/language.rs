use unity::prelude::*;
pub const SIMPLIFIED_CHINESE: i32 = 10;

#[unity::from_offset("App", "Language", "GetLang")]
fn engage_get_language(method_info: OptionalMethod) -> i32;
pub fn get() -> i32 {
    unsafe { engage_get_language(None) }
}
