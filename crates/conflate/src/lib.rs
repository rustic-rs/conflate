// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Provides [`Merge`][], a trait for objects that can be merged.
//!
//! # Usage
//!
//! ```
//! trait Merge {
//!     fn merge(&mut self, other: Self);
//! }
//! ```
//!
//! The [`Merge`][] trait can be used to merge two objects of the same type into one.  The intended
//! use case is merging configuration from different sources, for example environment variables,
//! multiple configuration files and command-line arguments, see the [`args.rs`][] example.
//!
//! This crate does not provide any `Merge` implementations, but `Merge` can be derived for
//! structs.  When deriving the `Merge` trait for a struct, you can provide custom merge strategies
//! for the fields that don’t implement `Merge`.  A merge strategy is a function with the signature
//! `fn merge<T>(left: &mut T, right: T)` that merges `right` into `left`.  The submodules of this
//! crate provide strategies for the most common types, but you can also define your own
//! strategies.
//!
//! ## Features
//!
//! This crate has the following features:
//!
//! - `derive` (default):  Enables the derive macro for the `Merge` trait using the `conflate_derive`
//!   crate.
//! - `num` (default): Enables the merge strategies in the `num` module that require the
//!   `num_traits` crate.
//! - `std` (default): Enables the merge strategies in the `hashmap` and `vec` modules that require
//!    the standard library.  If this feature is not set, `conflate` is a `no_std` library.
//!
//! # Example
//!
//! ```
//! use conflate::Merge;
//!
//! #[derive(Merge)]
//! struct User {
//!     // Fields with the skip attribute are skipped by Merge
//!     #[merge(skip)]
//!     pub name: &'static str,
//!
//!     // The strategy attribute is used to customize the merge behavior
//!     #[merge(strategy = conflate::option::overwrite_none)]
//!     pub location: Option<&'static str>,
//!
//!     #[merge(strategy = conflate::vec::append)]
//!     pub groups: Vec<&'static str>,
//! }
//!
//! let defaults = User {
//!     name: "",
//!     location: Some("Internet"),
//!     groups: vec!["rust"],
//! };
//! let mut ferris = User {
//!     name: "Ferris",
//!     location: None,
//!     groups: vec!["mascot"],
//! };
//! ferris.merge(defaults);
//!
//! assert_eq!("Ferris", ferris.name);
//! assert_eq!(Some("Internet"), ferris.location);
//! assert_eq!(vec!["mascot", "rust"], ferris.groups);
//! ```
//!
//! [`Merge`]: trait.Merge.html
//! [`args.rs`]: https://git.sr.ht/~ireas/merge-rs/tree/master/examples/args.rs

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "derive")]
pub use conflate_derive::*;

pub mod bool;
#[cfg(feature = "std")]
pub mod hashmap;
pub mod iter;
#[cfg(feature = "num")]
pub mod num;
pub mod option;
pub mod ord;
#[cfg(feature = "std")]
pub mod vec;

/// A trait for objects that can be merged.
///
/// # Deriving
///
/// `Merge` can be derived for structs if the `derive` feature is enabled.  The generated
/// implementation calls the `merge` method for all fields, or the merge strategy function if set.
/// You can use these field attributes to configure the generated implementation:
/// - `skip`: Skip this field in the `merge` method.
/// - `strategy = f`: Call `f(self.field, other.field)` instead of calling the `merge` function for
///    this field.
///
/// You can also set a default strategy for all fields by setting the `strategy` attribute for the
/// struct.
///
/// # Examples
///
/// Deriving `Merge` for a struct:
///
/// ```
/// use conflate::Merge;
///
/// #[derive(Debug, PartialEq, Merge)]
/// struct S {
///     #[merge(strategy = conflate::option::overwrite_none)]
///     option: Option<usize>,
///
///     #[merge(skip)]
///     s: String,
///
///     #[merge(strategy = conflate::bool::overwrite_false)]
///     flag: bool,
/// }
///
/// let mut val = S {
///     option: None,
///     s: "some ignored value".to_owned(),
///     flag: false,
/// };
/// val.merge(S {
///     option: Some(42),
///     s: "some other ignored value".to_owned(),
///     flag: true,
/// });
/// assert_eq!(S {
///     option: Some(42),
///     s: "some ignored value".to_owned(),
///     flag: true,
/// }, val);
/// ```
///
/// Setting a default merge strategy:
///
/// ```
/// use conflate::Merge;
///
/// #[derive(Debug, PartialEq, Merge)]
/// #[merge(strategy = conflate::option::overwrite_none)]
/// struct S {
///     option1: Option<usize>,
///     option2: Option<usize>,
///     option3: Option<usize>,
/// }
///
/// let mut val = S {
///     option1: None,
///     option2: Some(1),
///     option3: None,
/// };
/// val.merge(S {
///     option1: Some(2),
///     option2: Some(2),
///     option3: None,
/// });
/// assert_eq!(S {
///     option1: Some(2),
///     option2: Some(1),
///     option3: None,
/// }, val);
/// ```
pub trait Merge {
    /// Merge another object into this object.
    fn merge(&mut self, other: Self);
}
