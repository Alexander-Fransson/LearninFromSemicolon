#![allow(dead_code)]

pub fn struct_demonstration() {

    struct Firend{
        name: String,
        age: u32,
        similarities: Vec<String>
    }
    impl Firend {
        fn new(name: String, age: u32, similarities: Vec<String>) -> Firend {
            Firend {
                name,
                age,
                similarities
            }
        }

        fn greet(&self) {
            println!("Hello, my name is {} and I'm {} years old", self.name, self.age);
        }
    }

    let friend = Firend::new("Alice".to_string(), 30, vec!["a".to_string(), "b".to_string()]);

    friend.greet();
}