// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

use conflate::Merge;

#[derive(Merge)]
struct S {
    #[merge(strategy = my_custom_merge_strategy)]
    field1: u8,
}

fn main() {}
