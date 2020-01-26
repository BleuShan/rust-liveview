use proc_macro2::Span;
use proc_macro_error::*;
use quote::quote;
pub(crate) use rust_liveview_common::*;
use syn::{
    Attribute,
    ItemFn,
};

#[inline]
pub(crate) fn has_attr<I>(attrs: &[Attribute], ident: I) -> bool
where
    I: AsRef<str>,
{
    attrs.iter().any(|attr| attr.path.is_ident(&ident))
}

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

pub(crate) trait DarlingResultExt {
    type Ok;

    fn unwrap_or_abort(self) -> Self::Ok;
    fn expect_or_abort<S>(self, message: S) -> Self::Ok
    where
        S: AsRef<str>;
}

impl<T> DarlingResultExt for Result<T, darling::Error> {
    type Ok = T;
    fn unwrap_or_abort(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                e.emit();
                abort!(Span::call_site(), "An error occured.")
            }
        }
    }

    fn expect_or_abort<S>(self, message: S) -> T
    where
        S: AsRef<str>,
    {
        match self {
            Ok(v) => v,
            Err(e) => {
                e.emit();
                abort!(Span::call_site(), message.as_ref())
            }
        }
    }
}
