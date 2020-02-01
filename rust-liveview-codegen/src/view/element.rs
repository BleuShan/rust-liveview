use crate::helpers::{
    DarlingResultExt,
    Inflector,
};
use darling::{
    ast::Data,
    FromDeriveInput,
    FromField,
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::*;
use quote::{
    quote,
    ToTokens,
};
use std::str::pattern::Pattern;
use syn::{
    parse::{
        Parse,
        ParseStream,
        Result,
    },
    Attribute,
    DeriveInput,
    Ident,
    Type,
    Visibility,
};

#[derive(Debug, FromField)]
#[darling(attributes(element), forward_attrs)]
struct ElementField {
    ident: Option<Ident>,
    attrs: Vec<Attribute>,
    vis: Visibility,
    ty: Type,
    #[darling(default, rename = "skip")]
    skipped: bool,
}

impl ElementField {
    fn ident(&self) -> &Ident {
        self.ident
            .as_ref()
            .expect_or_abort("Anonymous fields are unsupported")
    }

    fn attribute_name(&self) -> String {
        self.ident().to_string().to_kebab_case()
    }

    fn is_option(&self) -> bool {
        if let Type::Path(ref typepath) = self.ty {
            if typepath.qself.is_none() {
                let mut segments = typepath.path.segments.iter();
                return segments.any(|segment| segment.ident == "Option");
            }
        }
        false
    }

    fn is_phantom(&self) -> bool {
        if let Type::Path(ref typepath) = self.ty {
            if typepath.qself.is_none() {
                let mut segments = typepath.path.segments.iter();
                return segments.any(|segment| {
                    segment.ident == "PhantomData" || segment.ident == "PhantomPinned"
                });
            }
        }
        false
    }

    fn is_skipped(&self) -> bool {
        self.skipped || "_".to_owned().is_prefix_of(&self.ident().to_string()) || self.is_phantom()
    }
}

impl ToTokens for ElementField {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = self.attribute_name();
        let ident = self.ident();
        let new_tokens = if self.is_option() {
            quote! {
                if let Some(ref #ident) = self.#ident {
                    attributes.push((#name, #ident.to_string()));
                }
            }
        } else {
            quote! {
                attributes.push((#name, self.#ident.to_string()));
            }
        };
        tokens.extend(new_tokens)
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(element),
    supports(struct_named, struct_unit),
    forward_attrs
)]
pub struct Element {
    ident: Ident,
    attrs: Vec<Attribute>,
    data: Data<(), ElementField>,
    #[darling(default)]
    self_closing: bool,
}

impl Element {
    fn fields<'a>(&'a self) -> Box<dyn Iterator<Item = &ElementField> + 'a> {
        if let Some(fields) = self.data.as_ref().take_struct() {
            box fields
                .fields
                .into_iter()
                .filter(|field| !field.is_skipped())
        } else {
            box std::iter::empty::<&ElementField>()
        }
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = &self.ident;
        let node_name = ident.to_string().to_kebab_case();
        let fields = self.fields();

        let render_impl = if self.self_closing {
            quote!(r.element_void(#node_name, self.attributes()))
        } else {
            quote! {
                r.element_open(#node_name, self.attributes())?;
                r.element_close()
            }
        };

        tokens.extend(quote! {
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<C> Render<C> for #ident<C> where C: RenderContext {
                fn render(&self, r: &mut Renderer<'_, C>) -> Result<()> {
                    #render_impl
                }
            }

            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<C> Element<C> for #ident<C> where C: RenderContext {
                fn attributes(&self) -> Vec<(&'static str, String)> {
                    let mut attributes = Vec::default();
                    #(#fields)*
                    attributes
                }
            }
        })
    }
}

impl Parse for Element {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let item: DeriveInput = input.parse()?;
        Ok(Self::from_derive_input(&item)
            .expect_or_abort("An error occured while parsing the input."))
    }
}

impl From<Element> for TokenStream {
    fn from(element: Element) -> TokenStream {
        TokenStream::from(quote! {
            #element
        })
    }
}
