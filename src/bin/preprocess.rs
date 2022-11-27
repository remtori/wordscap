use itertools::Itertools;
use std::{
    fs::File,
    io::{Read, Write},
};

#[allow(unstable_name_collisions)]
fn main() {
    let words = &mut String::new();
    File::open("./words.txt")
        .unwrap()
        .read_to_string(words)
        .unwrap();

    let out: String = words
        .split('\n')
        .map(|word| word.trim().to_ascii_lowercase())
        .filter(|word| word.len() >= 3)
        .intersperse("\n".to_owned())
        .collect();

    File::options()
        .write(true)
        .create(true)
        .open("./processed.txt")
        .unwrap()
        .write_all(out.as_bytes())
        .unwrap();
}
