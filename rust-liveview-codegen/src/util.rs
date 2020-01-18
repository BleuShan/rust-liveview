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
    attrs.into_iter().any(|attr| attr.path.is_ident(&ident))
}

#[inline]
pub fn set_fn_dummy(item: &ItemFn) {
    let sig = &item.sig;
    let attrs = &item.attrs;
    let ident = &sig.ident;
    let has_test_attr = has_attr(attrs, "test");
    let dummy_output = if (ident == "main" || has_test_attr) && item.sig.asyncness.is_some() {
        quote! {
            #(#attrs)*
            fn #ident() {}
        }
    } else {
        quote! {
            #(#attrs)*
            #item
        }
    };

    set_dummy(dummy_output);
}
