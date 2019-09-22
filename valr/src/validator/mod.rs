use syn::Ident;

#[derive(Debug)]
pub enum Validator {
    Email,
    Unknown,
}

impl From<String> for Validator {
    fn from(input: String) -> Self {
        Validator::from(input.as_str())
    }
}

impl From<&str> for Validator {
    fn from(input: &str) -> Self {
        match input {
            "email" => Validator::Email,
            _ => Validator::Unknown,
        }
    }
}

impl From<Ident> for Validator {
    fn from(input: Ident) -> Self {
        Validator::from(input.to_string())
    }
}