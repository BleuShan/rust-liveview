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
    LitStr,
    Meta,
    MetaNameValue,
    NestedMeta,
    Path,
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
        core_threads: Option<usize>,
        max_threads: Option<usize>,
    },
}

impl Executor {
    fn parse_path(
        path: &Path,
        core_threads: Option<usize>,
        max_threads: Option<usize>,
    ) -> Option<Self> {
        if path.is_ident("AsyncStd") || path.is_ident("async_std") {
            Some(Executor::AsyncStd)
        } else if path.is_ident("tokio") || path.is_ident("Tokio") {
            Some(Executor::Tokio)
        } else if path.is_ident("tokio_threaded") || path.is_ident("TokioThreaded") {
            Some(Executor::TokioThreaded {
                core_threads,
                max_threads,
            })
        } else {
            None
        }
    }

    fn parse_lit_str(
        expr: &LitStr,
        core_threads: Option<usize>,
        max_threads: Option<usize>,
    ) -> Option<Self> {
        if expr.value() == "async_std" || expr.value() == "AsyncStd" {
            Some(Executor::AsyncStd)
        } else if expr.value() == "tokio" || expr.value() == "Tokio" {
            Some(Executor::Tokio)
        } else if expr.value() == "tokio_threaded" || expr.value() == "TokioThreaded" {
            Some(Executor::TokioThreaded {
                core_threads,
                max_threads,
            })
        } else {
            None
        }
    }

    fn is_valid_path(path: &syn::Path) -> bool {
        path.is_ident("tokio_threadsed")
            || path.is_ident("TokioThreaded")
            || path.is_ident("tokio")
            || path.is_ident("Tokio")
            || path.is_ident("AsyncStd")
            || path.is_ident("async_std")
    }

    fn from_iter<'a>(args: Box<dyn Iterator<Item = &NestedMeta> + 'a>) -> Option<Self> {
        let mut core_threads = None;
        let mut max_threads = None;
        let mut rt = None;
        for arg in args {
            if let NestedMeta::Meta(meta) = arg {
                match meta {
                    Meta::Path(path) => {
                        if rt.is_none() {
                            rt = Self::parse_path(path, core_threads, max_threads)
                        } else if Self::is_valid_path(path) {
                            abort!(arg.span(), "Duplicate executor argument.");
                        }
                    }
                    Meta::NameValue(MetaNameValue { path, lit, .. })
                        if path.is_ident("executor") =>
                    {
                        if rt.is_some() {
                            abort!(arg.span(), "Duplicate executor argument.");
                        }
                        match &lit {
                            Lit::Str(expr) => {
                                rt = Self::parse_lit_str(expr, core_threads, max_threads)
                            }
                            lit => abort!(
                                lit.span(), "Unknown literal.";
                                help = "executor can have async_std or tokio, tokio_threadsed"
                            ),
                        }
                    }
                    Meta::NameValue(MetaNameValue { path, lit, .. })
                        if path.is_ident("core_threads") || path.is_ident("max_threads") =>
                    {
                        if core_threads.is_some() && path.is_ident("core_threads") {
                            abort!(arg.span(), "Duplicate core_threads argument.");
                        }

                        if max_threads.is_some() && path.is_ident("max_threads") {
                            abort!(arg.span(), "Duplicate max_threads argument.");
                        }

                        match rt {
                            Some(Executor::AsyncStd) => {
                                emit_warning!(arg, "argument is unsupported and will be ignored.")
                            }
                            Some(Executor::Tokio) => {
                                emit_warning!(arg, "argument is unsupported and will be ignored.")
                            }
                            _ => (),
                        };

                        match &lit {
                            Lit::Int(expr) => match expr.base10_parse() {
                                Ok(value) => {
                                    if path.is_ident("core_threads") {
                                        if let Some(max) = max_threads {
                                            if value > max {
                                                abort!(
                                                    expr,
                                                    "core_threads number cannot be above max limit"
                                                )
                                            }
                                        }
                                        core_threads = Some(value)
                                    } else {
                                        if let Some(count) = core_threads {
                                            if value < count {
                                                abort!(
                                                    expr,
                                                    "max_threads number cannot be below \
                                                     core_threads number"
                                                )
                                            }
                                        }
                                        max_threads = Some(value)
                                    }
                                }
                                Err(e) => abort!(e),
                            },
                            lit => abort!(lit.span(), "core_threads must be an int."),
                        }
                    }
                    _ => (),
                };
            }
        }

        match rt {
            Some(Self::TokioThreaded {
                core_threads: None,
                max_threads: None,
            }) if core_threads.is_some() && max_threads.is_some() => Some(Self::TokioThreaded {
                core_threads,
                max_threads,
            }),
            Some(Self::TokioThreaded {
                core_threads: None,
                max_threads,
            }) if core_threads.is_some() => Some(Self::TokioThreaded {
                core_threads,
                max_threads,
            }),
            Some(Self::TokioThreaded {
                core_threads,
                max_threads: None,
            }) if max_threads.is_some() => Some(Self::TokioThreaded {
                core_threads,
                max_threads,
            }),
            value => value,
        }
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
                        .thread_name("rust-liveview-runtime-worker")
                        .build()
                        .expect("Failed to create runtime.")
                        .block_on
                }
            }
            Executor::TokioThreaded {
                core_threads,
                max_threads,
            } => {
                let mut rt = quote! {
                    runtime::private::tokio::runtime::Builder::new()
                        .threaded_scheduler()
                        .enable_all()
                        .thread_name("rust-liveview-runtime-worker")
                };

                if let Some(v) = core_threads {
                    rt = quote! {
                        #rt
                        .core_threads(#v)
                    };
                }
                if let Some(v) = max_threads {
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
