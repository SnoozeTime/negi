extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

use syn::FnArg;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn task(_args: TokenStream, input: TokenStream) -> TokenStream {
    let fun: ItemFn = syn::parse_macro_input!(input as ItemFn);
    let ident = fun.ident;
    let block = fun.block;
    let args = (*fun.decl).inputs;

    let cloned: Vec<_> = args
        .iter()
        .map(|arg| match arg {
            FnArg::Captured(capt) => {
                let pat = capt.pat.clone();
                quote!(let #pat = self.#pat.clone();)
            }
            _ => panic!("Cannot capture that"),
        })
        .collect();

    let pub_fields: Vec<_> = args
        .iter()
        .map(|arg| match arg {
            FnArg::Captured(capt) => {
                let pat = capt.pat.clone();
                let ty = capt.ty.clone();
                quote!(pub #pat: #ty,)
            }
            _ => panic!("Cannot capture that"),
        })
        .collect();
    quote!(

        #[derive(Serialize, Deserialize)]
        pub struct #ident{#( #pub_fields )*}

        #[typetag::serde]
        impl Task for  #ident {
            fn execute(&self) {
                #( #cloned )*
                #block
            }
        }
    )
    .into()
}
