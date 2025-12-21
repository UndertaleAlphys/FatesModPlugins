use crate::util::class::{CanGetClass, UnityClassTrait};
use engage::calculator::{
    CalculatorCommand, CalculatorManager, GameCalculatorCommand, IsCalculatorCommand,
};
use unity::il2cpp::object::Array;
use unity::il2cpp::{self};
use unity::prelude::{Il2CppClass, OptionalMethod};
use unity::system::Il2CppString;

#[unity::class("App", "List")]
pub struct ListFloats {
    pub items: &'static Array<f32>,
    pub size: i32,
    pub version: i32,
}

pub trait CalculatorManagerTrait {
    fn find_checked(
        &mut self,
        command_name: impl AsRef<str>,
    ) -> Option<&'static mut CalculatorCommand>;
    fn clone_from_name(
        &mut self,
        command_name: impl AsRef<str>,
    ) -> Option<&'static mut GameCalculatorCommand>;
    fn add_with_reverse(&mut self, command: &mut (impl IsCalculatorCommand + CanGetClass));
}

impl CalculatorManagerTrait for CalculatorManager {
    fn find_checked(
        &mut self,
        command_name: impl AsRef<str>,
    ) -> Option<&'static mut CalculatorCommand> {
        unsafe { calculator_manager_find_command(self, command_name.into(), None) }
    }
    fn clone_from_name(
        &mut self,
        command_name: impl AsRef<str>,
    ) -> Option<&'static mut GameCalculatorCommand> {
        let command = self
            .find_checked(command_name)
            .map(|cmd| cmd.get_class().clone());
        if let Some(command) = command {
            match il2cpp::instantiate_class::<GameCalculatorCommand>(command) {
                Ok(cmd) => Some(cmd),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    fn add_with_reverse(&mut self, command: &mut (impl IsCalculatorCommand + CanGetClass)) {
        self.add_command(command);
        if let Some(command) = command.clone() {
            self.add_command(command.reverse());
        }
    }
}

impl CanGetClass for CalculatorCommand {
    fn get_class_mut_if_mod(&mut self) -> &mut Il2CppClass {
        self.get_class_mut()
    }
    fn get_class_if_mod(&mut self) -> &Il2CppClass {
        self.get_class()
    }
}

impl CanGetClass for GameCalculatorCommand {
    fn get_class_mut_if_mod(&mut self) -> &mut Il2CppClass {
        self.get_class_mut()
    }
    fn get_class_if_mod(&mut self) -> &Il2CppClass {
        self.get_class()
    }
}

#[skyline::from_offset(0x0298daa0)]
fn calculator_manager_find_command(
    this: &CalculatorManager,
    name: &Il2CppString,
    method_info: OptionalMethod,
) -> Option<&'static mut CalculatorCommand>;
