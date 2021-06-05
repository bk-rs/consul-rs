use std::collections::BTreeMap;

use syn::{
    parse::{Parse, ParseStream},
    Error as SynError, Ident, Token, Type,
};

#[derive(Default)]
pub struct PathParams(pub BTreeMap<Ident, Type>);

impl Parse for PathParams {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let mut inner = BTreeMap::new();

        loop {
            let path_param_name = input.parse::<Ident>()?;
            input.parse::<Token![=>]>()?;
            let path_param_type = input.parse::<Type>()?;

            if inner
                .insert(path_param_name.to_owned(), path_param_type)
                .is_some()
            {
                let err = format!("duplicate path param name: {}", &path_param_name);
                return Err(SynError::new_spanned(path_param_name, err));
            }

            input.parse::<Token![,]>()?;

            if !(input.peek(Ident) && input.peek2(Token![=>])) {
                break;
            }
        }

        Ok(Self(inner))
    }
}
