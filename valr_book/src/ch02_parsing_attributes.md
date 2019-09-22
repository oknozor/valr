# Chapter 2

## Derive macro + attribute
Here we don't care about the TokenStream output, we just ant to understand the ins and outs of the syn ast :
```rust
#[derive(Validator, ValidAttr)]
struct Person<'a> {
    first_name: &'a str,
    #[valid(name)]
    last_name: &'a str,
}

fn main() {
    let bob = Person {
        first_name: "Bob",
        last_name: "Martin",
    };

    let valid = bob.validate();
    println!("{}", valid);
    println!("{}", bob.first_name);
    println!("{}", bob.last_name);
}
```

```rust
#[proc_macro_derive(ValidAttr, attributes(valid))]
pub fn derive_valid_attr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                fields
                    .named
                    .pairs()
                    .map(|pair| pair.into_value())
                    .for_each(|field| print_attrs(&field.attrs));
            }
            Fields::Unnamed(_fields) => {}
            Fields::Unit => {}
        },
        Data::Enum(_data_enum) => {}
        Data::Union(_data_union) => {}
    }

    TokenStream::new()
}

fn print_attrs(attrs: &Vec<Attribute>) {
    attrs.iter().for_each(|attr| {
        let meta = attr
            .parse_meta()
            .unwrap();
            print_meta(&meta);
    });

    attrs.iter().for_each(|attr| {
        let meta_list = match attr.parse_meta().unwrap() {
            Meta::List(meta_list) => meta_list,
            _ => panic!("expected a Meta::List",),
        };

        meta_list
            .nested
            .pairs()
            .map(|pair| pair.into_value())
            .for_each(|nested_meta| match nested_meta {
                NestedMeta::Meta(meta) => print_meta(&meta),
                NestedMeta::Lit(lit) => { panic!("Unexpected NestedMeta::Lit")}
            });
    });
}

fn print_meta(meta: &Meta) {
    meta.path()
            .segments
            .pairs()
            .map(|pair| pair.into_value())
            .for_each(|seg| println!("found meta attribute with segment name : {}", seg.ident));
}
```

## Attributes macro 
