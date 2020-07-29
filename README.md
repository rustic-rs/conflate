<!---
Copyright (C) 2020 Robin Krahl <robin.krahl@ireas.org>
SPDX-License-Identifier: CC0-1.0
-->

# merge-rs

The `merge` crate provides the `Merge` trait that can be used to merge multiple
values into one:

```rust
trait Merge {
    fn merge(&mut self, other: &mut Self);
}
```

`Merge` is implemented for `Option` and can be derived for structs:

<!-- should be kept in sync with examples/user.rs -->
```rust
use merge::Merge;

#[derive(Merge)]
struct User {
    // Fields with the ignore attribute are skipped by Merge
    #[merge(ignore)]
    pub name: &'static str,

    // The Merge implementation for Option replaces its value if it is None
    pub location: Option<&'static str>,

    // The strategy attribute is used to customize the merge behavior
    #[merge(strategy = Vec::append)]
    pub groups: Vec<&'static str>,
}

let mut defaults = User {
    name: "",
    location: Some("Internet"),
    groups: vec!["rust"],
};
let mut ferris = User {
    name: "Ferris",
    location: None,
    groups: vec!["mascot"],
};
ferris.merge(&mut defaults);

assert_eq!("Ferris", ferris.name);
assert_eq!(Some("Internet"), ferris.location);
assert_eq!(vec!["mascot", "rust"], ferris.groups);
```

The trait can be used to merge configuration from different sources, for
example environment variables, multiple configuration files and command-line
arguments.

## Minimum Supported Rust Version

This crate supports Rust 1.36.0 or later.

## License

This project is dual-licensed under the [Apache-2.0][] and [MIT][] licenses.
The documentation and configuration files contained in this repository are
licensed under the [Creative Commons Zero][CC0] license.  You can find a copy
of the license texts in the `LICENSES` directory.

`merge-rs` complies with [version 3.0 of the REUSE specification][reuse].

[Apache-2.0]: https://opensource.org/licenses/Apache-2.0
[MIT]: https://opensource.org/licenses/MIT
[CC0]: https://creativecommons.org/publicdomain/zero/1.0/
[reuse]: https://reuse.software/practices/3.0/
