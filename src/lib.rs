// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "derive")]
pub use merge_derive::*;

pub trait Merge {
    fn merge(&mut self, other: Self);
}

impl<T> Merge for Option<T> {
    fn merge(&mut self, mut other: Self) {
        if !self.is_some() {
            *self = other.take();
        }
    }
}

pub mod bool {
    pub fn overwrite_false(left: &mut bool, right: bool) {
        if !*left {
            *left = right;
        }
    }

    pub fn overwrite_true(left: &mut bool, right: bool) {
        if *left {
            *left = right;
        }
    }
}

#[cfg(feature = "num")]
pub mod num {
    pub fn saturating_add<T: num_traits::SaturatingAdd>(left: &mut T, right: T) {
        *left = left.saturating_add(&right);
    }

    pub fn overwrite_zero<T: num_traits::Zero>(left: &mut T, right: T) {
        if left.is_zero() {
            *left = right;
        }
    }
}

pub mod ord {
    use core::cmp;

    pub fn max<T: cmp::Ord>(left: &mut T, right: T) {
        if cmp::Ord::cmp(left, &right) == cmp::Ordering::Less {
            *left = right;
        }
    }

    pub fn min<T: cmp::Ord>(left: &mut T, right: T) {
        if cmp::Ord::cmp(left, &right) == cmp::Ordering::Greater {
            *left = right;
        }
    }
}

#[cfg(feature = "std")]
pub mod vec {
    pub fn overwrite_empty<T>(left: &mut Vec<T>, mut right: Vec<T>) {
        if left.is_empty() {
            left.append(&mut right);
        }
    }

    pub fn append<T>(left: &mut Vec<T>, mut right: Vec<T>) {
        left.append(&mut right);
    }

    pub fn prepend<T>(left: &mut Vec<T>, mut right: Vec<T>) {
        right.append(left);
        *left = right;
    }
}
