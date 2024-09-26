// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Merge strategies for vectors.
//!
//! These strategies are only available if the `std` feature is enabled.

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
