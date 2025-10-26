extern crate proc_macro as pm;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;

#[proc_macro_derive(CompTimeReflected)]
pub fn comp_time_reflection(item: pm::TokenStream) -> pm::TokenStream {
    let item = syn::parse_macro_input!(item as syn::DeriveInput);
    let item_ident = &item.ident;

    let struct_ = match &item.data {
        syn::Data::Struct(struct_) => {
            let fields = struct_
                .fields
                .members()
                .map(|m| match m {
                    syn::Member::Named(i) => i,
                    _ => unreachable!(),
                })
                .enumerate()
                .map(|(i, f)| quote! {_ if ::reflection::str_equals(field, stringify!(#f)) => #i})
                .collect::<Vec<_>>();

            quote! {
                impl const ::reflection::StructDefine for #item_ident {
                    fn named_field(field: &'static str) -> usize {
                        match field {
                            #(#fields,)*
                            _ => panic!("Field doesn't exist")
                        }
                    }

                    fn named_field_checked(field: &'static str, message: &'static str) -> usize {
                        match field {
                            #(#fields,)*
                            _ => ::core::panicking::panic(message),
                        }
                    }
                }
            }
        }
        _ => quote!(),
    };

    let indexed = match &item.data {
        syn::Data::Struct(struct_) => {
            TokenStream::from_iter(struct_.fields.iter().enumerate().map(|(i, f)| {
                let ty = &f.ty;
                quote! {
                    impl ::reflection::GetFieldT<#i> for #item_ident {
                        type Type = #ty;
                    }
                }
            }))
        }
        syn::Data::Enum(_) => quote! {compile_error!("Enums not supported")},
        syn::Data::Union(_) => quote! {compile_error!("Unions not supported")},
    };

    quote! {
        #struct_
        #indexed
    }
    .into()
}

#[proc_macro]
pub fn reflect_trait(input: pm::TokenStream) -> pm::TokenStream {
    let input = syn::parse_macro_input!(input as syn::Ident);
    let impl_of = format_ident!("ImplOf{input}");

    quote! {
        trait #impl_of {
            const HAS: bool;
        }

        impl<T> #impl_of for T {
            default const HAS: bool = false;
        }

        impl<T: #input> #impl_of for T {
            const HAS: bool = true;
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn impl_of(_: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let item = syn::parse_macro_input!(input as syn::ItemTrait);
    let item_ident = &item.ident;
    let impl_of = format_ident!("ImplOf{item_ident}");

    let (def, decl) = item
        .items
        .iter()
        .map(|i| match i {
            syn::TraitItem::Fn(fn_) => {
                let ident = &fn_.sig.ident;
                let args = &fn_.sig.inputs;
                let ret = &fn_.sig.output;

                let def = quote! {fn #ident(#args) #ret};

                (def.clone(), Some((def, ident, args)))
            }
            _ => (
                quote_spanned! {i.span() => compile_error!("Only fn is supported in `impl_of`")},
                None,
            ),
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let (decl_default, decl_impl) = decl
        .into_iter()
        .filter_map(|v| v)
        .map(|(decl, fn_, args)| {
            (
                quote! {
                    default #decl {
                        panic!(
                            "{} has not implemented {}",
                            ::core::any::type_name::<T>(),
                            stringify!(#item_ident)
                        )
                    }
                },
                quote! {
                    #decl {
                        <T as #item_ident>::#fn_(#args)
                    }
                },
            )
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    quote! {
        #item

        trait #impl_of {
            const HAS: bool;

            #(#def;)*
        }

        impl<T> #impl_of for T {
            default const HAS: bool = false;

            #(#decl_default)*
        }

        impl<T: #item_ident> #impl_of for T {
            const HAS: bool = true;

            #(#decl_impl)*
        }
    }
    .into()
}
