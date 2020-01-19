use proc_macro_error::set_dummy;
use quote::quote;
use syn::{
    Attribute,
    ItemFn,
};

#[inline]
pub fn has_attr<I>(attrs: &[Attribute], ident: I) -> bool
where
    I: AsRef<str>,
{
    attrs.iter().any(|attr| attr.path.is_ident(&ident))
}

#[inline]
pub fn set_fn_dummy(item: &ItemFn) {
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
