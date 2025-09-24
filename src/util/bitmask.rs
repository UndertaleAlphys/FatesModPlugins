use std::ops::{BitAnd, BitOr, Not};

pub trait BitMask:
    Copy + BitAnd<Output = Self> + BitOr<Output = Self> + Not<Output = Self> + PartialEq
{
    #[inline]
    fn contains(self, other: Self) -> bool {
        (self & other) == other
    }

    #[inline]
    fn add(self, other: Self) -> Self {
        self | other
    }

    #[inline]
    fn remove(self, other: Self) -> Self {
        self & !other
    }
}

impl<T> BitMask for T where
    T: Copy + BitAnd<Output = T> + BitOr<Output = T> + Not<Output = T> + PartialEq
{
}
