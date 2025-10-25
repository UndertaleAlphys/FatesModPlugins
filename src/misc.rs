mod difficulty_select;
mod no_investment;
mod no_skirmishes;
mod no_well;
mod patch_msg;
mod save_header;
mod play_damage;

pub fn install() {
    no_well::install();
    no_investment::install();
    no_skirmishes::install();
    patch_msg::install();
    save_header::install();
    difficulty_select::install();
    play_damage::install();
}
