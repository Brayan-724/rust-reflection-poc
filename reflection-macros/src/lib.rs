use quote::quote;
use syn::DeriveInput;

extern crate proc_macro as pm;

#[proc_macro_derive(CompTimeReflected)]
pub fn comp_time_reflection(item: pm::TokenStream) -> pm::TokenStream {
    let item = syn::parse_macro_input!(item as DeriveInput);
    let item_ident = &item.ident;

    quote! {}.into()
}
