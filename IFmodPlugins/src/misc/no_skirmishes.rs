use unity::prelude::OptionalMethod;
#[skyline::hook(offset = 0x0251CA40)]
fn is_check_dispos(this: i64, method: OptionalMethod) -> bool {
    let o_result = call_original!(this, method);
    if o_result {
        // println!("Successfully blocked a skirmish");
    }
    false
}

pub fn install() {
    skyline::install_hook!(is_check_dispos);
}
