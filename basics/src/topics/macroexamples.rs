// macros are like functions but realy they are stadinns for code replaced at compile time
// macros can be created using the macro_rules! macro

#![allow(dead_code)]

macro_rules! add {
    ($a: expr, $b: expr) => {
        $a + $b
    }
}

macro_rules! sneeze {
    () => {
        println!("sneeze!");
    };
}

pub fn macro_example() {
    println!("{}", add!(1, 2));
    sneeze!();
}

pub fn debug_with_dbg() {
    let sum = dbg!(add!(1, 2)); // will print the value and the line, useful for debugging instead of printing console logging with println
    println!("{}", sum);
}

/*
    why use macros?

    In Rust, you would use a macro instead of a function in the following situations:

    Meta-programming: Macros allow you to write code that generates code at compile-time. This is useful when you need to perform complex transformations or generate boilerplate code.
    Compile-time evaluation: Macros can evaluate expressions at compile-time, which can be useful for things like constant folding or optimizing away unnecessary computations.
    Syntax extension: Macros can extend the Rust language itself, allowing you to create new syntax or modify existing syntax to better fit your needs.
    Zero-cost abstractions: Macros can create abstractions that have zero runtime cost, since the macro expansion happens at compile-time.
    Type manipulation: Macros can manipulate types in ways that are not possible with functions, such as creating new types or modifying existing types.
    In general, if you need to perform some kind of code transformation or generation, or if you need to extend the language itself, a macro is likely a better choice than a function.

    However, if you just need to perform some kind of computation or operation at runtime, a function is usually a better choice.

    It's worth noting that Rust's macro system is very powerful, but it's also complex and can be difficult to use. If you're new to Rust, it's usually recommended to start with functions 
    and only move to macros when you have a specific need for them

*/