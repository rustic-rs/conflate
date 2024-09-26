// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Merge strategies for types that form a total order.

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
