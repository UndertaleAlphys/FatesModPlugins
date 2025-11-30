use engage::script::DynValue;
use unity::prelude::OptionalMethod;

pub trait DynValueTrait {
    fn new_nil() -> &'static DynValue;
}

impl DynValueTrait for DynValue {
    fn new_nil() -> &'static DynValue {
        unsafe { dyn_value_new_nil(None) }
    }
}

#[skyline::from_offset(0x02e3ce30)]
fn dyn_value_new_nil(method: OptionalMethod) -> &'static DynValue;
