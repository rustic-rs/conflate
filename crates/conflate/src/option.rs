// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Merge strategies for `Option`

/// Overwrite the left value with the right value if the right value is `Some`.
pub fn overwrite_with_some<T>(left: &mut Option<T>, right: Option<T>) {
    if right.is_some() {
        *left = right;
    }
}

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
