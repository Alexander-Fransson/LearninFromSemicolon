#![allow(dead_code)]

struct Person<T: Car> {
    name: String,
    car: T
}

impl <T: Car> Person<T> {
    fn print_specifications(&self) {
        self.car.print_specifications();
    }
}

// traits are for types that should implement some functionality by a specific name
trait Car {
    fn print_specifications(&self);
}

struct Toyota {
    sentimental_value: u32,
}


struct Subaru {
    color: String
}

impl Car for Toyota {
    fn print_specifications(&self) {
        println!("Sentimental value: {}", self.sentimental_value);
    }
}

impl Car for Subaru {
    fn print_specifications(&self) {
        println!("Color: {}", self.color);
    }
}

pub fn demonstrate_generic() {
    let person = Person::<Toyota> { 
        name: "Alice".to_string(), 
        car: Toyota {sentimental_value: 5} 
    };
    person.print_specifications();
}