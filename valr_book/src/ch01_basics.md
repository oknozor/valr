# Chapter 1


## Raw derive

```rust
extern crate proc_macro;
use proc_macro::TokenStream;


#[proc_macro_derive(Validator)]
pub fn derive_validator(_item: TokenStream) -> TokenStream {
    "impl Struct<'_> {
        fn validate(&self) -> bool { true }
    }".parse().unwrap()
}
```

```rust
#[derive(Validator)]
struct Struct<'a> {
    name: &'a str
}

fn main() {
    let bob = Person {
        name: "Bob"
    };

    let valid = bob.validate();
    println!("{}", valid);
}
```

## Syn derive

```rust
#[proc_macro_derive(Validator)]
pub fn derive_validator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    println!("{:?}", &ident);

    format!("impl {} <'_> {{
        fn validate(&self) -> bool {{ true }}
    }}", ident).as_str().parse().unwrap()
}
```

## Syn + Quote derive

```rust
#[proc_macro_derive(Validator)]
pub fn derive_validator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    println!("{:?}", &ident);

    let tokens = quote!(
        impl #ident <'_> {
            fn validate(&self) -> bool { true }
        }
    );

    proc_macro::TokenStream::from(tokens)
}
```