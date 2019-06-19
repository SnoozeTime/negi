use syn::parse::{Parse, ParseStream, Result};
use syn::Ident;

mod kw {
    syn::custom_keyword!(module);
}

/// Args will determine where the `Task` trait is located.
/// Outside negi package, it is is negi crate but to use inside,
/// it would be negi_core
pub enum MacroArgs {
    Normal,
    Custom(Ident),
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(MacroArgs::Normal);
        }

        input.parse::<kw::module>()?;
        let ident = Ident::new("negi_core", proc_macro2::Span::call_site());
        Ok(MacroArgs::Custom(ident))
    }
}
