extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{FnArg, Ident, ItemFn};

mod parse;

#[proc_macro_attribute]
pub fn task(macro_args: TokenStream, input: TokenStream) -> TokenStream {
    let fun: ItemFn = syn::parse_macro_input!(input as ItemFn);
    let ident = fun.ident;
    let block = fun.block;
    let args = (*fun.decl).inputs;
    let macro_args = syn::parse_macro_input!(macro_args as parse::MacroArgs);
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

    let ctor_arg: Vec<_> = args
        .iter()
        .map(|arg| match arg {
            FnArg::Captured(capt) => {
                let pat = capt.pat.clone();
                quote!(#pat)
            }
            _ => panic!("Cannot capture that"),
        })
        .collect();
    let mod_root = if let parse::MacroArgs::Custom(root) = macro_args {
        root
    } else {
        Ident::new("negi", Span::call_site())
    };

    let module_ident = Ident::new(&format!("{}_mod", ident), Span::call_site());
    quote!(

        pub use #module_ident::#ident;
        pub mod #module_ident {

            use serde_derive::{Serialize, Deserialize};
            use #mod_root::Task;

            #[derive(Serialize, Deserialize)]
            pub struct #ident{#( #pub_fields )*}

            #[typetag::serde]
            impl Task for  #ident {
                fn execute(&self) {
                    #( #cloned )*
                    #block
                }
            }


            impl #ident {

                pub fn new(#args) -> Self {
                    Self {
                        #( #ctor_arg, )*
                    }
                }
            }
        }
    )
    .into()
}
