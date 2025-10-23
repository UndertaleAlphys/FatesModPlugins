use engage::calculator::GameCalculatorCommand;
use unity::il2cpp;
use unity::macro_context::Il2CppClass;

pub trait CanGetClass {
    fn get_class_mut_if_mod(&mut self) -> &mut Il2CppClass;
    fn get_class_if_mod(&mut self) -> &Il2CppClass;
}

pub trait UnityClassTrait {
    fn assign_virtual_method(&mut self, method_name: impl AsRef<str>, method_pointer: *mut u8);
    fn assign_vtable(&mut self, vtable_index: usize, method_pointer: *mut u8);
    fn clone(&mut self) -> Option<&'static mut GameCalculatorCommand>;
}

impl<T> UnityClassTrait for T
where
    T: CanGetClass,
{
    fn assign_virtual_method(&mut self, method_name: impl AsRef<str>, method_pointer: *mut u8) {
        self.get_class_mut_if_mod()
            .get_virtual_method_mut(method_name)
            .map(|method| method.method_ptr = method_pointer);
    }
    fn assign_vtable(&mut self, vtable_index: usize, method_pointer: *mut u8) {
        self.get_class_mut_if_mod().get_vtable_mut()[vtable_index].method_ptr = method_pointer;
    }
    fn clone(&mut self) -> Option<&'static mut GameCalculatorCommand> {
        match il2cpp::instantiate_class::<GameCalculatorCommand>(self.get_class_if_mod().clone()) {
            Ok(cmd) => Some(cmd),
            Err(_) => None,
        }
    }
}
