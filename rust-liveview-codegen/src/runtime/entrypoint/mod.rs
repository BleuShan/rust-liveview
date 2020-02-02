use crate::helpers::new;
use proc_macro::TokenStream;
use proc_macro2::{
    Span,
    TokenStream as TokenStream2,
};
use proc_macro_error::*;
use quote::{
    quote,
    ToTokens,
};
use syn::{
    parse::{
        Parse,
        ParseStream,
        Result,
    },
    punctuated::{
        Pair,
        Punctuated,
    },
    spanned::Spanned,
    Error,
    FnArg,
    ItemFn,
    Lit,
    Meta,
    NestedMeta,
    Token,
};

mod main;
mod test;

pub(crate) use main::{
    MainEntryPoint,
    MainEntryPointArgs,
};
pub(crate) use test::{
    TestEntryPoint,
    TestEntryPointArgs,
};

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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum Executor {
    AsyncStd,
    Tokio,
    TokioThreaded {
        core_thread: Option<usize>,
        max_thread: Option<usize>,
    },
}

impl Executor {
    fn from_iter<'a>(args: Box<dyn Iterator<Item = &NestedMeta> + 'a>) -> Option<Self> {
        let mut core_thread = None;
        let mut max_thread = None;
        let mut rt = None;
        for arg in args {
            if let NestedMeta::Meta(meta) = arg {
                match meta {
                    Meta::Path(path) => {
                        if rt.is_some() {
                            abort!(arg.span(), "Duplicate executor argument.");
                        }

                        if path.is_ident("AsyncStd") {
                            rt = Some(Self::AsyncStd)
                        }

                        if path.is_ident("Tokio") {
                            rt = Some(Self::Tokio)
                        }

                        if path.is_ident("TokioThreaded") {
                            rt = Some(Self::TokioThreaded {
                                core_thread: None,
                                max_thread: None,
                            })
                        }
                        if path.is_ident("async_std") {
                            rt = Some(Self::AsyncStd)
                        }
                        if path.is_ident("tokio") {
                            rt = Some(Self::Tokio)
                        }

                        if path.is_ident("tokio_threaded") {
                            rt = Some(Self::TokioThreaded {
                                core_thread,
                                max_thread,
                            })
                        }
                    }
                    Meta::NameValue(name_value) if name_value.path.is_ident("executor") => {
                        if rt.is_some() {
                            abort!(arg.span(), "Duplicate executor argument.");
                        }
                        match &name_value.lit {
                            Lit::Str(v) if v.value() == "async_std" || v.value() == "AsyncStd" => {
                                rt = Some(Self::AsyncStd)
                            }
                            Lit::Str(v) if v.value() == "tokio" || v.value() == "Tokio" => {
                                rt = Some(Self::Tokio)
                            }
                            Lit::Str(v)
                                if v.value() == "tokio_threaded"
                                    || v.value() == "TokioThreaded" =>
                            {
                                rt = Some(Self::TokioThreaded {
                                    core_thread,
                                    max_thread,
                                })
                            }
                            _ => abort!(arg.span(), "Unknown literal."),
                        }
                    }
                    Meta::NameValue(name_value) if name_value.path.is_ident("core_thread") => {
                        if let Lit::Int(expr) = &name_value.lit {
                            core_thread = Some(expr.base10_parse().expect("an"))
                        } else {
                            abort!(arg.span(), "Unknown literal.")
                        }
                    }

                    Meta::NameValue(name_value) if name_value.path.is_ident("max_thread") => {
                        if let Lit::Int(expr) = &name_value.lit {
                            max_thread = Some(expr.base10_parse().expect("an"))
                        } else {
                            abort!(arg.span(), "Unknown literal.")
                        }
                    }
                    _ => (),
                };
            }
        }

        rt
    }
}

impl ToTokens for Executor {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(match self {
            Executor::AsyncStd => {
                quote! {
                    runtime::private::async_std::task::block_on
                }
            }
            Executor::Tokio => {
                quote! {
                    runtime::private::tokio::runtime::Builder::new()
                        .basic_scheduler()
                        .enable_all()
                        .build()
                        .expect("Failed to create runtime.")
                        .block_on
                }
            }
            Executor::TokioThreaded {
                core_thread,
                max_thread,
            } => {
                let mut rt = quote! {
                    runtime::private::tokio::runtime::Builder::new()
                        .threaded_scheduler()
                        .enable_all()
                };

                if let Some(v) = core_thread {
                    rt = quote! {
                        #rt
                        .core_threads(#v)
                    };
                }
                if let Some(v) = max_thread {
                    rt = quote! {
                        #rt
                        .max_threads(#v)
                    };
                }

                quote! {
                    #rt
                    .build()
                    .expect("Failed to create runtime.")
                    .block_on
                }
            }
        });
    }
}
