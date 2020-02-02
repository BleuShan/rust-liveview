use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum TestKind {
    Test,
    Theory,
    Fact,
}

impl TestKind {
    fn parse_path(path: &Path) -> Option<Self> {
        if path.is_ident("test") {
            Some(Self::Test)
        } else if path.is_ident("fact") {
            Some(Self::Fact)
        } else if path.is_ident("theory") {
            Some(Self::Theory)
        } else {
            None
        }
    }

    fn parse_lit_str(expr: &LitStr) -> Option<Self> {
        if expr.value() == "test" {
            Some(Self::Test)
        } else if expr.value() == "fact" {
            Some(Self::Fact)
        } else if expr.value() == "theory" {
            Some(Self::Theory)
        } else {
            None
        }
    }

    fn is_valid_path(path: &Path) -> bool {
        path.is_ident("theory") || path.is_ident("fact") || path.is_ident("test")
    }

    fn from_iter<'a>(args: Box<dyn Iterator<Item = &NestedMeta> + 'a>) -> Option<Self> {
        let mut result = None;
        for arg in args {
            if let NestedMeta::Meta(meta) = arg {
                match meta {
                    Meta::Path(path) => {
                        if result.is_none() {
                            result = Self::parse_path(path)
                        } else if Self::is_valid_path(path) {
                            abort!(arg.span(), "Duplicate kind argument");
                        }
                    }
                    Meta::NameValue(name_value) if name_value.path.is_ident("kind") => {
                        match &name_value.lit {
                            Lit::Str(expr) => {
                                if result.is_some() {
                                    abort!(arg.span(), "Duplicate kind argument");
                                }

                                result = Self::parse_lit_str(expr)
                            }
                            lit => abort!(
                                lit.span(), "Unknown Literal";
                                help = "kind can have be test, fact or theory"
                            ),
                        }
                    }
                    _ => (),
                };
            }
        }

        result
    }
}

impl Default for TestKind {
    fn default() -> Self {
        Self::Test
    }
}

#[derive(Debug)]
pub(crate) struct TestEntryPointArgs {
    executor: Executor,
    kind: TestKind,
}

impl Parse for TestEntryPointArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs_args: Punctuated<NestedMeta, Token![,]> =
            input.parse_terminated(NestedMeta::parse)?;
        let maybe_executor: Option<Executor> = Executor::from_iter(box attrs_args.iter());
        let maybe_kind: Option<TestKind> = TestKind::from_iter(box attrs_args.iter());

        if let Some(executor) = maybe_executor {
            Ok(Self {
                executor,
                kind: maybe_kind.unwrap_or_default(),
            })
        } else {
            Err(Error::new(
                Span::call_site(),
                "Missing executor configuration.",
            ))
        }
    }
}

#[derive(Debug, new)]
pub(crate) struct TestEntryPoint {
    args: TestEntryPointArgs,
    item: ItemFn,
}

impl ToTokens for TestEntryPoint {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let TestEntryPoint { item, args } = self;
        let mut sig = self.item.sig.clone();
        let attrs = &item.attrs;
        let kind = args.kind;

        if sig.asyncness.is_none() {
            abort!(
                sig.fn_token.span(), "Only async functions are supported";
                help = "Consider writing the signature as:\n\nasync {}", sig.to_token_stream();
            );
        }
        let mut case_attrs_count = 0;
        for attr in attrs {
            if attr.path.is_ident("test") {
                abort!(attr.span(), "Second test attribute is supplied."; 
                note = "test attribute is supplied by default");
            }

            if attr.path.is_ident("fact") {
                abort!(
                    attr.span(), "Second fact attribute is supplied.";
                    help = Span::call_site() => "Consider adding the argument kind = fact to:"
                );
            }

            if attr.path.is_ident("theory") {
                abort!(
                    attr.span(), "Second theory attribute is supplied.";
                    help =
                    Span::call_site() => "Consider adding the argument kind = theory to:"
                );
            }

            if attr.path.is_ident("case") {
                case_attrs_count += 1;
            }
        }

        if kind == TestKind::Theory {
            let types = sig.inputs.pairs().map(|input| match input {
                Pair::Punctuated(FnArg::Typed(expr), p) => Pair::new(expr.ty.clone(), Some(p)),
                Pair::End(FnArg::Typed(expr)) => Pair::new(expr.ty.clone(), None),
                _ => abort!(sig.span(), "Theory test functions cannot a self argument."),
            });

            let ts = quote!(#(#types)*);
            if sig.inputs.is_empty() {
                abort!(sig.span(), "Theory test functions must have arguments.")
            }
            if case_attrs_count == 0 {
                abort_call_site!(
                    "No case attribute supplied to generate the tests.";
                    help = sig.span() => "Add #[case({})] above:", ts
                );
            }
        }

        let vis = &item.vis;
        let body = &item.block;
        sig.asyncness = None;
        tokens.extend(match kind {
            TestKind::Theory => {
                quote! {
                    #[theory]
                    #(#attrs)*
                }
            }
            TestKind::Fact => {
                quote! {
                    #[fact]
                    #(#attrs)*
                }
            }
            TestKind::Test => {
                quote! {
                    #[test]
                    #(#attrs)*
                }
            }
        });

        let block_on = args.executor;

        tokens.extend(quote! {
            #vis #sig {
                #block_on(async move {
                    #body
                })
            }
        })
    }
}

impl From<TestEntryPoint> for TokenStream {
    fn from(entry: TestEntryPoint) -> Self {
        Self::from(quote! {
            #entry
        })
    }
}
