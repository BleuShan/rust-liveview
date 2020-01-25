use crate::helpers::{
    Deref,
    IntoIterator,
};
use proc_macro::{
    Span,
    TokenStream,
};
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::*;
use quote::quote;
use syn::{
    braced,
    parse::{
        Parse,
        ParseStream,
        Result,
    },
    punctuated::Punctuated,
    token::Brace,
    Expr,
    FieldValue,
    Ident,
    Member,
    Token,
};

#[derive(Debug)]
pub struct ElementDeclaration {
    ident: Ident,
    brace_token: Brace,
    fields: Punctuated<FieldValue, Token![,]>,
}

#[derive(IntoIterator, Deref, Debug)]
pub struct ElementDeclarations(Punctuated<ElementDeclaration, Token![,]>);

impl ElementDeclaration {
    fn generate_fields(&self) -> impl Iterator<Item = TokenStream2> {
        self.fields.clone().into_iter().map(|field| {
            let attrs = &field.attrs;
            let name = match &field.member {
                Member::Named(v) => v,
                Member::Unnamed(_) => {
                    abort!(Span::call_site(), "Anonymous members are not supported")
                }
            };

            let ty = match field.expr {
                Expr::Path(p) => p,
                _ => abort!(Span::call_site(), "Unsupported expression"),
            };
            quote! {
                #(#attrs)*
                #name: Option<#ty>,
            }
        })
    }
}

impl From<ElementDeclaration> for TokenStream2 {
    #[inline]
    fn from(declaration: ElementDeclaration) -> Self {
        let ident = &declaration.ident;
        let fields = declaration.generate_fields();
        let doc = format!(" {} element.", ident);
        quote! {
            #[derive(Debug, Element)]
            #[doc = #doc]
            pub struct #ident<C> {
                #(#fields)*
                _c: std::marker::PhantomData<C>
            }
        }
    }
}

impl From<ElementDeclaration> for TokenStream {
    #[inline]
    fn from(declaration: ElementDeclaration) -> Self {
        Self::from(TokenStream2::from(declaration))
    }
}
impl From<ElementDeclarations> for TokenStream2 {
    #[inline]
    fn from(items: ElementDeclarations) -> Self {
        #[allow(clippy::redundant_closure)]
        let streams = items.into_iter().map(|item| TokenStream2::from(item));
        quote! {
           #(#streams)*
        }
    }
}

impl From<ElementDeclarations> for TokenStream {
    #[inline]
    fn from(declaration: ElementDeclarations) -> Self {
        Self::from(TokenStream2::from(declaration))
    }
}

impl Parse for ElementDeclaration {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let ident = input.parse()?;
        let content;
        let brace_token = braced!(content in input);
        let fields = content.parse_terminated(FieldValue::parse)?;
        Ok(Self {
            ident,
            brace_token,
            fields,
        })
    }
}

impl Parse for ElementDeclarations {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let decls = input.parse_terminated(ElementDeclaration::parse)?;
        Ok(Self(decls))
    }
}
