<!---
Copyright (C) 2020 Robin Krahl <robin.krahl@ireas.org>
SPDX-License-Identifier: CC0-1.0
-->

# merge-derive

This crate provides a derive macro for the `merge::Merge` crate. See the
[`merge`][`merge`] crate for more information.

## Minimum Rust version policy

This crate's minimum supported `rustc` version is `1.64.0`.

The current policy is that the minimum Rust version required to use this crate
can be increased in minor version updates. For example, if `crate 1.0` requires
Rust 1.20.0, then `crate 1.0.z` for all values of `z` will also require Rust
1.20.0 or newer. However, `crate 1.y` for `y > 0` may require a newer minimum
version of Rust.

In general, this crate will be conservative with respect to the minimum
supported version of Rust.

[`merge`]: https://lib.rs/crates/merge
