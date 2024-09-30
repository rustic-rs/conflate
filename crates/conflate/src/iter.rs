//! Merge strategies for iterable types.

/// Set left to the saturated some of left and right.
pub fn extend<A, T: IntoIterator<Item = A> + Extend<A>>(left: &mut T, right: T) {
    left.extend(right);
}
