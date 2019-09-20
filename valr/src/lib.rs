extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(Validator)]
pub fn derive_validator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    println!("{:?}", &ident);

    format!("impl {} <'_> {{
        fn validate(&self) -> bool {{ true }}
    }}", ident).as_str().parse().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
