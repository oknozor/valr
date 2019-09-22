extern crate valr;
use valr::ValidAttr;
use valr::Validator;

#[derive(Validator, ValidAttr)]
struct Person<'a> {
    id: i32,

    #[valid(name)]
    name: &'a str,

    #[valid(email)]
    email: String,
}

fn main() {
    let bob = Person {
        id: 1,
        name: "Bob Martin",
        email: String::from("bob.martin@yopmail.org"),
    };

    let valid = bob.validate();
    println!("{}", valid);
    println!("{}", bob.name);
    println!("{}", bob.email);
}
