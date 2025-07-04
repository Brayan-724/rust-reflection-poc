#![allow(incomplete_features)]
#![allow(internal_features)]
#![feature(associated_type_defaults)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(never_type)]
#![feature(more_maybe_bounds)]

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

enum AdtId {
    Struct,
}

trait AdtDescriptor {
    const ID: AdtId;
    const NAME: &'static str;
}

struct NoType;

trait FieldsAdt {
    type Fields = NoType;
    const FIELD_COUNT: usize = 0;
}

#[const_trait]
trait StructDefine {
    type Fields = NoType;

    fn named_field(field: &'static str) -> usize;
}

const fn named_field<T: [const] StructDefine>(field: &'static str) -> usize {
    <T as StructDefine>::named_field(field)
}

trait GetFieldT<const FIELD: usize> {
    type Type = NoType;
}

type GetField<T, const FIELD: usize> = <T as GetFieldT<FIELD>>::Type;

impl const StructDefine for Kitty {
    fn named_field(field: &'static str) -> usize {
        match field {
            _ if str_equals(field, "name") => 0,
            _ if str_equals(field, "decibels") => 1,
            _ => usize::MAX,
        }
    }
}

impl GetFieldT<0> for Kitty {
    type Type = String;
}

impl GetFieldT<1> for Kitty {
    type Type = u8;
}

// impl dyn Sound {
//     fn make_sound() {
//         unimplemented!()
//     }
// }

trait HasImplOf<As: ?Sized> {
    const HAS: bool = false;
}

trait AsImplOf<As: ?Sized> {
    fn as_impl_of(&self) -> &As;
}

impl<As: ?Sized, T> HasImplOf<As> for T {
    default const HAS: bool = false;
}

impl<As: ?Sized, T> AsImplOf<As> for T {
    default fn as_impl_of(&self) -> &As {
        panic!("{} has not implemented {}", any::type_name::<T>(), any::type_name::<As>())
    }
}

impl<T: Sound> HasImplOf<dyn Sound> for T {
    const HAS: bool = true;
}

impl<T: Sound + 'static> AsImplOf<dyn Sound> for T {
    fn as_impl_of(&self) -> &(dyn Sound + 'static) {
        self
    }
}

const fn has_impl_of<As: ?Sized, T: HasImplOf<As>>() -> bool {
    <T as HasImplOf<As>>::HAS
}

fn as_impl_of<As: ?Sized, T: AsImplOf<As>>(this: &T) -> &As {
    <T as AsImplOf<As>>::as_impl_of(this)
}

// #[derive(CompTimeReflected)]
struct Kitty;

impl Sound for Kitty {
    fn make_sound(&self) {
        println!("purr");
    }
}

struct Space;

trait Sound {
    fn make_sound(&self);
}

fn make_sound<T>(this: T) {
    as_impl_of::<dyn Sound, T>(&this).make_sound();
}

fn main() {
    // const NAME_FIELD: usize = named_field::<Kitty>("name");
    // type Name = GetField<Kitty, NAME_FIELD>;
    // let name: Name = 0;

    make_sound(Kitty);
    make_sound(Space);
}
