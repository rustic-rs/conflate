// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Merge strategies for `BTreeMap`s.
//!
//! These strategies are only available if the `std` feature is enabled.

use std::collections::BTreeMap;
use std::hash::Hash;

/// On conflict, overwrite elements of `left` with `right`.
///
/// In other words, this gives precedence to `right`.
pub fn overwrite<K: Eq + Hash + Ord, V>(left: &mut BTreeMap<K, V>, right: BTreeMap<K, V>) {
    left.extend(right)
}

/// On conflict, ignore elements from `right`.
///
/// In other words, this gives precedence to `left`.
pub fn ignore<K: Eq + Hash + Ord, V>(left: &mut BTreeMap<K, V>, right: BTreeMap<K, V>) {
    for (k, v) in right {
        left.entry(k).or_insert(v);
    }
}

/// On conflict, recursively merge the elements.
pub fn recurse<K: Eq + Hash + Ord, V: crate::Merge>(
    left: &mut BTreeMap<K, V>,
    right: BTreeMap<K, V>,
) {
    use std::collections::btree_map::Entry;

    for (k, v) in right {
        match left.entry(k) {
            Entry::Occupied(mut existing) => existing.get_mut().merge(v),
            Entry::Vacant(empty) => {
                empty.insert(v);
            }
        }
    }
}
