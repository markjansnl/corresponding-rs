//! [![github]](https://github.com/markjansnl/corresponding-rs)&ensp;[![crates-io]](https://crates.io/crates/corresponding)&ensp;[![docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! Use this crate to copy structs within a module to each other. Only fields with the same name and type will be copied,
//! hence the crate name corresponding.
//!
//! By adding the attribute [derive_corresponding] to a module, the trait [MoveCorresponding] will be implemented
//! for all the structs within the module. This makes it possible to call the [move_corresponding](MoveCorresponding::move_corresponding)
//! function on all the structs in the attributed module with all the structs in the module as parameter.
//!
//! The trait [From] will be implemented for all structs in the module that derive [Default] for all other structs in the module.
//!
//! # Example
//!
//! Put the [derive_corresponding] attribute on a module. This module must be declared inline:
//!
//! ```no_run
//! use corresponding::derive_corresponding;
//!
//! #[derive_corresponding]
//! mod my_mod {
//!     #[derive(Debug, Default)]
//!     pub struct A {
//!         pub a: u8,
//!         pub b: u8,
//!         pub c: u8,
//!     }
//!
//!     #[derive(Debug, Clone)]
//!     pub struct B {
//!         pub a: u8,
//!         pub b: Option<u8>,
//!         pub d: u8,
//!     }
//! }
//! ```
//!
//! Make sure [MoveCorresponding] is in scope and start moving corresponding fields from `B` to `A` and vice versa:
//!
//! ```
//! # use corresponding::derive_corresponding;
//! # #[derive_corresponding]
//! # mod my_mod {
//! #     #[derive(Debug, Default)]
//! #     pub struct A {
//! #         pub a: u8,
//! #         pub b: u8,
//! #         pub c: u8,
//! #     }
//! #
//! #     #[derive(Debug, Clone)]
//! #     pub struct B {
//! #         pub a: u8,
//! #         pub b: Option<u8>,
//! #         pub d: u8,
//! #     }
//! # }
//! use corresponding::MoveCorresponding;
//! use my_mod::*;
//!
//! fn start_moving() {
//!     let mut a = A { a: 1, b: 1, c: 1 };
//!     let mut b = B { a: 2, b: Some(2), d: 2 };
//!
//!     a.move_corresponding(b.clone());
//!     println!("{a:?}");      // Output: A { a: 2, b: 2, c: 1 }
//!
//!     let a2 = A { a: 3, b: 3, c: 3 };
//!     b.move_corresponding(a2);
//!     println!("{b:?}");      // Output: B { a: 3, b: Some(3), d: 2 }
//! }
//! ```
//!
//! Because struct `A` derives [Default], it will also implement [From]. So you can transform `B` to `A`:
//!
//! ```
//! # use corresponding::derive_corresponding;
//! # #[derive_corresponding]
//! # mod my_mod {
//! #     #[derive(Debug, Default)]
//! #     pub struct A {
//! #         pub a: u8,
//! #         pub b: u8,
//! #         pub c: u8,
//! #     }
//! #
//! #     #[derive(Debug, Clone)]
//! #     pub struct B {
//! #         pub a: u8,
//! #         pub b: Option<u8>,
//! #         pub d: u8,
//! #     }
//! # }
//! # use corresponding::MoveCorresponding;
//! # use my_mod::*;
//! fn start_transforming() {
//!     let b = B { a: 4, b: Some(4), d: 4 };
//!
//!     let a: A = b.into();
//!     println!("{a:?}");      // Output: A { a: 4, b: 4, c: 0 }
//! }
//! ```
//!
//! Struct `B` doesn't derive [Default], so you cannot transform `A` to `B`. [From] is not implemented for this case.
//!
//! # Options
//!
//! Also fields with types `T` and `Option<T>` are considered corresponding.
//!
//! - Moving `Option<T>` to `T` will only set the target field when the source field is `Some(value)`
//! - Moving `T` to `Option<T>` will always set the target field with `Some(value)`
//! - Moving `Option<T>` to `Option<T>` will only set the target field when the source field is `Some(value)`
//!
//! This means there is no way of setting an [Option] to [None] by using [move_corresponding](MoveCorresponding::move_corresponding).
//!
//! Deeper nested [Option]s are not supported, so `Option<Option<V>>` is considered as `Option<T>` with `T` = `Option<V>`.

pub use corresponding_macros::derive_corresponding;
pub mod prelude;

/// Trait holding the [move_corresponding](MoveCorresponding::move_corresponding) function.
pub trait MoveCorresponding<R> {
    /// Move the corresponding fields from `rhs` to `self`.
    ///
    /// See the [crate] documentation for more information.
    ///
    /// # Example
    ///
    /// For a module with the `derive_corresponding` attribute. The following code does not compile in rustdoc, because you cannot use
    /// a module in another file or folder. It does compile in the real world though.
    ///
    /// ```no_run
    /// use corresponding::derive_corresponding;
    ///
    /// #[derive_corresponding]
    /// mod my_mod {
    ///     #[derive(Debug, Default)]
    ///     pub struct A {
    ///         pub a: u8,
    ///         pub b: u8,
    ///         pub c: u8,
    ///     }
    ///
    ///     #[derive(Debug, Clone)]
    ///     pub struct B {
    ///         pub a: u8,
    ///         pub b: Option<u8>,
    ///         pub d: u8,
    ///     }
    /// }
    /// ```
    ///
    /// Make sure [MoveCorresonding](MoveCorresponding) is in scope and start moving the corresponding fields from `B` to `A` and vice versa:
    ///
    /// ```
    /// # use corresponding::derive_corresponding;
    /// # #[derive_corresponding]
    /// # mod my_mod {
    /// #     #[derive(Debug, Default)]
    /// #     pub struct A {
    /// #         pub a: u8,
    /// #         pub b: u8,
    /// #         pub c: u8,
    /// #     }
    /// #
    /// #     #[derive(Debug, Clone)]
    /// #     pub struct B {
    /// #         pub a: u8,
    /// #         pub b: Option<u8>,
    /// #         pub d: u8,
    /// #     }
    /// # }
    /// use corresponding::MoveCorresponding;
    /// use my_mod::*;
    ///
    /// fn start_moving() {
    ///     let mut a = A { a: 1, b: 1, c: 1 };
    ///     let mut b = B { a: 2, b: Some(2), d: 2 };
    ///
    ///     a.move_corresponding(b.clone());
    ///     println!("{a:?}");      // Output: A { a: 2, b: 2, c: 1 }
    ///
    ///     let a2 = A { a: 3, b: 3, c: 3 };
    ///     b.move_corresponding(a2);
    ///     println!("{b:?}");      // Output: B { a: 3, b: Some(3), d: 2 }
    /// }
    /// ```
    fn move_corresponding(&mut self, rhs: R);
}

pub trait CloneCorresponding<R: Clone> {
    fn clone_corresponding(&mut self, rhs: &R);
}

pub trait FromCloned<R: Clone>: Default {
    fn from_cloned(rhs: &R) -> Self;
}

pub trait ClonedInto<L: FromCloned<Self>>: Clone {
    fn cloned_into(&self) -> L;
}

impl<L: FromCloned<R>, R: Clone> ClonedInto<L> for R {
    fn cloned_into(&self) -> L {
        L::from_cloned(self)
    }
}
