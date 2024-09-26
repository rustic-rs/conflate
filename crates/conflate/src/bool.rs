// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Merge strategies for boolean types.

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
