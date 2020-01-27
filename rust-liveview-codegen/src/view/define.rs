use crate::helpers::{
    once_cell::unsync::Lazy,
    Deref,
    IntoIterator,
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::*;
use quote::{
    quote,
    ToTokens,
};
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

#[allow(clippy::declare_interior_mutable_const)]
const GLOBAL_ATTRS: Lazy<Punctuated<ExprType, Token![,]>> = Lazy::new(|| {
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
        ElementDefinition::Struct { fields, .. } => fields,
        ElementDefinition::Unit { ident, .. } => {
            abort!(ident.span(), "Invalid global attributes definitions.")
        }
    }
});

fn global_attributes<'a>() -> Box<dyn Iterator<Item = ExprType> + 'a> {
    let fields = GLOBAL_ATTRS;
    box (*fields).clone().into_iter()
}

#[derive(Debug)]
struct ElementDefinitionField(Vec<Attribute>, Ident, Box<Type>);

impl ToTokens for ElementDefinitionField {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ElementDefinitionField(ref attrs, ref name, ref ty) = *self;
        tokens.extend(quote! {
            #(#attrs)*
            #name: Option<#ty>,
        })
    }
}

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

            ElementDefinitionField(attrs, name, ty)
        })
    }
}

impl ToTokens for ElementDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let attrs = self.attrs();
        let ident = self.ident();
        let fields = self.fields();
        let doc = format!(" {} element.", ident);
        tokens.extend(quote! {
            #[doc = #doc]
            #[derive(Element, Clone, Default, Debug)]
            #(#attrs)*
            pub struct #ident<C> {
                #(#fields)*
                _c: std::marker::PhantomData<C>
            }
        })
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

impl From<ElementDefinition> for TokenStream {
    fn from(definition: ElementDefinition) -> Self {
        Self::from(quote! {
            #definition
        })
    }
}

#[derive(IntoIterator, Deref, Debug)]
pub struct ElementDefinitions(Punctuated<ElementDefinition, Token![,]>);

impl Parse for ElementDefinitions {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let definitions = input.parse_terminated(ElementDefinition::parse)?;
        Ok(Self(definitions))
    }
}

impl ToTokens for ElementDefinitions {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let definitions = self.iter();
        tokens.extend(quote! {
            #(#definitions)*
        })
    }
}

impl From<ElementDefinitions> for TokenStream {
    fn from(definitions: ElementDefinitions) -> Self {
        Self::from(quote! {
            #definitions
        })
    }
}
