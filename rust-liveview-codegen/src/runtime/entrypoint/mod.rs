use crate::helpers::{
    new,
    DarlingResultExt,
};
use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::*;
use quote::{
    quote,
    ToTokens,
};
use syn::{
    parse::{
        Parse,
        ParseStream,
        Result,
    },
    punctuated::{
        Pair,
        Punctuated,
    },
    spanned::Spanned,
    AttributeArgs,
    ItemFn,
    NestedMeta,
    Token,
};

mod main;
mod test;

pub(crate) use main::{
    MainEntryPoint,
    MainEntryPointArgs,
};
pub(crate) use test::{
    TestEntryPoint,
    TestEntryPointArgs,
};

#[inline]
pub(crate) fn set_fn_dummy(item: &ItemFn) {
    let sig = &item.sig;
    let attrs = &item.attrs;
    let ident = &sig.ident;
    let inputs = &sig.inputs;
    let vis = &item.vis;
    set_dummy(quote! {
        #(#attrs)*
        #vis fn #ident(#inputs) {}
    });
}

#[derive(Debug, Clone, Copy, FromMeta)]
pub(crate) enum Executor {
    AsyncStd,
    Tokio,
}
