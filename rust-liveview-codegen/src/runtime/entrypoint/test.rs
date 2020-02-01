use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromMeta, Debug)]
pub(crate) enum TestKind {
    Test,
    Theory,
    Fact,
}

impl Default for TestKind {
    fn default() -> Self {
        Self::Test
    }
}

#[derive(Debug, FromMeta)]
pub(crate) struct TestEntryPointArgs {
    executor: Executor,
    #[darling(default)]
    kind: TestKind,
}

impl Parse for TestEntryPointArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs_args: Punctuated<NestedMeta, Token![,]> =
            input.parse_terminated(NestedMeta::parse)?;
        let attrs_args: AttributeArgs = attrs_args
            .pairs()
            .map(|item| {
                match item {
                    Pair::Punctuated(meta, _) => meta,
                    Pair::End(meta) => meta,
                }
                .clone()
            })
            .collect();
        Ok(Self::from_list(&attrs_args)
            .expect_or_abort("An error occured while parsing the input."))
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
                    help = Span::call_site() => "Consider adding the argument kind = \"fact\" to:"
                );
            }

            if attr.path.is_ident("theory") {
                abort!(
                    attr.span(), "Second theory attribute is supplied.";
                    help =
                    Span::call_site() => "Consider adding the argument kind = \"theory\" to:"
                );
            }

            if attr.path.is_ident("case") {
                case_attrs_count += 1;
            }
        }

        if kind == TestKind::Theory {
            let types = sig.inputs.pairs().map(|input| match input {
                Pair::Punctuated(FnArg::Typed(v), p) => Pair::new(v.ty.clone(), Some(p)),
                Pair::End(FnArg::Typed(v)) => Pair::new(v.ty.clone(), None),
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

        let block_on = match args.executor {
            Executor::AsyncStd => {
                quote! {
                    async_std::task::block_on
                }
            }
            Executor::Tokio => {
                quote! {
                    tokio::runtime::Builder::new()
                        .basic_scheduler()
                        .enable_all()
                        .build()
                        .unwrap()
                        .block_on
                }
            }
        };

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
