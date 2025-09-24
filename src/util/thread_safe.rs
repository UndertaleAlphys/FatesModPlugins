use std::sync::{Mutex, MutexGuard, OnceLock};

pub struct Safe<T> {
    inner: OnceLock<Mutex<T>>,
}

impl<T> Safe<T> {
    pub const fn new() -> Self {
        Self {
            inner: OnceLock::new(),
        }
    }

    fn ensure_with(&self, make: impl FnOnce() -> T) {
        if self.inner.get().is_none() {
            let _ = self.inner.set(Mutex::new(make()));
        }
    }

    pub fn set_value(&self, value: T, make_default: impl FnOnce() -> T) {
        self.ensure_with(make_default);
        *self.inner.get().unwrap().lock().unwrap() = value;
    }

    pub fn get_cloned(&self, make_default: impl FnOnce() -> T) -> T
    where
        T: Clone,
    {
        self.ensure_with(make_default);
        self.inner.get().unwrap().lock().unwrap().clone()
    }

    pub fn update(&self, f: impl FnOnce(&mut T), make_default: impl FnOnce() -> T) {
        self.ensure_with(make_default);
        let mut guard: MutexGuard<'_, T> = self.inner.get().unwrap().lock().unwrap();
        f(&mut *guard);
    }
}
