mod micro_ent;
mod structure_ent;

use micro_ent::micro_ent_impl;
use proc_macro::TokenStream;
use structure_ent::structure_ent_impl;

#[proc_macro_attribute]
pub fn micro_ent(attr: TokenStream, item: TokenStream) -> TokenStream {
  micro_ent_impl(attr.into(), item.into())
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}

#[proc_macro_attribute]
pub fn structure_ent(attr: TokenStream, item: TokenStream) -> TokenStream {
  structure_ent_impl(attr.into(), item.into())
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}
