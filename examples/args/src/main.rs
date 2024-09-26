// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: CC0-1.0

//! This example demonstrates how to merge configuration from different sources using the `conflate`
//! crate.  The example defines a struct `Args` with three fields: `debug`, `input` and `output`.  The
//! `debug` field is overwritten if it is `false`, the `input` and `output` fields are overwritten if they
//! are `None`.  The example then merges a default configuration into a user configuration and asserts that
//! the merged configuration is correct.

use conflate::Merge;
use serde_derive::Deserialize;
use structopt::StructOpt;

#[derive(Debug, Default, Deserialize, Merge, StructOpt)]
#[serde(default)]
struct Args {
    #[structopt(short, long)]
    #[merge(strategy = conflate::bool::overwrite_false)]
    debug: bool,

    #[merge(strategy = conflate::option::overwrite_none)]
    input: Option<String>,

    #[merge(strategy = conflate::option::overwrite_none)]
    output: Option<String>,
}

fn get_config() -> Option<Args> {
    let path: &std::path::Path = "args.toml".as_ref();
    if path.is_file() {
        let s = std::fs::read_to_string(path).expect("Could not read configuration file");
        Some(toml::from_str(&s).expect("Could not parse configuration"))
    } else {
        None
    }
}

fn get_env() -> Args {
    envy::prefixed("ARGS_")
        .from_env()
        .expect("Could not read environment variables")
}

fn main() {
    let mut args = Args::from_args();
    args.merge(get_env());
    if let Some(config) = get_config() {
        args.merge(config);
    }
    println!("{args:?}");
}
