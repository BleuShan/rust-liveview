use crate::helpers::DarlingResultExt;
use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro_error::*;
use quote::quote;
use syn::{
    AttributeArgs,
    Error,
    ItemFn,
    Path,
};

#[inline]
pub(crate) fn has_attr<I>(attrs: &[syn::Attribute], ident: I) -> bool
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

#[derive(Debug, FromMeta)]
pub(crate) struct Attribute {
    executor_entrypoint: Path,
    #[darling(default)]
    test: bool,
}

impl From<AttributeArgs> for Attribute {
    fn from(attr_args: AttributeArgs) -> Self {
        Self::from_list(&attr_args).expect_or_abort("An error occured while parsing the input.")
    }
}

impl Attribute {
    pub(crate) fn generate(self, mut item: ItemFn) -> TokenStream {
        let sig = &mut item.sig;

        if sig.asyncness.is_none() {
            abort!(Error::new_spanned(
                sig.fn_token,
                "Only async functions are supported"
            ));
        }

        let vis = &item.vis;
        let attrs = &item.attrs;
        let body = &item.block;
        let entrypoint = &self.executor_entrypoint;

        sig.asyncness = None;

        let attributes = if self.test && !has_attr(attrs, "test") {
            quote! {
                #[test]
                #(#attrs)*
            }
        } else {
            quote!(#(#attrs)*)
        };
        TokenStream::from(quote! {
            #attributes
            #vis #sig {
                #entrypoint(async move {
                    #body
                })
            }
        })
    }
}
