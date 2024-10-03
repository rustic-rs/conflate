//! Merge strategies for `BTreeMap`s.
//!
//! These strategies are only available if the `std` feature is enabled.

use std::collections::BTreeMap;

/// Append values, on conflict, overwrite elements of `left` with `right`.
///
/// In other words, this gives precedence to `right`.
pub fn append_or_overwrite<K: Eq + Ord, V>(left: &mut BTreeMap<K, V>, right: BTreeMap<K, V>) {
    left.extend(right)
}

/// Append values, on conflict, ignore elements from `right`.
///
/// In other words, this gives precedence to `left`.
pub fn append_or_ignore<K: Eq + Ord, V>(left: &mut BTreeMap<K, V>, right: BTreeMap<K, V>) {
    for (k, v) in right {
        left.entry(k).or_insert(v);
    }
}

/// Append values, on conflict, recursively merge the elements.
pub fn append_or_recurse<K: Eq + Ord, V: crate::Merge>(
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
