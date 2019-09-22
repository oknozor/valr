# Chapter 3

```rust
struct ValidationBinding {
    struct_type: Ident,
    field_name: Ident,
    field_type: Type,
    validators: Vec<Validator>,
}

#[proc_macro_derive(ValidAttr, attributes(valid))]
pub fn derive_valid_attr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // get the identity of the struct or enum
    let identity = input.ident;
    println!("{}", identity.to_string());

    // retrieve a Vec<Field> from the syn DataStruct
    let fields = get_fields(input.data);


    // map each annotated field to a ValidationBinding
    let validators = fields
        .iter()
        .map(|field| ValidationBinding {
            struct_type: identity.clone(),
            field_name: field.ident.as_ref().unwrap().clone(),
            field_type: field.ty.clone(),
            validators: get_validators_ident(field),
        })
        .collect::<Vec<ValidationBinding>>();

    println!("found {} validators", validators.len());

    TokenStream::new()
}

fn get_fields(data: Data) -> Vec<Field> {
    match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => fields
                .named
                .pairs()
                .map(|pair| pair.into_value().clone())
                .collect::<Vec<Field>>(),
            Fields::Unnamed(_fields) => panic!("Unexpected unamed Fields::Unamed"),
            Fields::Unit => panic!("Unexpected Fields::Unit"),
        },
        Data::Enum(_data_enum) => panic!("Unexpected unamed Data::Union"),
        Data::Union(_data_union) => panic!("Unexpected unamed Data::Union"),
    }
}

fn get_validators_ident(field: &Field) -> Vec<Validator> {

    let mut meta_attributes = vec![];

    for attr in field.attrs.iter() {

        let meta_list = match attr.parse_meta().unwrap() {
            Meta::List(meta_list) => meta_list,
            _ => panic!("expected a Meta::List",),
        };

        let inner = meta_list
            .nested
            .pairs()
            .map(|pair| pair.into_value())
            .flat_map(|nested_meta| match nested_meta {
                NestedMeta::Meta(meta) => get_metas_ident(&meta),
                NestedMeta::Lit(lit) => panic!("Unexpected NestedMeta::Lit"),
            })
            .map(Validator::from)
            .collect::<Vec<Validator>>();

        meta_attributes.extend(inner);
    }

    meta_attributes.iter().for_each(|validator| println!("{:?}", validator));

    meta_attributes
}

fn get_metas_ident(meta: &Meta) -> Vec<Ident> {
    meta.path()
        .segments
        .pairs()
        .map(|pair| pair.into_value())
        .map(|seg| seg.ident.clone())
        .collect::<Vec<Ident>>()
}
```
