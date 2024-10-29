// Enums are custom types with associated data
#![allow(dead_code)]

enum Fruit {
    Apple,
    Banana,
    Pear
}

pub fn use_fruit() {
    let current_fruit = Fruit::Apple;

    match current_fruit {
        Fruit::Apple => println!("Apple"),
        Fruit::Banana => println!("Banana"),
        Fruit::Pear => println!("Pear")
    }
}

// My example

enum Firends {
    Alice,
    Bob,
    Charlie(u32) // you can force a match to pick a specific value
}

pub fn use_firends() {
    let current_firend = Firends::Alice;

    match current_firend {
        Firends::Alice => println!("Alice"),
        Firends::Bob => println!("Bob"),
        Firends::Charlie(value) => println!("Charlie {}", value)
    }
}
