use syn::{
    parse::{Parse, ParseStream},
    Error as SynError, Ident, Token,
};

#[derive(Default)]
pub struct QueryOptionNames(pub Vec<Ident>);

impl Parse for QueryOptionNames {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let mut inner = vec![];

        loop {
            let query_option_name = input.parse::<Ident>()?;

            if inner.contains(&query_option_name) {
                let err = format!("duplicate query option name: {}", &query_option_name);
                return Err(SynError::new_spanned(query_option_name, err));
            }
            inner.push(query_option_name);

            input.parse::<Token![,]>()?;

            if !input.peek(Ident) {
                break;
            }
        }

        Ok(Self(inner))
    }
}
