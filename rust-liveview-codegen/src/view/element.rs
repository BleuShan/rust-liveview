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
use quote::quote;
use std::str::pattern::Pattern;
use syn::{
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
        self.ident().to_string().to_kebab_case().to_lowercase()
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
                return segments.any(|segment| segment.ident == "PhantomData");
            }
        }
        false
    }

    fn is_skipped(&self) -> bool {
        self.skipped || "_".to_owned().is_prefix_of(&self.ident().to_string()) || self.is_phantom()
    }

    fn generate_attribute_render_impl(&self) -> TokenStream2 {
        let name = self.attribute_name();
        let ident = self.ident();
        if self.is_option() {
            quote! {
                if let Some(ref #ident) = self.#ident {
                    r.attribute(#name, #ident.to_string());
                }
            }
        } else {
            quote! {
                r.attribute(#name, self.#ident.to_string());
            }
        }
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
    vis: Visibility,
    data: Data<(), ElementField>,
    #[darling(default)]
    self_closing: bool,
}

impl From<DeriveInput> for Element {
    fn from(input: DeriveInput) -> Self {
        Self::from_derive_input(&input).expect_or_abort("An error occured while parsing the input.")
    }
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

    fn generate_node_impl(&self) -> TokenStream2 {
        let ident = &self.ident;
        let node_name = ident.to_string().to_kebab_case().to_lowercase();
        let attributes_impls = self
            .fields()
            .map(|field| field.generate_attribute_render_impl());

        let self_closing = self.self_closing;
        quote! {
            impl<T> Node<T> for #ident<T> where T: Send {
                fn node_name(&self) -> &'static str {
                    #node_name
                }

                fn render(&self, r: Box<&mut dyn Renderer<Output = T>>) {
                    r.start_node(#node_name);
                    #(#attributes_impls)*
                    r.end_node(#self_closing);
                }
            }
        }
    }

    fn generate_element_impl(&self) -> TokenStream2 {
        let ident = &self.ident;
        let fields = self.fields();
        let field_names = fields.map(|field| field.attribute_name());
        quote! {
            impl<T> Element for #ident<T> where T: Send {
                fn attribute_names(&self) -> &'static [&'static str] {
                    &[#(#field_names,)*]
                }
            }
        }
    }

    pub fn generate(self) -> TokenStream {
        let impls = vec![self.generate_node_impl(), self.generate_element_impl()];
        TokenStream::from(quote! {
           #(
                #[automatically_derived]
                #[allow(unused_qualifications)]
                #impls
           )*
        })
    }
}
