// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: CC0-1.0

use merge::Merge;

#[derive(Merge)]
struct User {
    #[merge(skip)]
    pub name: &'static str,
    pub location: Option<&'static str>,
    #[merge(strategy = append)]
    pub groups: Vec<&'static str>,
}

fn append<T>(left: &mut Vec<T>, mut right: Vec<T>) {
    left.append(&mut right);
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
