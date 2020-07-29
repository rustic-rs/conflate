// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

pub use merge_derive::*;

pub trait Merge {
    fn merge(&mut self, other: &mut Self);
}

impl<T> Merge for Option<T> {
    fn merge(&mut self, other: &mut Self) {
        if !self.is_some() {
            *self = other.take();
        }
    }
}
