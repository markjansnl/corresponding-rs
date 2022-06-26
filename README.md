# Corresponding

This Rust crate can be used to copy structs within a module to each other. Only fields with the same name and type will be copied,
hence the crate name corresponding.

By adding the attribute `derive_corresponding` to a module, the trait `MoveCorresponding` will be implemented
for all the structs within the module. This makes it possible to call the `move_corresponding`
function on all the structs in the attributed module with all the structs in the module as parameter.
The trait `From` will be implemented for all structs in the module that derive `Default` for all other structs in the module.

## Example

Put the `derive_corresponding` attribute on a module:

```rust
use corresponding::*;

// Mod implemented in file or folder
#[derive_corresponding]
mod my_other_mod;

// Mod implemented directly
#[derive_corresponding]
mod my_mod {
    #[derive(Debug, Default)]
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
```

And start moving corresponding fields from `B` to `A` and vice versa:

```rust
use my_mod::*;

fn start_moving() {
    let mut a = A { a: 1, b: 1, c: 1 };
    let mut b = B { a: 2, b: Some(2), d: 2 };

    a.move_corresponding(b.clone());
    println!("{a:?}");      // Output: A { a: 2, b: 2, c: 1 }

    let a2 = A { a: 3, b: 3, c: 3 };
    b.move_corresponding(a2);
    println!("{b:?}");      // Output: B { a: 3, b: Some(3), d: 2 }
}
```

Because struct `A` derives `Default`, it will also implement `From`. So you can transform `B` into `A`:

```rust
fn start_transforming() {
    let b = B { a: 4, b: Some(4), d: 4 };
    let a: A = b.into();
    println!("{a:?}");      // Output: A { a: 4, b: 4, c: 0 }
}
```

Struct `B` doesn't derive `Default`, so you cannot transform `A` to `B`. `From` is not implemented for this case.

Also see a working example in the `examples` folder.

## Options

Also fields with types `T` and `Option<T>` are considered corresponding.

- Moving `Option<T>` to `T` will only set the target field when the source field is `Some(value)`
- Moving `T` to `Option<T>` will always set the target field with `Some(value)`
- Moving `Option<T>` to `Option<T>` will only set the target field when the source field is `Some(value)`

This means there is no way of setting an `Option` to `None` by using `move_corresponding`.

Deeper nested `Option`s are not supported, so `Option<Option<V>>` is considered as `Option<T>` with `T` = `Option<V>`.

## Expand

If you have `cargo-expand` installed, you can see the generated implementations by running:

```console
$ cd corresponding
$ cargo expand --example corresponding
```