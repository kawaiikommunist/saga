use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{ItemFn, parse};

#[proc_macro]
pub fn branch(item: TokenStream) -> TokenStream {
    let ret = quote! {};
    ret.into()
}
