use syn::{
    LitStr,
    Token,
    TypePath,
    parse::{
        Parse,
        ParseStream,
    },
};

pub struct Args {
    pub api: TypePath,
    pub endpoint: LitStr,
    pub return_type: TypePath,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let api: TypePath = input.parse()?;
        let _: Token![:] = input.parse()?;
        let endpoint: LitStr = input.parse()?;
        let _: Token![->] = input.parse()?;
        let return_type: TypePath = input.parse()?;

        if !endpoint.value().starts_with('/') {
            return Err(syn::Error::new_spanned(
                endpoint,
                "endpoint path must start with a `/`",
            ));
        }

        Ok(Self {
            api,
            endpoint,
            return_type,
        })
    }
}
