extern "C" fn get___SNAKENAME___command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    Il2CppString::new("__CAMELNAME__")
}

extern "C" fn get___SNAKENAME___unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    _method: OptionalMethod,
) -> f32 {
    unit.map_or(0f32, |u| u.get_variable("__CAMELNAME__") as f32)
}

extern "C" fn get___SNAKENAME___battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    _method: OptionalMethod,
) -> f32 {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map_or(0f32, |u| u.get_variable("__CAMELNAME__") as f32)
}

extern "C" fn set___SNAKENAME___unit(
    _this: &GameCalculatorCommand,
    unit: Option<&Unit>,
    value: f32,
    _method: OptionalMethod,
) {
    unit.map(|u| u.set_variable("__CAMELNAME__", value as i32));
}

extern "C" fn set___SNAKENAME___battle_info(
    _this: &GameCalculatorCommand,
    battle_info_side: Option<&BattleInfoSide>,
    value: f32,
    _method: OptionalMethod,
) {
    let unit = battle_info_side.map(|b| b.unit).unwrap_or(None);
    unit.map(|u| u.set_variable("__CAMELNAME__", value as i32));
}


