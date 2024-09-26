// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: CC0-1.0

//! This example demonstrates how to merge configuration from different sources using the `conflate`
//! crate.  The example defines a struct `User` with three fields: `name`, `location` and `groups`.  The
//! `name` field is skipped during the merge, the `location` field is overwritten if it is `None` and the
//! `groups` field is appended.  The example then merges a default configuration into a user configuration
//! and asserts that the merged configuration is correct.

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
