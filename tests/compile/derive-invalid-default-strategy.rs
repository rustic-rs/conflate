// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

use merge::Merge;

#[derive(Merge)]
#[merge(strategy = my_custom_merge_strategy)]
struct S {
    field1: u16,
}

fn my_custom_merge_strategy(left: &mut u8, right: u8) {
    *left += right
}

fn main() {}

