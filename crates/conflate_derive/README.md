<!---
Copyright (C) 2020 Robin Krahl <robin.krahl@ireas.org>
SPDX-License-Identifier: CC0-1.0
-->

# conflate-derive

<p align="center">
<a href="https://crates.io/crates/conflate_derive"><img src="https://img.shields.io/crates/v/conflate_derive.svg" /></a>
<a href="https://docs.rs/conflate_derive/"><img src="https://img.shields.io/docsrs/conflate_derive?style=flat&amp;labelColor=1c1d42&amp;color=4f396a&amp;logo=Rust&amp;logoColor=white" /></a>
<a href="https://github.com/rustic-rs/conflate/"><img src="https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg" /></a>
<a href="https://crates.io/crates/conflate_derive"><img src="https://img.shields.io/crates/d/conflate_derive.svg" /></a>
<p>

This crate provides a derive macro for the `conflate::Merge` crate. See the
[`conflate`][`conflate`] crate for more information.

## Contact

You can ask questions in the
[Discussions](https://github.com/rustic-rs/rustic/discussions) or have a look at
the [FAQ](https://rustic.cli.rs/docs/FAQ.html).

| Contact       | Where?                                                                                                          |
| ------------- | --------------------------------------------------------------------------------------------------------------- |
| Issue Tracker | [GitHub Issues](https://github.com/rustic-rs/conflate/issues/choose)                                            |
| Discord       | [![Discord](https://dcbadge.vercel.app/api/server/WRUWENZnzQ?style=flat-square)](https://discord.gg/WRUWENZnzQ) |
| Discussions   | [GitHub Discussions](https://github.com/rustic-rs/rustic/discussions)                                           |

## Minimum Rust version policy

This crate's minimum supported `rustc` version is `1.64.0`.

The current policy is that the minimum Rust version required to use this crate
can be increased in minor version updates. For example, if `crate 1.0` requires
Rust 1.20.0, then `crate 1.0.z` for all values of `z` will also require Rust
1.20.0 or newer. However, `crate 1.y` for `y > 0` may require a newer minimum
version of Rust.

In general, this crate will be conservative with respect to the minimum
supported version of Rust.

[`conflate`]: https://crates.io/crates/conflate

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
