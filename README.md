<!---
Copyright (C) 2020 Robin Krahl <robin.krahl@ireas.org>
SPDX-License-Identifier: CC0-1.0
-->

# conflate

<p align="center">
<a href="https://crates.io/crates/conflate"><img src="https://img.shields.io/crates/v/conflate.svg" /></a>
<a href="https://docs.rs/conflate/"><img src="https://img.shields.io/docsrs/conflate?style=flat&amp;labelColor=1c1d42&amp;color=4f396a&amp;logo=Rust&amp;logoColor=white" /></a>
<a href="https://github.com/rustic-rs/conflate/blob/main/"><img src="https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg" /></a>
<a href="https://crates.io/crates/conflate"><img src="https://img.shields.io/crates/d/conflate.svg" /></a>
<p>

The `conflate` crate provides the `Merge` trait that can be used to merge
multiple values into one.

## Contact

You can ask questions in the
[Discussions](https://github.com/rustic-rs/rustic/discussions) or have a look at
the [FAQ](https://rustic.cli.rs/docs/FAQ.html).

| Contact       | Where?                                                                                                          |
| ------------- | --------------------------------------------------------------------------------------------------------------- |
| Issue Tracker | [GitHub Issues](https://github.com/rustic-rs/conflate/issues/choose)                                            |
| Discord       | [![Discord](https://dcbadge.vercel.app/api/server/WRUWENZnzQ?style=flat-square)](https://discord.gg/WRUWENZnzQ) |
| Discussions   | [GitHub Discussions](https://github.com/rustic-rs/rustic/discussions)                                           |

## Usage

```rust
trait Merge {
    fn merge(&mut self, other: Self);
}
```

`Merge` can be derived for structs:

<!-- should be kept in sync with examples/user -->

```rust
use conflate::Merge;

#[derive(Merge)]
struct User {
    // Fields with the skip attribute are skipped by Merge
    #[merge(skip)]
    pub name: &'static str,

    // The strategy attribute is used to select the merge behavior
    #[merge(strategy = conflate::option::overwrite_none)]
    pub location: Option<&'static str>,

    #[merge(strategy = conflate::vec::append)]
    pub groups: Vec<&'static str>,
}

fn main() {
    let defaults = User {
        name: "",
        location: Some("Internet"),
        groups: vec!["rust"],
    };
    let mut ferris = User {
        name: "Ferris",
        location: None,
        groups: vec!["mascot"],
    };
    ferris.merge(defaults);

    assert_eq!("Ferris", ferris.name);
    assert_eq!(Some("Internet"), ferris.location);
    assert_eq!(vec!["mascot", "rust"], ferris.groups);
}
```

A merge strategy is a function with the signature
`fn merge<T>(left: &mut T, right: T)` that merges `right` into `left`. The
`conflate` crate provides strategies for the most common types, but you can also
define your own strategies.

The trait can be used to merge configuration from different sources, for example
environment variables, multiple configuration files and command-line arguments,
see the `args.rs` example.

## Features

This crate has the following features:

- `derive` (default): Enables the derive macro for the `Merge` trait using the
  `conflate_derive` crate.
- `num` (default): Enables the merge strategies in the `num` module that require
  the `num_traits` crate.
- `std` (default): Enables the merge strategies in the `hashmap` and `vec`
  modules that require the standard library. If this feature is not set,
  `conflate` is a `no_std` library.

## Minimum Rust version policy

This crate's minimum supported `rustc` version is `1.64.0`.

The current policy is that the minimum Rust version required to use this crate
can be increased in minor version updates. For example, if `crate 1.0` requires
Rust 1.20.0, then `crate 1.0.z` for all values of `z` will also require Rust
1.20.0 or newer. However, `crate 1.y` for `y > 0` may require a newer minimum
version of Rust.

In general, this crate will be conservative with respect to the minimum
supported version of Rust.

## License

This project is dual-licensed under the [Apache-2.0][Apache-2.0] and [MIT][MIT]
licenses. The documentation and configuration files contained in this repository
are licensed under the [Creative Commons Zero][CC0] license. You can find a copy
of the license texts in the `LICENSES` directory.

`conflate` complies with [version 3.0 of the REUSE specification][reuse].

## Credits

This project is based on the awesome [merge](https://crates.io/crates/merge)
crate by Robin Krahl.

[Apache-2.0]: https://opensource.org/licenses/Apache-2.0
[MIT]: https://opensource.org/licenses/MIT
[CC0]: https://creativecommons.org/publicdomain/zero/1.0/
[reuse]: https://reuse.software/practices/3.0/
