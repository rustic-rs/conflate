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
//! - `derive` (default):  Enables the derive macro for the `Merge` trait using the `merge_derive`
//!   crate.
//! - `num` (default): Enables the merge strategies in the `num` module that require the
//!   `num_traits` crate.
//! - `std` (default): Enables the merge strategies in the `hashmap` and `vec` modules that require
//!    the standard library.  If this feature is not set, `merge` is a `no_std` library.
//!
//! # Example
//!
//! ```
//! use merge::Merge;
//!
//! #[derive(Merge)]
//! struct User {
//!     // Fields with the skip attribute are skipped by Merge
//!     #[merge(skip)]
//!     pub name: &'static str,
//!
//!     // The strategy attribute is used to customize the merge behavior
//!     #[merge(strategy = merge::option::overwrite_none)]
//!     pub location: Option<&'static str>,
//!
//!     #[merge(strategy = merge::vec::append)]
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
pub use merge_derive::*;

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
/// use merge::Merge;
///
/// #[derive(Debug, PartialEq, Merge)]
/// struct S {
///     #[merge(strategy = merge::option::overwrite_none)]
///     option: Option<usize>,
///
///     #[merge(skip)]
///     s: String,
///
///     #[merge(strategy = merge::bool::overwrite_false)]
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
/// use merge::Merge;
///
/// #[derive(Debug, PartialEq, Merge)]
/// #[merge(strategy = merge::option::overwrite_none)]
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

/// Merge strategies for `Option`
pub mod option {
    /// Overwrite `left` with `right` only if `left` is `None`.
    pub fn overwrite_none<T>(left: &mut Option<T>, right: Option<T>) {
        if left.is_none() {
            *left = right;
        }
    }

    /// If both `left` and `right` are `Some`, recursively merge the two.
    /// Otherwise, fall back to `overwrite_none`.
    pub fn recurse<T: crate::Merge>(left: &mut Option<T>, right: Option<T>) {
        if let Some(new) = right {
            if let Some(original) = left {
                original.merge(new);
            } else {
                *left = Some(new);
            }
        }
    }
}

/// Merge strategies for boolean types.
pub mod bool {
    /// Overwrite left with right if the value of left is false.
    pub fn overwrite_false(left: &mut bool, right: bool) {
        if !*left {
            *left = right;
        }
    }

    /// Overwrite left with right if the value of left is true.
    pub fn overwrite_true(left: &mut bool, right: bool) {
        if *left {
            *left = right;
        }
    }
}

/// Merge strategies for numeric types.
///
/// These strategies are only available if the `num` feature is enabled.
#[cfg(feature = "num")]
pub mod num {
    /// Set left to the saturated some of left and right.
    pub fn saturating_add<T: num_traits::SaturatingAdd>(left: &mut T, right: T) {
        *left = left.saturating_add(&right);
    }

    /// Overwrite left with right if the value of left is zero.
    pub fn overwrite_zero<T: num_traits::Zero>(left: &mut T, right: T) {
        if left.is_zero() {
            *left = right;
        }
    }
}

/// Merge strategies for types that form a total order.
pub mod ord {
    use core::cmp;

    /// Set left to the maximum of left and right.
    pub fn max<T: cmp::Ord>(left: &mut T, right: T) {
        if cmp::Ord::cmp(left, &right) == cmp::Ordering::Less {
            *left = right;
        }
    }

    /// Set left to the minimum of left and right.
    pub fn min<T: cmp::Ord>(left: &mut T, right: T) {
        if cmp::Ord::cmp(left, &right) == cmp::Ordering::Greater {
            *left = right;
        }
    }
}

/// Merge strategies for vectors.
///
/// These strategies are only available if the `std` feature is enabled.
#[cfg(feature = "std")]
pub mod vec {
    /// Overwrite left with right if left is empty.
    pub fn overwrite_empty<T>(left: &mut Vec<T>, mut right: Vec<T>) {
        if left.is_empty() {
            left.append(&mut right);
        }
    }

    /// Append the contents of right to left.
    pub fn append<T>(left: &mut Vec<T>, mut right: Vec<T>) {
        left.append(&mut right);
    }

    /// Prepend the contents of right to left.
    pub fn prepend<T>(left: &mut Vec<T>, mut right: Vec<T>) {
        right.append(left);
        *left = right;
    }
}

/// Merge strategies for hash maps.
///
/// These strategies are only available if the `std` feature is enabled.
#[cfg(feature = "std")]
pub mod hashmap {
    use std::collections::HashMap;
    use std::hash::Hash;

    /// On conflict, overwrite elements of `left` with `right`.
    ///
    /// In other words, this gives precedence to `right`.
    pub fn overwrite<K: Eq + Hash, V>(left: &mut HashMap<K, V>, right: HashMap<K, V>) {
        left.extend(right.into_iter())
    }

    /// On conflict, ignore elements from `right`.
    ///
    /// In other words, this gives precedence to `left`.
    pub fn ignore<K: Eq + Hash, V>(left: &mut HashMap<K, V>, right: HashMap<K, V>) {
        for (k, v) in right {
            left.entry(k).or_insert(v);
        }
    }

    /// On conflict, recursively merge the elements.
    pub fn recurse<K: Eq + Hash, V: crate::Merge>(left: &mut HashMap<K, V>, right: HashMap<K, V>) {
        use std::collections::hash_map::Entry;

        for (k, v) in right {
            match left.entry(k) {
                Entry::Occupied(mut existing) => existing.get_mut().merge(v),
                Entry::Vacant(empty) => {
                    empty.insert(v);
                }
            }
        }
    }
}
