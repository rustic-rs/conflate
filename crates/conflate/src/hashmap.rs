// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Merge strategies for hash maps.
//!
//! These strategies are only available if the `std` feature is enabled.

use std::collections::HashMap;
use std::hash::Hash;

/// Append values, on conflict, overwrite elements of `left` with `right`.
///
/// In other words, this gives precedence to `right`.
pub fn append_or_overwrite<K: Eq + Hash, V>(left: &mut HashMap<K, V>, right: HashMap<K, V>) {
    left.extend(right)
}

/// Append values, on conflict, ignore elements from `right`.
///
/// In other words, this gives precedence to `left`.
pub fn append_or_ignore<K: Eq + Hash, V>(left: &mut HashMap<K, V>, right: HashMap<K, V>) {
    for (k, v) in right {
        left.entry(k).or_insert(v);
    }
}

/// Append values, on conflict, recursively merge the elements.
pub fn append_or_recurse<K: Eq + Hash, V: crate::Merge>(
    left: &mut HashMap<K, V>,
    right: HashMap<K, V>,
) {
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
