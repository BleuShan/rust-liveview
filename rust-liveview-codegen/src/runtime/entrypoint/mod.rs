use crate::helpers::new;
use proc_macro::TokenStream;
use proc_macro2::{
    Span,
    TokenStream as TokenStream2,
};
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
    punctuated::Pair,
    spanned::Spanned,
    Error,
    FnArg,
    ItemFn,
    Path,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum Executor {
    AsyncStd,
}

impl Parse for Executor {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let path: Path = input
            .parse()
            .map_err(|e| Error::new(e.span(), "Missing executor configuration."))?;
        if path.is_ident("async_std") {
            Ok(Self::AsyncStd)
        } else {
            Err(Error::new_spanned(path, "Unknown executor configuration."))
        }
    }
}

impl ToTokens for Executor {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(match self {
            Executor::AsyncStd => {
                quote! {
                    runtime::task::block_on
                }
            }
        });
    }
}
