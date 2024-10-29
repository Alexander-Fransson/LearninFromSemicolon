#![allow(dead_code)]

use std::fs::File;
use std::io::Read;

pub fn read_from_file() {
    let mut f = File::open("../basics/data/atextfile.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    println!("{}", contents);
}