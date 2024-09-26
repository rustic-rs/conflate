// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Merge strategies for numeric types.
//!
//! These strategies are only available if the `num` feature is enabled.

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
