#![allow(dead_code)]

use std::fs::File;
use std::io::Read;
use std::io::Write;// this is required to write to a file

pub fn read_from_file() {
    let mut f = File::open("../basics/data/atextfile.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    println!("{}", contents);
}

pub fn write_to_file() {
    let mut new_file = File::create("../basics/data/btextfile.txt").expect("Unable to create file");
    new_file.write_all("Hello, world!".as_bytes()).expect("Unable to write data");
}