extern crate valr;
use valr::Validator;


#[derive(Validator)]
struct Struct<'a> {
    name: &'a str
}

fn main() {
    let bob = Struct {
        name: "Bob"
    };

    let valid = bob.validate();
    println!("{}", valid);
}