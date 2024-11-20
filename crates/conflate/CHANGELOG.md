# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.3](https://github.com/rustic-rs/conflate/compare/conflate-v0.3.2...conflate-v0.3.3) - 2024-11-20

### Added

- enhance MergeFrom trait with detailed documentation and add MergePrecedence trait for prioritized merging
- add MergeFrom trait to conflate ([#34](https://github.com/rustic-rs/conflate/pull/34))

## [0.3.2](https://github.com/rustic-rs/conflate/compare/conflate-v0.3.1...conflate-v0.3.2) - 2024-11-19

### Added

- add `option::overwrite_with_some` to option strategies ([#32](https://github.com/rustic-rs/conflate/pull/32))

## [0.3.1](https://github.com/rustic-rs/conflate/compare/conflate-v0.3.0...conflate-v0.3.1) - 2024-10-22

### Other

- *(deps)* update actions ([#26](https://github.com/rustic-rs/conflate/pull/26))
- add btreemap to std feature description ([#20](https://github.com/rustic-rs/conflate/pull/20))

## [0.3.0](https://github.com/rustic-rs/conflate/compare/conflate-v0.2.0...conflate-v0.3.0) - 2024-10-03

### Added

- *(strategy)* implement merge strategies for BTreeMap ([#13](https://github.com/rustic-rs/conflate/pull/13))

### Other

- *(strategy)* [**breaking**] Change function names for HashMap strategies ([#14](https://github.com/rustic-rs/conflate/pull/14))
<!---
SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
SPDX-License-Identifier: CC0-1.0
-->

# v0.1.0 (2020-09-01)

Initial release providing the `Merge` trait and some merge strategies in the
`bool`, `num`, `ord` and `vec` modules.
