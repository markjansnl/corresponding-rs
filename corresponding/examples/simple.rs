// In this example we show that you can copy the fields with the same
// name and type from one struct to the other, by adding the attribute
// `derive_corresponding`. The attribute should be put on a module. All
// structs within this module will get the `move_corresponding` method
// to copy all the structs within the module to each other.
// If a struct derives the Default trait, you can also use `.into()` from
// another struct in the same module.

// In this example there is a datamodel module with a user module.
// The user module has the `derive_corresponding` attribute, so all
// the structs in this module can use the `move_corresponding` method
// Because structs User and UserUpdate derive Default, you can also use
// `.into()` to construct them from another struct.

// Here is the data module with the `derive_corresponding` attribute:

use corresponding::derive_corresponding;

#[derive_corresponding]
pub mod my_mod {
    #[derive(Default, Debug)]
    pub struct A {
        pub a: u8,
        pub b: u8,
        pub c: u8,
    }

    #[derive(Debug, Clone)]
    pub struct B {
        pub a: u8,
        pub b: Option<u8>,
        pub d: u8,
    }
}

// And here we are going to use them. Let's pretend we have a database and
// want to create a user and update it.

use corresponding::MoveCorresponding;
use my_mod::*;

fn main() {
    start_moving();
    start_transforming();
}

fn start_moving() {
    let mut a = A { a: 1, b: 1, c: 1 };
    let mut b = B { a: 2, b: Some(2), d: 2 };

    a.move_corresponding(b.clone());
    println!("{a:?}");      // Output: A { a: 2, b: 2, c: 1 }

    let a2 = A { a: 3, b: 3, c: 3 };
    b.move_corresponding(a2);
    println!("{b:?}");      // Output: B { a: 3, b: Some(3), d: 2 }
}

fn start_transforming() {
    let b = B { a: 4, b: Some(4), d: 4 };

    let a: A = b.into();
    println!("{a:?}");      // Output: A { a: 4, b: 4, c: 0 }
}