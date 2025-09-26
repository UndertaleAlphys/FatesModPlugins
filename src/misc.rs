mod no_investment;
mod no_well;
mod patch_msg;

pub fn install() {
    no_well::install();
    no_investment::install();
    patch_msg::install();
}
