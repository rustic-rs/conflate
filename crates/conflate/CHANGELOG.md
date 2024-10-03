# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
