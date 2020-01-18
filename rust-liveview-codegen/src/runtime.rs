use crate::util::has_attr;
use darling::FromMeta;
use proc_macro::{
    Span,
    TokenStream,
};
use proc_macro_error::*;
use quote::quote;
use syn::{
    AttributeArgs,
    Error,
    ItemFn,
    Path,
};

#[derive(Debug, FromMeta)]
pub struct Attribute {
    executor_entrypoint: Path,
    #[darling(default)]
    test: bool,
}

impl From<AttributeArgs> for Attribute {
    fn from(attr_args: AttributeArgs) -> Self {
        match Self::from_list(&attr_args) {
            Ok(attr) => attr,
            Err(e) => {
                e.emit();
                abort!(Span::call_site(), "Invalid arguments");
            }
        }
    }
}

impl Attribute {
    pub fn generate(self, mut item: ItemFn) -> TokenStream {
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
