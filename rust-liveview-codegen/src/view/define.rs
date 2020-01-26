use crate::helpers::{
    Deref,
    IntoIterator,
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::*;
use quote::quote;
use std::iter;
use syn::{
    braced,
    parse::{
        Parse,
        ParseStream,
        Result,
    },
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute,
    Expr,
    ExprPath,
    ExprType,
    Ident,
    Token,
    Type,
};

#[derive(Debug)]
pub enum ElementDefinition {
    Unit {
        attrs: Vec<Attribute>,
        ident: Ident,
    },
    Struct {
        attrs: Vec<Attribute>,
        ident: Ident,
        fields: Punctuated<ExprType, Token![,]>,
    },
}

thread_local! {
    static GLOBAL_ATTRS: Punctuated<ExprType, Token![,]> = {
        let def = syn::parse_str::<TokenStream2>(
            "GlobalAttributes {
                    accesskey: attributes::SpacedSet<String>,
                    class: attributes::SpacedSet<attributes::Class>, 
                    id: attributes::Id, 
                    lang: attributes::LanguageTag,
                    title: String
                }",
        )
        .and_then(syn::parse2::<ElementDefinition>)
        .expect_or_abort("Failed to parse global attributes definitions.");

        match def {
          ElementDefinition::Struct{fields, ..} => fields,
          ElementDefinition::Unit {ident, ..} =>
            abort!(ident.span(), "Invalid global attributes definitions.")
        }
    }
}

fn global_attributes<'a>() -> Box<dyn Iterator<Item = ExprType> + 'a> {
    box GLOBAL_ATTRS.with(|attrs| attrs.clone().into_iter())
}

#[derive(IntoIterator, Deref, Debug)]
pub struct ElementDefinitions(Punctuated<ElementDefinition, Token![,]>);
type ElementDefinitionField = (Vec<Attribute>, Ident, Box<Type>);

impl ElementDefinition {
    fn attrs(&self) -> &Vec<Attribute> {
        match self {
            Self::Unit { attrs, .. } => attrs,
            Self::Struct { attrs, .. } => attrs,
        }
    }

    fn ident(&self) -> &Ident {
        match self {
            Self::Unit { ident, .. } => ident,
            Self::Struct { ident, .. } => ident,
        }
    }

    fn own_fields<'a>(&'a self) -> Box<dyn Iterator<Item = ExprType> + 'a> {
        match self {
            Self::Struct { fields, .. } => box fields.clone().into_iter(),
            _ => box iter::empty::<ExprType>(),
        }
    }

    fn fields<'a>(&'a self) -> Box<dyn Iterator<Item = ElementDefinitionField> + 'a> {
        let iter = global_attributes().chain(self.own_fields());
        box iter.map(|field| {
            let (attrs, name) = match *field.expr {
                Expr::Path(ExprPath { path, qself, attrs })
                    if qself.is_none() && path.segments.len() == 1 =>
                {
                    (attrs, path.segments[0].ident.clone())
                }
                e => abort!(e.span(), "Invalid field name."),
            };
            let ty = field.ty;

            (attrs, name, ty)
        })
    }
    fn generate_struct_fields<'a>(&'a self) -> Box<dyn Iterator<Item = TokenStream2> + 'a> {
        box self.fields().map(|item| {
            let (attrs, name, ty) = item;
            quote! {
                #(#attrs)*
                #name: Option<#ty>,
            }
        })
    }

    fn generate_struct(&self) -> TokenStream2 {
        let attrs = self.attrs();
        let ident = self.ident();
        let fields = self.generate_struct_fields();
        let doc = format!(" {} element.", ident);
        quote! {
            #[doc = #doc]
            #[derive(Element, Clone, Default, Debug)]
            #(#attrs)*
            pub struct #ident<C> {
                #(#fields)*
                _c: std::marker::PhantomData<C>
            }
        }
    }
}

impl From<ElementDefinition> for TokenStream2 {
    #[inline]
    fn from(definition: ElementDefinition) -> Self {
        let type_definition = definition.generate_struct();
        quote! {
            #type_definition
        }
    }
}

impl From<ElementDefinition> for TokenStream {
    #[inline]
    fn from(definition: ElementDefinition) -> Self {
        Self::from(TokenStream2::from(definition))
    }
}
impl From<ElementDefinitions> for TokenStream2 {
    #[inline]
    fn from(items: ElementDefinitions) -> Self {
        #[allow(clippy::redundant_closure)]
        let streams = items.into_iter().map(|item| TokenStream2::from(item));
        quote! {
           #(#streams)*
        }
    }
}

impl From<ElementDefinitions> for TokenStream {
    #[inline]
    fn from(definition: ElementDefinitions) -> Self {
        Self::from(TokenStream2::from(definition))
    }
}

impl Parse for ElementDefinition {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let ident = input.parse()?;
        if input.peek(Token![,]) {
            Ok(Self::Unit { attrs, ident })
        } else {
            let content;
            let _ = braced!(content in input);
            let fields = content.parse_terminated(ExprType::parse)?;
            Ok(Self::Struct {
                attrs,
                ident,
                fields,
            })
        }
    }
}

impl Parse for ElementDefinitions {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let definitions = input.parse_terminated(ElementDefinition::parse)?;
        Ok(Self(definitions))
    }
}
