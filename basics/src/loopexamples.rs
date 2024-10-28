#![allow(dead_code)]

pub fn loop_examples() {

    for i in 0..3 {
        println!("{}", i);
    }

    'my_loop: loop {
        println!("loop will break here");
        break 'my_loop;
    }

    let mut j = 3;

    while j >= 0 {
        println!("{}", j);
        j -= 1;
    }
}