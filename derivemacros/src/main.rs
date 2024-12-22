pub use reflective_derive::GetName;

pub trait Reflective {
    fn name(&self) -> &'static str;
    fn fields(&self) -> Vec<&'static str>;
}

#[derive(GetName)]
#[allow(dead_code)]
struct Boo {
    pub a: i32,
    b: bool,
    c: String
}

fn main() {
    let foo = Boo { a: 1, b: true, c: "hello".to_string() };

    println!("{}", foo.name());
    println!("{:?}", foo.fields());
}
