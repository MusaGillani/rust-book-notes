use std::fs::{self};
use std::io::{self};

fn _read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {}
