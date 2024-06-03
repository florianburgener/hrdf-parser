use std::{
    collections::BTreeSet,
    hash::{DefaultHasher, Hash, Hasher},
    process,
};

fn main() {
    if let Err(e) = hrdf::run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
