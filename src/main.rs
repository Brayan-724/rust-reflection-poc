#![allow(incomplete_features)]
#![allow(internal_features)]
#![feature(const_trait_impl)]
#![feature(panic_internals)]
#![feature(specialization)]

use reflection::{field, impl_of, CompTimeReflected};

#[impl_of]
trait Sound {
    fn make_sound(&self);
}

#[derive(CompTimeReflected)]
struct Kitty {
    name: String,
}

impl Sound for Kitty {
    fn make_sound(&self) {
        println!("purr");
    }
}

struct Space {
    name: field!(Kitty.name)
}

fn make_sound<T: ImplOfSound>(this: T) {
    this.make_sound();
}

fn main() {
    type Name = field!(Kitty.name);
    let name: Name = "".to_owned();

    type Age = field!(Kitty.age); // compile fail
    let _: Age = (); // Needed for lazy type/const eval
    
    let name: Name = 0; // compile fail

    make_sound(Kitty {
        name: "".to_owned(),
    });
    make_sound(Space {
        name: "".to_owned()
    });
}
