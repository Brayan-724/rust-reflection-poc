#![allow(incomplete_features)]
#![allow(internal_features)]
#![feature(const_trait_impl)]
#![feature(panic_internals)]
#![feature(specialization)]

pub use reflection_macros::*;

use std::any;

/// A function which takes 2 byte slices and compares them for strict equality, in both values and size.
pub const fn u8_slice_equals(x: &[u8], y: &[u8]) -> bool {
    if x.len() != y.len() {
        return false;
    }

    let mut first = x;
    let mut second = y;
    while let ([h1, tail1 @ ..], [h2, tail2 @ ..]) = (first, second) {
        if *h1 != *h2 {
            return false;
        }

        first = tail1;
        second = tail2;
    }

    true
}

/// A function which takes 2 string slices and compares them for strict equality, in both values and size.
pub const fn str_equals(x: &str, y: &str) -> bool {
    u8_slice_equals(x.as_bytes(), y.as_bytes())
}

#[const_trait]
pub trait StructDefine {
    fn named_field(field: &'static str) -> usize;
    fn named_field_checked(field: &'static str, message: &'static str) -> usize;
}

pub const fn named_field<T: const StructDefine>(field: &'static str) -> usize {
    <T as StructDefine>::named_field(field)
}

pub const fn named_field_checked<T: const StructDefine>(
    field: &'static str,
    message: &'static str,
) -> usize {
    <T as StructDefine>::named_field_checked(field, message)
}

pub trait GetFieldT<const FIELD: usize> {
    type Type;
}

pub type GetField<T, const FIELD: usize> = <T as GetFieldT<FIELD>>::Type;

#[macro_export]
macro_rules! field {
    ($T:ident.$field:ident) => {
        $crate::GetField<$T, {$crate::named_field_checked::<$T>(stringify!($field), concat!(stringify!($field), " doesn't exist on ", stringify!($T)))}>
    };
}
