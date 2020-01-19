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

    fn generate_attribute_display_impl(&self) -> TokenStream2 {
        let name = self.attribute_name();
        let ident = self.ident();
        if self.is_option() {
            quote! {
                if let Some(ref #ident) = self.#ident {
                    write!(f, " {}=\"{}\"", #name, #ident)?;
                }
            }
        } else {
            quote! {
                write!(f, " {}=\"{}\"", #name, self.#ident)?;
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
            box fields.fields.into_iter().filter(|field| !field.skipped)
        } else {
            box std::iter::empty::<&ElementField>()
        }
    }

    fn generate_node_impl(&self) -> TokenStream2 {
        let ident = &self.ident;
        let node_name = ident.to_string().to_kebab_case().to_lowercase();
        quote! {
            impl Node for #ident {
                const NODE_NAME: &'static str = #node_name;
            }
        }
    }

    fn generate_element_impl(&self) -> TokenStream2 {
        let ident = &self.ident;
        let fields = self.fields();
        let field_names = fields.map(|field| field.attribute_name());
        quote! {
            impl Element for #ident {
                const ATTRIBUTE_NAMES: &'static [&'static str] = &[#(#field_names,)*];
            }
        }
    }

    fn generate_display_impl_closing_tag(&self) -> TokenStream2 {
        if self.self_closing {
            quote! {
                f.write_str("/>")
            }
        } else {
            quote! {
                write!(f, "</{}>", Self::NODE_NAME)
            }
        }
    }

    fn generate_display_impl_attributes(&self) -> TokenStream2 {
        let fields = self.fields();
        let attributes = fields.map(|field| field.generate_attribute_display_impl());
        if self.self_closing {
            quote!(#(#attributes)*)
        } else {
            quote! {
                #(#attributes)*
                f.write_str(">")?;
            }
        }
    }

    fn generate_display_impl(&self) -> TokenStream2 {
        let ident = &self.ident;
        let attributes = self.generate_display_impl_attributes();
        let closing_tag = self.generate_display_impl_closing_tag();
        quote! {
            impl std::fmt::Display for #ident {
                #[inline]
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "<{}", Self::NODE_NAME)?;
                    #attributes
                    #closing_tag
                }
            }
        }
    }

    pub fn generate(self) -> TokenStream {
        let impls = vec![
            self.generate_node_impl(),
            self.generate_element_impl(),
            self.generate_display_impl(),
        ];
        TokenStream::from(quote! {
           #(
                #[automatically_derived]
                #[allow(unused_qualifications)]
                #impls
           )*
        })
    }
}
